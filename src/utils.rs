use std::fs::File;
use std::io::{BufReader, Read};
use bitstream_io::{BigEndian, BitRead, BitReader, ByteReader};

pub fn create_file(fout : String) -> File  {
    match File::create(fout){
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
    }
}

pub fn open_file(fin: String) -> File {
    match File::open(fin) {
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
    }
}

pub fn string_to_binary(s: String) -> Vec<u8> {
    let mut out:Vec<u8> = vec![0; s.len() +1];

    out[0] = s.len() as u8;

    for (i, c) in s.chars().enumerate() {
        out[i + 1] = c as u8;
    }

    out
}

pub fn binary_word_to_string(bit_reader: &mut BitReader<File, BigEndian>) -> String {
    let size = bit_reader.read::<u8>(8).unwrap();
    let mut s = String::new();

    for _ in 0..size {
        s += (bit_reader.read::<u8>(8).unwrap() as char).to_string().as_mut_str();
    }

    s
}

pub fn nb_mots(filePath: String) -> i32 {
    let file = open_file(filePath);

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");


    let mut out = 0;
    for car in contents.chars() {
        match car {
            ' '|','|';'|'!'|'?'|'.'|'\n'|'\r' => {
                out += 2;
            },
            _ => ()
        }
    }
    match contents.chars().last().unwrap() {
        ' '|'.'|'\n'|'\r' => (),
        _ => out += 1
    }
    out
}