use std::io::{stdin, Read};

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn metadata_sum(&self) -> u32 {
        self.metadata
            .iter()
            .copied()
            .chain(self.children.iter().map(Node::metadata_sum))
            .sum()
    }

    fn value(&self) -> u32 {
        let meta = self.metadata.iter();
        if self.children.is_empty() {
            meta.sum()
        } else {
            meta.flat_map(|index| self.children.get((index - 1) as usize))
                .map(Node::value)
                .sum()
        }
    }

    fn from_iter(iter: &mut impl Iterator<Item = u32>) -> Node {
        let child_count = iter.next().expect("Invalid iteration. Input too small?") as usize;
        let meta_count = iter.next().expect("Invalid iteration. Input too small?") as usize;

        let children = (0..child_count).map(|_| Node::from_iter(iter)).collect();
        let metadata = iter.take(meta_count).collect();

        Node { children, metadata }
    }
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut iter = input.split_whitespace().flat_map(str::parse);
    let root = Node::from_iter(&mut iter);

    println!("Part 1. Sum: {}", root.metadata_sum());
    println!("Part 2. Value: {}", root.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_iter() -> impl Iterator<Item = u32> {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
            .split_whitespace()
            .flat_map(str::parse)
    }

    #[test]
    fn test_part1() {
        let root = Node::from_iter(&mut get_iter());

        assert_eq!(138, root.metadata_sum());
    }

    #[test]
    fn test_part2() {
        let root = Node::from_iter(&mut get_iter());

        assert_eq!(66, root.value());
    }
}
