use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use bitstream_io::{BigEndian, BitRead, BitReader, BitWriter, ByteWrite, ByteWriter};
use crate::huffman;
use crate::huffman::list_to_canonical;
use crate::utils::{binary_word_to_string, create_file, open_file};

pub fn decode_file(fin:String, fout : String) {
    let input_file = open_file(fin);
    let output_file = create_file(fout);
    let mut writer: ByteWriter<File, BigEndian> = ByteWriter::new(output_file);
    let mut reader: BitReader<File, BigEndian> = BitReader::new(input_file );



    //extract header
    let canonical_map=  create_cannonical_map(&mut reader);
    println!("Cannonical map : {:?}", canonical_map);

    let mut key;
    'outer: for i in 0..reader.read::<u8>(8).unwrap() {
        key = 0;
        loop {
            let bit = reader.read_bit().unwrap();

            key += bit as u8;
            println!("Key: {:?}", key);

            match canonical_map.get(&key) {
                Some(s) => {
                    let mut word = s.clone();
                    println!("Key: {:?} prints in output: {:?}",key,  word);
                    writer.write_bytes( word.as_bytes() ).expect("TODO: panic message");
                    continue 'outer;
                },
                None => ()
            }

            key = key << 1;

        }
    }
}

fn create_cannonical_map(reader: &mut BitReader<File, BigEndian>) -> HashMap<u8, String> {
    let taille_max = reader.read::<u8>(8).unwrap();
    println!("Taille max des mots binaires{:?}", taille_max);
    let mut nombre_de_mots_dans_chaque_taille = vec![0u8; taille_max as usize];

    for i in 0..taille_max {
        nombre_de_mots_dans_chaque_taille[i as usize] = reader.read::<u8>(8).unwrap();
    }
    println!("Nombre de mots de chaque tailles: {:?}", nombre_de_mots_dans_chaque_taille);
    let reading_words = reader.read_bit().unwrap();
    let nb_entite_a_lire: usize = nombre_de_mots_dans_chaque_taille.iter().sum::<u8>() as usize;
    let mut list_for_cannonical: Vec<(String, u8)> = vec![("".to_string(), 0u8); nb_entite_a_lire];

    if !reading_words { // 0 c'est pour un fichier rempli de caractères uniquement et 1 qui contient des mots

        println!("Nombre de chars à lire : {:?}", nb_entite_a_lire);

        let mut current_taille = 0;
        let mut nb_char_restant_dans_taille = 0;

        for i in 0..nb_entite_a_lire {
            while nb_char_restant_dans_taille == 0 {
                nb_char_restant_dans_taille = nombre_de_mots_dans_chaque_taille[current_taille];
                current_taille += 1;
            }

            list_for_cannonical[i] = ((reader.read::<u8>(8).unwrap() as char).to_string(), current_taille as u8);
            nb_char_restant_dans_taille -= 1;
        }
        println!("Liste pour créer l'arbre canonique{:?}", list_for_cannonical);
    } else {
        println!("Nombre de mots à lire : {:?}", nb_entite_a_lire);

        let mut current_taille = 0;
        let mut nb_char_restant_dans_taille = 0;


        for i in 0..nb_entite_a_lire {
            while nb_char_restant_dans_taille == 0 {
                nb_char_restant_dans_taille = nombre_de_mots_dans_chaque_taille[current_taille];
                current_taille += 1;
            }

            list_for_cannonical[i] = (binary_word_to_string(reader), current_taille as u8);
            nb_char_restant_dans_taille -= 1;
        }
        println!("Liste pour créer l'arbre canonique{:?}", list_for_cannonical);
    }
    huffman::list_to_hashmap_decoding(list_to_canonical(list_for_cannonical))
}