use std::cmp::Ordering;
use std::collections::HashMap;
use either::*;


pub enum Node {
    Leaf(String, i32),
    Internal(Box<Node>, Box<Node> )
}

impl Node {
    fn weight(&self) -> i32 {
        match self {
            Node::Leaf(_, weight) => *weight,
            Node::Internal(left, right) => left.weight() + right.weight()
        }
    }

    fn to_string(&self) -> String {
        match self {
            Node::Leaf(string, _) => string.to_string(),
            Node::Internal(left, right) => left.to_string() + &right.to_string()
        }
    }

    pub fn to_binary_map(&self) -> HashMap<String, String> {
        match self.to_hash_map() {
            Either::Left(_) => panic!(),
            Either::Right(map) => map,
        }
    }

    pub fn to_hash_map(&self) -> Either<String, HashMap<String, String>> {
        match self {
            Node::Leaf(string, _) => Either::Left(string.to_owned()),
            Node::Internal(left, right) => {
                let mut map:HashMap<String, String> = HashMap::new();

                match left.to_hash_map() {
                    Either::Left(string) => {
                        map.insert(string, "0".to_owned());
                    },
                    Either::Right(embedded_map) => {
                        for (key, value) in embedded_map {
                            map.insert(key, "0".to_owned() + &value);
                        }
                    }
                };

                match right.to_hash_map() {
                    Either::Left(string) => {
                        map.insert(string, "1".to_owned());
                    },
                    Either::Right(embedded_map) => {
                        for (key, value) in embedded_map {
                            map.insert(key, "1".to_owned() + &value);
                        }
                    }
                }

                Either::Right(map)
            }
        }
    }
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string() && self.weight() == other.weight()
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight().partial_cmp(&other.weight())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}
