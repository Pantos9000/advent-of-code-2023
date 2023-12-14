use crate::part1::{Direction, Guide, NodeArena, NodeId};

pub fn run(input: &str) -> usize {
    let nodes = NodeArena::parse(input);
    let mut guide = Guide::parse(input);
    let mut path = Path::parse(input);

    let mut arrived = false;
    while !arrived {
        let direction = guide.where_to();
        arrived = path.are_we_there_yet(&nodes, direction);
    }
    path.num_steps()
}

struct Path<'a> {
    node_ids: Vec<NodeId<'a>>,
    num_steps: usize,
}
impl<'a> Path<'a> {
    fn parse(input: &'a str) -> Self {
        let node_ids = input
            .lines()
            .filter_map(|line| line.split(" = ").next())
            .map(NodeId::from)
            .filter(|node| node.is_start())
            .collect();
        Self {
            node_ids,
            num_steps: 0,
        }
    }

    /// return true if all nodes are done
    fn are_we_there_yet(&mut self, nodes: &'a NodeArena, direction: Direction) -> bool {
        self.num_steps += 1;
        let mut we_are_there = true;
        for node_id in &mut self.node_ids {
            let new_node = nodes.get(*node_id);
            *node_id = new_node.walk_further(direction);
            if !node_id.is_end() {
                we_are_there = false;
            }
        }
        we_are_there
    }

    fn num_steps(&self) -> usize {
        self.num_steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            LR\n\
            \n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)";
        let result = run(input);
        assert_eq!(result, 6);
    }
}
