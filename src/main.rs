mod entropie;

fn main() {
    let files = vec!["texte1Lettres.txt","texte2Lettres.txt","texte3Lettres.txt","texte1Mots.txt","texte2Mots.txt"];

    for f in files {
        println!("{}", entropie::calcul_entropie_lettres((*f).to_owned()));
    }
}




