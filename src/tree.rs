use std::cmp::Ordering;

enum Node {
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