use std::collections::HashMap;

use crate::tree::Node;

fn huffman(map: HashMap<String, i32>) -> Node {
    let v: Vec<Node> = faire_liste_triee(map);

    for _ in 1..v.len() {
        v = iterer_liste(v);

    }


}

fn faire_liste_triee(map: HashMap<String, i32>) -> Vec<Node> {
    let mut sortie: Vec<Node> = Vec::new();
    for(key, value) in map {
        sortie.push(Node::Leaf(key,value));
    }
    sortie.sort();
    sortie
}