use crate::part1::{Guide, NodeArena, NodeId};

pub fn run(input: &str) -> usize {
    let nodes = NodeArena::parse(input);
    let mut guide = Guide::parse(input);

    nodes
        .inner()
        .keys()
        .filter(|node_id| node_id.is_start())
        .map(|node_id| walk_until_the_end(&nodes, &mut guide, *node_id))
        .fold(1, lcm)
}

fn lcm(x: usize, y: usize) -> usize {
    let mut buf1 = usize::max(x, y);
    let mut buf2 = usize::min(x, y);

    if buf2 == 0 {
        return buf1;
    }

    let mut remainder = buf1 % buf2;
    while remainder != 0 {
        buf1 = buf2;
        buf2 = remainder;
        remainder = buf1 % buf2;
    }

    x * y / buf2
}

fn walk_until_the_end(nodes: &NodeArena, guide: &mut Guide, start_node_id: NodeId) -> usize {
    let mut current_node_id = start_node_id;
    let mut num_walks = 0;

    while !current_node_id.is_end() {
        let node = nodes.get(current_node_id);
        current_node_id = node.walk_further(guide.where_to());
        num_walks += 1;
    }
    num_walks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(1, 0), 1);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(2, 4), 4);
        assert_eq!(lcm(6, 9), 18);
    }

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
