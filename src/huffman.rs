use std::collections::HashMap;

use crate::tree::Node;

pub fn huffman(map: HashMap<String, i32>) -> Option<Node> {
    let mut v: Vec<Node> = faire_liste_triee(map);

    for _ in 1..v.len() {
        iterer_liste(&mut v);

    }

    v.pop()

}

fn faire_liste_triee(map: HashMap<String, i32>) -> Vec<Node> {
    let mut sortie: Vec<Node> = Vec::new();
    for(key, value) in map {
        sortie.push(Node::Leaf(key,value));
    }
    sortie.sort();
    sortie.reverse();
    sortie
}

fn iterer_liste(v: &mut Vec<Node>) {

    let left = match v.pop() {
        Some(x) => x,
        None => return
    };

    let right = match v.pop() {
        Some(x) => x,
        None => return
    };

    v.push(Node::Internal(Box::new(left),Box::new(right)));

    v.sort();
    v.reverse();
}