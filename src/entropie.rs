use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn calcul_entropie_lettres(file_path : String) -> f64 {
    let fichier = match File::open(file_path) {
        Ok(f) => {
            // L'ouverture du fichier s'est bien déroulée, on renvoie l'objet
            f
        }
        Err(e) => {
            // Il y a eu un problème, affichons l'erreur pour voir ce qu'il se passe
            println!("erreur : {:?}", e);
            // On ne peut pas renvoyer le fichier ici, donc on quitte la fonction
            return 0f64;
        }
    };

    let mut occurences:HashMap<char,i32> = HashMap::new();
    let mut buf_reader = BufReader::new(fichier);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");


    // nombre de caractères
    let mut n:i32 = contents.len() as i32;

    for car in contents.chars() {

        // pour considérer le retour à la ligne comme un seul caractère
        if car == '\r' {
            // on le retire du compte de caractères
            n -= 1;
        }
        else {
            let stat = occurences.entry(car).or_insert(0);
            *stat += 1;
        }
    }

    let mut entropie: f64 = 0f64;

    println!("occurences : {:?}", occurences);

    for occ in occurences.values() {
        let pi = *occ as f64 /n as f64 ;
        // println!("{}",pi);
        entropie -= pi * pi.log2()
    }

    entropie
}

pub fn calcul_entropie_mots(file_path : String) -> f64 {
    let fichier = match File::open(file_path) {
        Ok(f) => {
            // L'ouverture du fichier s'est bien déroulée, on renvoie l'objet
            f
        }
        Err(e) => {
            // Il y a eu un problème, affichons l'erreur pour voir ce qu'il se passe
            println!("erreur : {:?}", e);
            // On ne peut pas renvoyer le fichier ici, donc on quitte la fonction
            return 0f64;
        }
    };

    let mut occurences:HashMap<String,i32> = HashMap::new();
    let mut buf_reader = BufReader::new(fichier);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");

    let mut mot = String::from("");

    for car in contents.chars() {
        match car {
            ' ' => push_word(&mut occurences, &mut mot),
            '.'|'\n'|'\r' => {
                push_word(&mut occurences, &mut mot);
                push_word(&mut occurences, &mut String::from(car));
            }
            c => mot.push(c)
        }
    }

    push_word(&mut occurences, &mut mot);

    println!("{:?}", occurences);

    // nombre de caractères
    let n:i32 = occurences.values().sum();

    let mut entropie: f64 = 0f64;

    for occ in occurences.values() {
        let pi = *occ as f64 /n as f64 ;
        // println!("{}",pi);
        entropie -= pi * pi.log2()
    }

    entropie
}

fn push_word(occurences: &mut HashMap<String, i32>, mut mot: &mut String) {
    if !mot.is_empty() {
        let stat = occurences.entry(mot.clone()).or_insert(0);
        *stat += 1;
        mot.clear();
    }
}