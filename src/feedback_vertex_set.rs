use petgraph::prelude::UnGraph;
use rand::prelude::*;

fn apply_reductions(graph: &mut UnGraph<(), ()>, k: i32) -> i32 {
    let mut k = k;

    // remove all loops
    graph.retain_nodes(|g, i| {
        if !g.neighbors(i).any(|n| n == i) {
            true
        } else {
            k -= 1;
            false
        }
    });

    // reduce multiplicity of edges > 2 to 2
    while let Some((from, to, c)) = graph.node_indices().find_map(|node| {
        graph.neighbors(node).find_map(|node2| {
            let c = graph
                .neighbors(node)
                .filter(|&node3| node2 == node3)
                .count();
            if c > 2 {
                Some((node, node2, c - 2))
            } else {
                None
            }
        })
    }) {
        for _ in 1..=c {
            if let Some(e) = graph.find_edge(from, to) {
                graph.remove_edge(e);
            }
        }
    }

    // remove nodes of degree at most 1
    while let Some(node) = graph
        .node_indices()
        .find(|&i| graph.neighbors(i).count() <= 1)
    {
        graph.remove_node(node);
    }

    // remove nodes of degree 2 and connect its neighbors by a new edge
    while let Some((i, n1, n2)) = graph.node_indices().find_map(|i| {
        let mut neighbors = graph.neighbors(i);
        let n1 = neighbors.next()?;
        let n2 = neighbors.next()?;
        if n1 == i || n2 == i {
            return None;
        }

        if neighbors.next().is_none() {
            Some((i, n1, n2))
        } else {
            None
        }
    }) {
        graph.add_edge(n1, n2, ());
        graph.remove_node(i);
    }

    k
}

fn reduce_graph(graph: &mut UnGraph<(), ()>, k: i32) -> i32 {
    let mut k = k;

    // apply reductions exhaustively
    loop {
        let num_nodes = graph.node_count();
        let num_edges = graph.edge_count();

        k = apply_reductions(graph, k);

        if k < 0 {
            return k;
        }

        if graph.node_count() == num_nodes && graph.edge_count() == num_edges {
            return k;
        }
    }
}

fn remove_random_node(graph: &mut UnGraph<(), ()>) {
    let mut rng = rand::thread_rng();
    let edge = graph.raw_edges().choose(&mut rng);
    if let Some(edge) = edge {
        if rng.gen() {
            graph.remove_node(edge.source());
        } else {
            graph.remove_node(edge.target());
        }
    }
}

pub fn find_feedback_vertex_set(
    graph: &mut UnGraph<(), ()>,
    k: u32,
    max_repeats: u32,
) -> bool {
    for _ in 1..=max_repeats {
        let mut graph2 = graph.clone();
        let mut k2 = k as i32;
        loop {
            k2 = reduce_graph(&mut graph2, k2);
            if k2 < 0 {
                break;
            }

            if graph2.node_count() == 0 {
                return true;
            }

            if k2 == 0 {
                break;
            }

            remove_random_node(&mut graph2);
            k2 -= 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_loops() {
        let mut g = UnGraph::from_edges([
            (1, 1),
            (2, 2),
            (3, 4),
            (3, 5),
            (3, 6),
            (4, 5),
            (4, 6),
            (5, 6),
        ]);

        let k = reduce_graph(&mut g, 2);
        assert_eq!(k, 0);
        assert_eq!(g.node_count(), 4);
        assert_eq!(g.edge_count(), 6);
    }

    #[test]
    fn reduce_multiplicity() {
        let mut g = UnGraph::from_edges([
            (3, 4),
            (3, 5),
            (3, 6),
            (3, 6),
            (3, 6),
            (4, 5),
            (4, 5),
            (4, 5),
            (4, 6),
            (5, 6),
        ]);

        let k = reduce_graph(&mut g, 2);
        assert_eq!(k, 2);
        assert_eq!(g.node_count(), 4);
        assert_eq!(g.edge_count(), 8);
    }

    #[test]
    fn reduce_degree_one_nodes() {
        let mut g = UnGraph::from_edges([
            (3, 4),
            (3, 5),
            (3, 6),
            (4, 5),
            (4, 6),
            (5, 6),
            (1, 2),
            (2, 3),
        ]);

        let k = reduce_graph(&mut g, 2);
        assert_eq!(k, 2);
        assert_eq!(g.node_count(), 4);
        assert_eq!(g.edge_count(), 6);
    }

    #[test]
    fn reduce_degree_two_nodes() {
        let mut g = UnGraph::from_edges([
            (1, 3),
            (1, 2),
            (2, 4),
            (3, 4),
            (3, 5),
            (3, 6),
            (4, 5),
            (4, 6),
            (5, 6),
        ]);

        let k = reduce_graph(&mut g, 2);
        assert_eq!(k, 2);
        assert_eq!(g.node_count(), 4);
        assert_eq!(g.edge_count(), 7);
    }

    #[test]
    fn reduce_graph_to_empty() {
        let mut g = UnGraph::from_edges([
            (1, 2),
            (1, 3),
            (2, 3),
            (2, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (6, 8),
            (4, 9),
            (7, 9),
            (8, 9),
        ]);

        let k = reduce_graph(&mut g, 2);
        assert_eq!(k, 0);
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }
}
