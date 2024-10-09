use std::collections::HashMap;
use either::Either;
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

pub(crate) fn to_canonical(tree: Node) -> Result(HashMap<String, u8>) {
    let map = match tree.to_hash_map() {
        Either::Left(_) => return Err("Invalid tree"),
        Either::Right(map) => map
    };

    let first_list = to_list_for_canonical(map);

    // println!("{:?}",list);

    let canonical_list = list_to_canonical(first_list);

    // println!("{:?}",canonical_list);

    let canonical_map = list_to_hashmap(canonical_list);

    // println!("{:?}",canonical_map);

    Some(canonical_map);
}

fn list_to_hashmap(canonical_list: Vec<(String, u8)>) -> HashMap<String, u8> {
    let mut canonical_map = HashMap::new();

    for (k, v) in canonical_list {
        canonical_map.insert(k, v);
    }
    canonical_map
}

fn list_to_canonical(first_list: Vec<(String, u8)>) -> Vec<(String, u8)> {
    let mut canonical_list = vec![];
    let mut code = 0u8;
    for (i, (v, s)) in first_list.iter().enumerate() {
        canonical_list.push((v.to_owned(), code));

        if i < first_list.len() - 1 {
            let (_, next_size) = first_list[i + 1];
            code = (code + 1) << (next_size - s);
        };
    }
    canonical_list
}

fn to_list_for_canonical(map: HashMap<String, String>) -> Vec<(String, u8)> {
    let mut list: Vec<(String, u8)> = vec![];

    'outer: for (key, value) in map {
        let size = value.chars().count() as u8;

        for (i, (k, s)) in list.iter().enumerate() {
            if size < *s || (size == *s && key < *k) {
                list.insert(i, (key, size));
                continue 'outer;
            }
        }

        list.push((key, size));
        continue 'outer;
    }
    list
}