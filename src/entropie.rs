use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn comptage_lettres(file_path : String) -> HashMap<String, i32> {
    let (mut occurences, contents) = read_file(file_path);
    for car in contents.chars() {
        let stat = occurences.entry(car.to_string()).or_insert(0);
        *stat += 1;
    }
    occurences
}

fn read_file(file_path: String) -> (HashMap<String, i32>, String) {
    let fichier = match File::open(file_path) {
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

    let mut occurences: HashMap<String, i32> = HashMap::new();
    let mut buf_reader = BufReader::new(fichier);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Erreur dans la lecteur du fichier");
    (occurences, contents)
}

pub fn calcul_entropie(occurences: &mut Vec<i32>) -> f64 {
    let n: i32 = occurences.iter().sum();

    let mut entropie: f64 = 0f64;

    println!("occurences : {:?}", occurences);
    for occ in occurences {
        let pi = *occ as f64 / n as f64;
        // println!("{}",pi);
        entropie -= pi * pi.log2()
    }
    entropie
}

pub fn comptage_mots(file_path : String) -> HashMap<String, i32> {
    let (mut occurences, contents) = read_file(file_path);
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
    occurences
}

fn push_word(occurences: &mut HashMap<String, i32>, mot: &mut String) {
    if !mot.is_empty() {
        let stat = occurences.entry(mot.clone()).or_insert(0);
        *stat += 1;
        mot.clear();
    }
}