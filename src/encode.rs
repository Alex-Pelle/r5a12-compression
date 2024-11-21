use std::collections::HashMap;
use std::fs::File;
use bitstream_io::{BigEndian, BitWriter};
use crate::{entropie, huffman};
use std::io::{BufReader, Read, Write};
use bitstream_io::{BitWrite};
use crate::huffman::{max_encoded_length, number_of_symbols, to_ordered_list, to_ordered_list_words};
use crate::utils::{create_file, open_file};

pub fn encode_file(fin: String, out : String) {

    let fichier = open_file(fin.clone());

    let mut buf_reader = BufReader::new(fichier);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");

    let (is_words, map) = generate_correct_map(fin);

    let non_canonical = huffman::huffman(map).unwrap();
    let cannonical:HashMap<String, u8> = huffman::to_canonical(&non_canonical).unwrap();

    println!("{:?}", cannonical);
    println!("{:?}", max_encoded_length(&cannonical));

    let header = generate_header(&cannonical, &contents, true);
    let write = create_file(out);
    let mut writer: BitWriter<File, BigEndian> = BitWriter::new(write);

    writer.write_bytes(&*header).expect("Erreur écriture");

    if is_words {
        let mut word = String::new();
        for c in contents.chars() {
            println!("Word: {:?}", word);
            match c {
                ' '|','|';'|'!'|'?'|'.'|'\n'|'\r' => {
                    if (word != String::new()) {
                        write_binary(cannonical[&word], &mut writer);
                    }
                    write_binary(cannonical[&(c.to_string())], &mut writer);
                    word = String::new();

                }
                c => word += c.to_string().as_str()
            }

        }
        write_binary(cannonical[&word], &mut writer);
    }
    else {
        for c in contents.chars() {
            let x = cannonical[&(c.to_string())];
            println!("{:?}", x);
            write_binary(x, &mut writer);

            if x == 0 {
                print!("{:?}", x % 2);
                writer.write_bit(x % 2 == 1).expect("TODO: panic message");
            }
            println!();
        }
    }

    writer.write(8, 0).expect("TODO: panic message");

    writer.flush().expect("TODO: panic message");

    println!("{:?}", header);
}

fn generate_correct_map(fin: String) -> (bool, HashMap<String, i32>) {
    let mut cpt_mots = vec![];
    for (_, val) in entropie::comptage_mots(fin.clone()) {
        cpt_mots.push(val);
    }

    let mut cpt_lettres = vec![];
    for (_, val) in entropie::comptage_lettres(fin.clone()) {
        cpt_lettres.push(val);
    }

    let is_words = entropie::calcul_entropie(&mut cpt_mots) < entropie::calcul_entropie(&mut cpt_lettres);

    let map;
    if is_words {
        map = entropie::comptage_mots(fin.clone());
    } else {
        map = entropie::comptage_lettres(fin.clone());
    }
    (is_words, map)
}

fn generate_header(cannonical: &HashMap<String, u8>, contents: &String, encoding_words: bool) -> Vec<u8> {
    let max_length = max_encoded_length(&cannonical);
    let mut size_of_header:usize;
    if !encoding_words {
        size_of_header = (1 + max_length + number_of_symbols(&cannonical) + 1) as usize;
    }
    else {
        size_of_header = (1 + max_length + cumulative_size_of_words(&cannonical) + 1) as usize;
    }

    size_of_header = (1 + max_length + cumulative_size_of_words(&cannonical) + 1) as usize;

    let mut header: Vec<u8> = vec![0; size_of_header];

    println!("{:?}", header);

    header[0] = max_length + 128 * encoding_words as u8;
    for (i, n) in huffman::length_list(&cannonical).iter().enumerate() {
        header[i + 1] = *n
    }


    //Cas où on encode des caractères seulement
    if !encoding_words {


        println!("{:?}", to_ordered_list(&cannonical));

        for (i, s) in to_ordered_list(&cannonical).iter().enumerate() {
            header[1 + max_length as usize + i] = *s;
        }

    } else

    //Cas où on encode des mots
    {
        println!("{:?}", to_ordered_list_words(&cannonical));
        let mut offset= 1 + max_length as usize;
        for (_, s) in to_ordered_list_words(&cannonical).iter().enumerate() {
            for (_, c) in s.to_string().chars().enumerate() {
                header[offset] = c as u8;
                offset += 1;
            }
            header[offset] = 0;
            offset += 1;
        }
    }

    println!("{:?}", header);

    header[size_of_header - 1] = contents.chars().count() as u8;

    header
}

fn cumulative_size_of_words(map: &&HashMap<String, u8>) -> u8 {
    map.iter().map(|(s, i)| s.len() + 1).reduce(|a, b| a + b).unwrap() as u8
}

fn write_binary(x: u8, writer: &mut BitWriter<File, BigEndian>) {

    if x <= 0 {
        return
    }
    write_binary(x / 2, writer);
    print!("{:?}", x % 2);
    writer.write_bit(x % 2 == 1).expect("TODO: panic message");
}