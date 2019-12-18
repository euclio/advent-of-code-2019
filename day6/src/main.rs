use std::collections::HashMap;
use std::error::Error;
use std::fs;

use daggy::{Dag, NodeIndex, Walker};

type Graph = Dag<usize, u32>;

fn parse_orbits(input: &str) -> Graph {
    let mut counter = 0;

    let mut orbit_map = HashMap::new();

    let edges = input.lines().map(|line| {
        let mut split = line.split(')');
        let parent = split.next().unwrap();
        let child = split.next().unwrap();

        let parent_id = *orbit_map.entry(parent).or_insert_with(|| {
            counter += 1;
            counter - 1
        });

        let child_id = *orbit_map.entry(child).or_insert_with(|| {
            counter += 1;
            counter - 1
        });

        (parent_id, child_id)
    });

    Dag::from_edges(edges).unwrap()
}

fn orbit_checksum(graph: Graph) -> usize {
    let mut sum = 0;

    for index in (0..graph.node_count()).map(NodeIndex::new) {
        let path_length = graph.recursive_walk(index, |g, i| {
            g.parents(i).iter(g).next()
        }).iter(&graph).count();

        sum += path_length;
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/day6.txt")?;

    println!("part 1: {}", orbit_checksum(parse_orbits(&input)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{parse_orbits, orbit_checksum};

    #[test]
    fn example() {
        let input = indoc!("
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
        ");

        let graph = parse_orbits(input);
        assert_eq!(orbit_checksum(graph), 42);
    }
}
