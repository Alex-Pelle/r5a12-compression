use std::collections::HashMap;

mod entropie;
mod tree;
mod huffman;

use tree::Node;

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
    println!("{:?}", non_canonical.to_binary_map());
    huffman::to_canonical(non_canonical);
}




