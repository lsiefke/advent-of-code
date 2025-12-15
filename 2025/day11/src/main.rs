use std::hash::RandomState;

use petgraph::{
    algo::{all_simple_paths, dijkstra},
    dot::{Config, Dot},
    graph::NodeIndex,
    prelude::StableDiGraph,
};

struct GraphWithIndices {
    indices: Vec<String>,
    graph: StableDiGraph<u32, ()>,
}

fn main() {
    for path in ["input/example", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let input = parse(&input);
        save_dot(&input, path);

        let solution1 = part1(&input.graph, &input.indices, "you", "out").count();
        println!("{path} - part1: {solution1}");
    }

    for path in ["input/example2", "input/input"] {
        let input = std::fs::read_to_string(path).unwrap();
        let input = parse(&input);
        save_dot(&input, path);

        println!("{path} - part2: {}", part2(&input.graph, &input.indices));
    }
}

fn part1(
    graph: &StableDiGraph<u32, ()>,
    indices: &[String],
    from: &str,
    to: &str,
) -> impl Iterator<Item = Vec<NodeIndex>> {
    let idx_from = indices.iter().position(|key| key == from).unwrap() as u32;
    let idx_to = indices.iter().position(|key| key == to).unwrap() as u32;
    all_simple_paths::<Vec<_>, _, RandomState>(graph, idx_from.into(), idx_to.into(), 0, None)
}

/// In contrast to part1, the big distance between "svr" and "out" causes a huge runtime.
/// To enable fast computation, the graph is filtered to only keep nodes pointing to the according goal node.
fn part2(graph: &StableDiGraph<u32, ()>, indices: &[String]) -> usize {
    let to_fft = trim_after_node(graph, indices, "fft");
    let to_dac = trim_after_node(graph, indices, "dac");
    let to_out = trim_after_node(graph, indices, "out");
    part1(&to_fft, indices, "svr", "fft").count()
        * part1(&to_dac, indices, "fft", "dac").count()
        * part1(&to_out, indices, "dac", "out").count()
}

/// Removes every node, which has no reachable path to "node".
fn trim_after_node(
    graph: &StableDiGraph<u32, ()>,
    indices: &[String],
    node: &str,
) -> StableDiGraph<u32, ()> {
    let idx_node = indices.iter().position(|key| *key == node).unwrap() as u32;
    let mut newgraph = graph.clone();
    let followingnodes = graph
        .node_indices()
        .map(|i| dijkstra(&graph, i, None, |_| 1))
        .filter(|nodes| {
            let key = NodeIndex::new(idx_node as usize);
            nodes.contains_key(&key) && *nodes.get(&key).unwrap() == 0
        });
    for node_dist in followingnodes {
        for (node, dist) in node_dist {
            if dist != 0 {
                // dist == 0: the goal node, which is included in this list as it is reachable with a distance of 0
                newgraph.remove_node(node);
            }
        }
    }
    newgraph
}

fn parse(input: &str) -> GraphWithIndices {
    let mut indices: Vec<String> = input
        .lines()
        .map(|line| line.split(':').next().unwrap().to_owned())
        .collect();

    // There is a single node without outgoing edges
    indices.push("out".to_string());

    // egdes is a tuple of directed edges from first to second element
    let edges = input.lines().flat_map(|line| {
        let mut it = line.split(':');
        let from = it.next().unwrap();
        let idxfrom = indices.iter().position(|key| key == from).unwrap();
        let to = it.next().unwrap().split_whitespace();
        to.map(|edge| {
            let idxto = indices.iter().position(|key| key == edge).unwrap();
            (idxfrom as u32, idxto as u32)
        })
        .collect::<Vec<(u32, u32)>>()
    });

    let graph = StableDiGraph::<u32, ()>::from_edges(edges);
    GraphWithIndices { indices, graph }
}

fn save_dot(input: &GraphWithIndices, name: &str) {
    let dot = Dot::with_config(&input.graph, &[Config::NodeIndexLabel]);
    std::fs::write(name.to_string() + ".dot", format!("{:?}", dot)).unwrap();
}
