mod entropie;

fn main() {
    let files = vec!["texte1Lettres.txt","texte2Lettres.txt","texte3Lettres.txt","texte1Mots.txt","texte2Mots.txt"];
    let files_mot = vec!["texte1Mots.txt","texte2Mots.txt"];


    println!("Entropies par lettres");

    for f in files {
        println!("{}", entropie::calcul_entropie(&mut entropie::comptage_lettres((*f).to_owned()).values().cloned().collect()));
    }


    println!();
    println!("Entropies par mots");

    for f in files_mot {
        println!("{}", entropie::calcul_entropie(&mut entropie::comptage_mots((*f).to_owned()).values().cloned().collect()));
    }



}




