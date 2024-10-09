use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::sync::atomic::AtomicIsize;
use bitstream_io::{BigEndian, BitWrite, LittleEndian};
use bitstream_io::write::BitWriter;

mod entropie;
mod tree;
mod huffman;

use tree::Node;
use crate::huffman::{max_encoded_length, number_of_symbols, to_list_for_canonical, to_ordered_list};

fn main() {
    let files = vec!["texte1Lettres.txt","texte2Lettres.txt","texte3Lettres.txt","texte1Mots.txt","texte2Mots.txt"];
    let files_mot = vec!["texte1Mots.txt","texte2Mots.txt"];

    /* println!("Entropies par lettres");

    for f in files {


        let map = entropie::comptage_lettres((*f).to_owned());

        println!("{:?}", huffman::huffman(map).unwrap().to_binary_map());    }


    println!();
    println!("Entropies par mots");

    for f in files_mot {


        let map = entropie::comptage_mots((*f).to_owned());

        println!("{:?}", huffman::huffman(map).unwrap().to_binary_map());    }

    println!(); */

    let map = entropie::comptage_lettres("customMots.txt".to_owned());

    let non_canonical = huffman::huffman(map).unwrap();
    let mut cannonical:HashMap<String, u8> = huffman::to_canonical(&non_canonical).unwrap();

    println!("{:?}", cannonical);
    println!("{:?}", max_encoded_length(&mut cannonical));
    let size_of_header = 1 + max_encoded_length(&mut cannonical) + number_of_symbols(&mut cannonical);

    let mut header:Vec<u8> = vec![0; size_of_header as usize];

    println!("{:?}", header);

    header[0] = max_encoded_length(&mut cannonical);
    for (i, n) in huffman::length_list(&mut cannonical).iter().enumerate() {
        header[i +1] = *n
    }

    header[0] = max_encoded_length(&mut cannonical);
    for (i, n) in huffman::length_list(&mut cannonical).iter().enumerate() {
        header[i +1] = *n
    }

    println!("{:?}", to_ordered_list(&cannonical));

    for (i, s) in to_ordered_list(&cannonical).iter().enumerate() {
        header[1 + max_encoded_length(&mut cannonical) as usize + i] = *s;
    }


    let fichier = match File::open("customMots.txt") {
        Ok(f) => {
            // L'ouverture du fichier s'est bien déroulée, on renvoie l'objet
            f
        }
        Err(e) => {
            // Il y a eu un problème, affichons l'erreur pour voir ce qu'il se passe
            println!("erreur : {:?}", e);
            // On ne peut pas renvoyer le fichier ici, donc on quitte la fonction
            panic!("erreur ");
        }
    };

    let mut buf_reader = BufReader::new(fichier);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");


    let mut write = match File::create("test.txt") {
        Ok(f) => {
            // L'ouverture du fichier s'est bien déroulée, on renvoie l'objet
            f
        }
        Err(e) => {
            // Il y a eu un problème, affichons l'erreur pour voir ce qu'il se passe
            println!("erreur : {:?}", e);
            // On ne peut pas renvoyer le fichier ici, donc on quitte la fonction
            panic!("erreur ");
        }
    };



    write.write(&*header).expect("Erreur écriture");
    let mut writer:BitWriter<File, LittleEndian> = BitWriter::new(write);

    for c in contents.chars() {
        println!("{:?}", cannonical[&(c.to_string())]);
        write_binary(cannonical[&(c.to_string())], &mut writer);

        if cannonical[&(c.to_string())] == 0 {
            
        }
        println!();

    }

    writer.flush().expect("TODO: panic message");

    println!("{:?}", header);
}

fn write_binary(x: u8, writer: &mut BitWriter<File, LittleEndian>) {

    if x <= 0 {
        return
    }
    write_binary(x / 2, writer);
    print!("{:?}", x % 2);
    writer.write_bit(x % 2 == 1).expect("TODO: panic message");
}





