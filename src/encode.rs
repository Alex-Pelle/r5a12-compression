use std::collections::HashMap;
use std::fs::File;
use bitstream_io::{BigEndian, BitWriter};
use crate::{entropie, huffman};
use std::io::{BufReader, Read, Write};
use bitstream_io::{BitWrite};
use crate::huffman::{max_encoded_length, number_of_symbols, to_ordered_list};
use crate::utils::{create_file, open_file};

pub fn encode_file(fin: String, out : String) {

    let fichier = open_file(fin.clone());

    let mut buf_reader = BufReader::new(fichier);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");

    let map = entropie::comptage_lettres(fin.clone());

    let non_canonical = huffman::huffman(map).unwrap();
    let cannonical:HashMap<String, u8> = huffman::to_canonical(&non_canonical).unwrap();

    println!("{:?}", cannonical);
    println!("{:?}", max_encoded_length(&cannonical));

    let header = generate_header(&cannonical, &contents);
    let write = create_file(out);
    let mut writer: BitWriter<File, BigEndian> = BitWriter::new(write);

    writer.write_bytes(&*header).expect("Erreur écriture");

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

    writer.write(7, 0).expect("TODO: panic message");

    writer.flush().expect("TODO: panic message");

    println!("{:?}", header);
}

fn generate_header(cannonical: &HashMap<String, u8>, contents: &String, encoding_words: bool) -> Vec<u8> {
    let max_length = max_encoded_length(&cannonical);
    if !encoding_words {
        let size_of_header = 1 + max_length + number_of_symbols(&cannonical) + 1;

        let mut header: Vec<u8> = vec![0; size_of_header as usize];

        println!("{:?}", header);

        header[0] = max_length;
        for (i, n) in huffman::length_list(&cannonical).iter().enumerate() {
            header[i + 1] = *n
        }

        println!("{:?}", to_ordered_list(&cannonical));

        for (i, s) in to_ordered_list(&cannonical).iter().enumerate() {
            header[1 + max_length as usize + i] = *s;
        }

        header[size_of_header as usize - 1] = contents.chars().count() as u8;
    }
    header
}

fn write_binary(x: u8, writer: &mut BitWriter<File, BigEndian>) {

    if x <= 0 {
        return
    }
    write_binary(x / 2, writer);
    print!("{:?}", x % 2);
    writer.write_bit(x % 2 == 1).expect("TODO: panic message");
}