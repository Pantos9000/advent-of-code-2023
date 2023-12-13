use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let nodes = NodeArena::parse(input);
    let mut guide = Guide::parse(input);
    let start_id = NodeId::from("AAA");
    let end_id = NodeId::from("ZZZ");

    let mut current_node_id = start_id;
    let mut num_walks = 0;

    while current_node_id != end_id {
        let node = nodes.get(current_node_id);
        current_node_id = node.walk_further(guide.where_to());
        num_walks += 1;
    }
    num_walks
}

struct Guide<'a> {
    data: &'a str,
    iter: std::str::Chars<'a>,
}
impl<'a> Guide<'a> {
    fn parse(input: &'a str) -> Self {
        let data = input.lines().next().unwrap();
        let iter = data.chars();
        Self { data, iter }
    }
    fn get_next_char(&mut self) -> char {
        self.iter.next().unwrap_or_else(|| {
            self.iter = self.data.chars();
            return self.iter.next().unwrap();
        })
    }
    fn where_to(&mut self) -> Direction {
        self.get_next_char().into()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unexpected value {value}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId<'a>(&'a str);
impl<'a> From<&'a str> for NodeId<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

struct NodeArena<'a> {
    nodes: HashMap<NodeId<'a>, Node<'a>>,
}
impl<'a> NodeArena<'a> {
    fn parse(input: &'a str) -> Self {
        let mut nodes = HashMap::new();
        'line_loop: for line in input.lines() {
            let Some((node_id, node_content)) = line.split_once(" = ") else {
                continue 'line_loop;
            };

            let node_id = NodeId::from(node_id);
            let node = Node::parse(node_content);
            let overwritten = nodes.insert(node_id, node);
            assert!(overwritten.is_none());
        }
        Self { nodes }
    }

    fn get(&self, node_id: NodeId<'a>) -> &Node {
        self.nodes.get(&node_id).unwrap()
    }
}

struct Node<'a> {
    left: NodeId<'a>,
    right: NodeId<'a>,
}
impl<'a> Node<'a> {
    fn parse(content: &'a str) -> Self {
        // remove round brackets
        let mut chars = content.chars();
        chars.next();
        chars.next_back();
        let content = chars.as_str();
        let (left, right) = content.split_once(", ").unwrap();
        Self {
            left: left.into(),
            right: right.into(),
        }
    }
    fn walk_further(&self, direction: Direction) -> NodeId<'a> {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}
