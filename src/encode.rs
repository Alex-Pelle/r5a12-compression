use std::collections::HashMap;
use std::fs::File;
use bitstream_io::{BigEndian, BitWriter};
use crate::{entropie, huffman};
use std::io::{BufReader, Read, Write};
use bitstream_io::{BitWrite};
use crate::entropie::comptage_mots;
use crate::huffman::{max_encoded_length, number_of_symbols, to_ordered_list, to_ordered_list_words};
use crate::utils::{create_file, open_file};

pub fn encode_file(fin: String, out : String, is_words: Option<bool>) {

    let fichier = open_file(fin.clone());

    let mut buf_reader = BufReader::new(fichier);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");

    let (is_words, map) = generate_correct_map(fin, is_words);
    println!("{:?}", map);

    let non_canonical = huffman::huffman(map).unwrap();
    let cannonical:HashMap<String, String> = huffman::to_canonical(&non_canonical).unwrap();

    println!("{:?}, words : {:?}", cannonical, is_words);
    println!("{:?}", max_encoded_length(&cannonical));

    let header = generate_header(&cannonical, &contents, is_words);
    let write = create_file(out);
    let mut writer: BitWriter<File, BigEndian> = BitWriter::new(write);

    writer.write_bytes(&*header).expect("Erreur écriture");

    if is_words {
        let mut word = String::new();
        for c in contents.chars() {
            //println!("Word: {:?}", word);
            match c {
                ' '|','|';'|'!'|'?'|'.'|'\n'|'\r' => {
                    if word != String::new() {
                        write_binary(cannonical[&word].as_str(), &mut writer);
                    }
                    write_binary(cannonical[&(c.to_string())].as_str(), &mut writer);
                    word = String::new();

                }
                c => word += c.to_string().as_str()
            }

        }
        if word != String::new() {
            write_binary(cannonical[&word].as_str(), &mut writer);
        }
    }
    else {
        for c in contents.chars() {
            let x = cannonical[&(c.to_string())].as_str();
            //println!("{:?}", x);
            write_binary(x, &mut writer);

            // Sert peut-être à rien
            /*
            if x == 0 {
                print!("{:?}", x % 2);
                writer.write_bit(x % 2 == 1).expect("TODO: panic message");
            }
            */
            //println!();
        }
    }

    writer.write(8, 0).expect("TODO: panic message");

    writer.flush().expect("TODO: panic message");

    println!("{:?}", header);
}

fn generate_correct_map(fin: String, words :Option<bool>) -> (bool, HashMap<String, i32>) {
    let mut cpt_mots = vec![];
    for (_, val) in entropie::comptage_mots(fin.clone()) {
        cpt_mots.push(val);
    }

    let mut cpt_lettres = vec![];
    for (_, val) in entropie::comptage_lettres(fin.clone()) {
        cpt_lettres.push(val);
    }

    let mut is_words = entropie::calcul_entropie(&mut cpt_mots) < entropie::calcul_entropie(&mut cpt_lettres) && cpt_mots.len() > 2;
    if words.is_some() {
        is_words = words.unwrap();
    }
    let map;
    if is_words {
        map = entropie::comptage_mots(fin.clone());
    } else {
        map = entropie::comptage_lettres(fin.clone());
    }
    (is_words, map)
}

fn generate_header(cannonical: &HashMap<String, String>, contents: &String, encoding_words: bool) -> Vec<u8> {
    let max_length = max_encoded_length(&cannonical);
    let mut size_of_header:usize;
    if !encoding_words {
        size_of_header = (1 + max_length + number_of_symbols(&cannonical) + 4) as usize;
    }
    else {
        size_of_header = (1 + max_length as usize + cumulative_size_of_words(&cannonical) + 4);
    }

    let mut header: Vec<u8> = vec![0; size_of_header];

    println!("{:?}", header);
    println!("{:?}", cannonical);
    println!("{:?}", huffman::length_list(&cannonical));

    header[0] = max_length + 128 * encoding_words as u8;
    for (i, n) in huffman::length_list(&cannonical).iter().enumerate() {
        header[i + 1] = *n
    }


    header[size_of_header - 1] = 0;

    //Cas où on encode des caractères seulement
    if !encoding_words {


        println!("{:?}", to_ordered_list(&cannonical));

        for (i, s) in to_ordered_list(&cannonical).iter().enumerate() {
            header[1 + max_length as usize + i] = *s;
        }
        let words = contents.chars().count();

        header[size_of_header - 4] = (words >> 24) as u8;
        header[size_of_header - 3] = (words >> 16) as u8;
        header[size_of_header - 2] = (words >> 8) as u8;
        header[size_of_header - 1] = (words >> 0) as u8;

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
        header[size_of_header - 1] = 0;
        let mut word = false;
        let mut words = 0;
        for car in contents.chars() {
            match car {
                ' '|','|';'|'!'|'?'|'.'|'\n'|'\r' => {
                    words += 1 + word as u32;
                    word = false;
                }
                c => word = true
            }
        }
        words += 1 + word as u32;

        header[size_of_header - 4] = (words >> 24) as u8;
        header[size_of_header - 3] = (words >> 16) as u8;
        header[size_of_header - 2] = (words >> 8) as u8;
        header[size_of_header - 1] = (words >> 0) as u8;

    }

    println!("{:?}", header);


    header
}

fn cumulative_size_of_words(map: &&HashMap<String, String>) -> usize {
    map.iter().map(|(s, _)| s.chars().count() + 1).reduce(|a, b| a + b).unwrap()
}

fn write_binary(x: &str, writer: &mut BitWriter<File, BigEndian>) {
    for c in x.chars() {
        writer.write_bit(c == '1').unwrap();
    }
}