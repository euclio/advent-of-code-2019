use std::collections::HashMap;
use std::error::Error;
use std::fs;

use daggy::{Dag, NodeIndex, Walker};
use daggy::petgraph::{Undirected, algo::dijkstra};

type Graph = Dag<usize, u32>;

fn parse_orbits(input: &str) -> (Graph, HashMap<String, u32>) {
    let mut counter = 0;

    let mut destinations = HashMap::new();

    let edges = input.lines().map(|line| {
        let mut split = line.split(')');
        let parent = split.next().unwrap();
        let child = split.next().unwrap();

        let parent_id = *destinations.entry(parent.to_owned()).or_insert_with(|| {
            counter += 1;
            counter - 1
        });

        let child_id = *destinations.entry(child.to_owned()).or_insert_with(|| {
            counter += 1;
            counter - 1
        });

        (parent_id, child_id)
    });

    let graph = Dag::from_edges(edges).unwrap();

    (graph, destinations)
}

fn orbit_checksum(graph: &Graph) -> usize {
    let mut sum = 0;

    for index in (0..graph.node_count()).map(NodeIndex::new) {
        let path_length = graph.recursive_walk(index, |g, i| {
            g.parents(i).iter(g).next()
        }).iter(&graph).count();

        sum += path_length;
    }

    sum
}

fn orbital_transfers(graph: Graph, you: NodeIndex, santa: NodeIndex) -> u32 {
    // Convert to an undirected graph.
    let graph = graph.into_graph().into_edge_type::<Undirected>();

    let costs = dijkstra(&graph, you, Some(santa), |_| 1);
    costs[&santa] - 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("inputs/day6.txt")?;

    let (graph, destinations) = parse_orbits(&input);
    println!("part 1: {}", orbit_checksum(&graph));

    let you = NodeIndex::new(destinations["YOU"] as usize);
    let santa = NodeIndex::new(destinations["SAN"] as usize);

    println!("part 2: {}", orbital_transfers(graph, you, santa));

    Ok(())
}

#[cfg(test)]
mod tests {
    use daggy::NodeIndex;
    use indoc::indoc;

    use super::{parse_orbits, orbit_checksum, orbital_transfers};

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

        let (graph, _) = parse_orbits(input);
        assert_eq!(orbit_checksum(&graph), 42);
    }

    #[test]
    fn part2_example() {
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
            K)YOU
            I)SAN
        ");

        let (graph, destinations) = parse_orbits(input);

        let you = NodeIndex::new(destinations["YOU"] as usize);
        let santa = NodeIndex::new(destinations["SAN"] as usize);

        assert_eq!(orbital_transfers(graph, you, santa), 4);
    }
}
