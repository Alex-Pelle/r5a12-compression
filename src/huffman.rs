use std::collections::HashMap;
use std::error::Error;
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

pub(crate) fn to_canonical(tree: &Node) -> Result<HashMap<String, String>, String> {
    let map = match tree.to_hash_map() {
        Either::Left(_) => return Err("Invalid tree".to_owned()),
        Either::Right(map) => map
    };

    let first_list = to_list_for_canonical(map);

    println!("list for canonical: {:?}",first_list);

    let canonical_list = list_to_canonical(first_list);

    println!("canonical list: {:?}",canonical_list);

    let canonical_map = list_to_hashmap_encoding(canonical_list);

    println!("canonical map: {:?}",canonical_map);

    Ok(canonical_map)
}

pub fn list_to_hashmap_encoding(canonical_list: Vec<(String, String)>) -> HashMap<String, String> {
    let mut canonical_map = HashMap::new();

    for (k, v) in canonical_list {
        canonical_map.insert(k, v);
    }
    canonical_map
}

pub fn list_to_hashmap_decoding(canonical_list: Vec<(String, String)>) -> HashMap<String, String> {
    let mut canonical_map = HashMap::new();

    for (k, v) in canonical_list {
        canonical_map.insert(v, k);
    }
    canonical_map
}

pub(crate) fn list_to_canonical(first_list: Vec<(String, u8)>) -> Vec<(String, String)> {
    let mut canonical_list = vec![];
    let mut code = 0u16;
    let mut left_zeroes= 0;
    for (i, (v, s)) in first_list.iter().enumerate() {
        println!("code {}, zeros {}, s {}",code, left_zeroes, s);
        left_zeroes =  *s - ((code as f64).log2().trunc() as u8 + 1);
        if left_zeroes > 0 {
            canonical_list.push((v.to_owned(), std::iter::repeat("0").take(left_zeroes as usize).collect::<String>() + &format!("{:b}", code)));
        }
        else {
            canonical_list.push((v.to_owned(), format!("{:b}", code)));
        }

        if i < first_list.len() - 1 {
            let (_, next_size) = first_list[i + 1];
            // TODO this can break
            code = (code + 1) << (next_size - s);
        };
    }
    canonical_list
}

pub fn to_list_for_canonical(map: HashMap<String, String>) -> Vec<(String, u8)> {
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

pub fn max_encoded_length(map: &HashMap<String, String>) -> u8 {
    let mut max = 0;

    for (_, k) in map {
        let s = k.len();
        if s > max {
            max = s
        }
    }

    max as u8
}

pub fn number_of_symbols(map: &HashMap<String, String>) -> u8 {
    println!("map {:?} size {:?}", map, map.len());
    map.len() as u8
}


pub fn length_list(map: &HashMap<String, String>) -> Vec<u8> {
    let mut l: Vec<u8> = vec![0; max_encoded_length(map) as usize];

    for (_, k) in map {
        let s = k.len();
        l[s -1] += 1;
    }

    l
}

pub fn to_ordered_list(map: &HashMap<String, String>) -> Vec<u8> {
    let mut list = vec![];

    'outer: for (key, size) in map {

        for (i, (k, s)) in list.iter().enumerate() {
            if size < *s || (size == *s && key < *k) {
                list.insert(i, (key, size));
                continue 'outer;
            }
        }

        list.push((key, size));
        continue 'outer;
    }

    let mut l = vec![];

    for (s, v) in list {
        l.push(s.chars().last().unwrap() as u8);
    }

    l
}

pub fn to_ordered_list_words(map: &HashMap<String, String>) -> Vec<&String> {
    let mut list = vec![];

    'outer: for (key, size) in map {

        for (i, (k, s)) in list.iter().enumerate() {
            if size < *s || (size == *s && key < *k) {
                list.insert(i, (key, size));
                continue 'outer;
            }
        }

        list.push((key, size));
        continue 'outer;
    }

    let mut l = vec![];

    for (s, v) in list {
        l.push(s);
    }

    l
}