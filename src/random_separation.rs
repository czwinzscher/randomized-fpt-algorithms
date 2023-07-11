use petgraph::{
    algo::{is_isomorphic, tarjan_scc},
    graph::NodeIndex,
    prelude::UnGraph,
};
use rand::Rng;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum Color {
    #[default]
    Red,
    Blue,
}

fn color_graph(g: &mut UnGraph<(), Color>) {
    let mut rng = rand::thread_rng();

    for e in g.edge_weights_mut() {
        if rng.gen() {
            *e = Color::Red;
        } else {
            *e = Color::Blue;
        }
    }
}

fn find_isomorphic_component(
    h_component: &UnGraph<(), ()>,
    components_gred: &Vec<Vec<NodeIndex>>,
    g_red: &UnGraph<(), Color>,
) -> Option<usize> {
    for (i, component) in components_gred.iter().enumerate() {
        let mut g_component = g_red.clone();
        g_component.retain_nodes(|_, n| component.contains(&n));

        if is_isomorphic(h_component, &g_component) {
            return Some(i);
        }
    }

    None
}

fn has_isomorphic_subgraph(
    g: &UnGraph<(), Color>,
    h: &UnGraph<(), ()>,
) -> bool {
    let mut g_red = g.clone();
    g_red.retain_edges(|g, e| g[e] == Color::Red);

    let mut connected_components_gred = tarjan_scc(&g_red);
    let connected_components_h = tarjan_scc(h);

    for component_h in connected_components_h {
        let mut h_component = h.clone();
        h_component.retain_nodes(|_, n| component_h.contains(&n));
        if let Some(i) = find_isomorphic_component(
            &h_component,
            &connected_components_gred,
            &g_red,
        ) {
            connected_components_gred.remove(i);
        } else {
            return false;
        }
    }

    true
}

pub fn find_subgraph(
    mut g: UnGraph<(), Color>,
    h: &UnGraph<(), ()>,
    max_repeats: u32,
) -> bool {
    for _ in 1..=max_repeats {
        color_graph(&mut g);

        if has_isomorphic_subgraph(&g, h) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;

    use super::*;

    #[test]
    fn has_subgraph_single_component() {
        let mut g = Graph::new_undirected();
        let n0 = g.add_node(());
        let n1 = g.add_node(());
        let n2 = g.add_node(());
        let n3 = g.add_node(());
        let n4 = g.add_node(());
        let n5 = g.add_node(());
        let n6 = g.add_node(());
        let n7 = g.add_node(());
        let n8 = g.add_node(());
        let n9 = g.add_node(());
        g.add_edge(n0, n1, Color::Red);
        g.add_edge(n1, n2, Color::Red);
        g.add_edge(n2, n3, Color::Blue);
        g.add_edge(n3, n4, Color::Blue);
        g.add_edge(n3, n5, Color::Red);
        g.add_edge(n5, n6, Color::Red);
        g.add_edge(n5, n7, Color::Red);
        g.add_edge(n6, n7, Color::Red);
        g.add_edge(n7, n8, Color::Blue);
        g.add_edge(n7, n9, Color::Blue);

        let h = UnGraph::from_edges([(0, 1), (0, 2), (0, 3), (2, 3)]);

        let res = has_isomorphic_subgraph(&g, &h);
        assert!(res);
    }

    #[test]
    fn no_subgraph_single_component() {
        let mut g = Graph::new_undirected();
        let n0 = g.add_node(());
        let n1 = g.add_node(());
        let n2 = g.add_node(());
        let n3 = g.add_node(());
        let n4 = g.add_node(());
        let n5 = g.add_node(());
        let n6 = g.add_node(());
        let n7 = g.add_node(());
        let n8 = g.add_node(());
        let n9 = g.add_node(());
        g.add_edge(n0, n1, Color::Red);
        g.add_edge(n1, n2, Color::Red);
        g.add_edge(n2, n3, Color::Blue);
        g.add_edge(n3, n4, Color::Blue);
        g.add_edge(n3, n5, Color::Blue);
        g.add_edge(n5, n6, Color::Red);
        g.add_edge(n5, n7, Color::Red);
        g.add_edge(n6, n7, Color::Red);
        g.add_edge(n7, n8, Color::Blue);
        g.add_edge(n7, n9, Color::Blue);

        let h = UnGraph::from_edges([(0, 1), (0, 2), (0, 3), (2, 3)]);

        let res = has_isomorphic_subgraph(&g, &h);
        assert!(!res);
    }

    #[test]
    fn has_subgraph_multiple_components() {
        let mut g = Graph::new_undirected();
        let n0 = g.add_node(());
        let n1 = g.add_node(());
        let n2 = g.add_node(());
        let n3 = g.add_node(());
        let n4 = g.add_node(());
        let n5 = g.add_node(());
        let n6 = g.add_node(());
        let n7 = g.add_node(());
        let n8 = g.add_node(());
        let n9 = g.add_node(());
        g.add_edge(n0, n1, Color::Red);
        g.add_edge(n1, n2, Color::Red);
        g.add_edge(n2, n3, Color::Blue);
        g.add_edge(n3, n4, Color::Blue);
        g.add_edge(n3, n5, Color::Blue);
        g.add_edge(n5, n6, Color::Red);
        g.add_edge(n5, n7, Color::Red);
        g.add_edge(n6, n7, Color::Red);
        g.add_edge(n7, n8, Color::Blue);
        g.add_edge(n7, n9, Color::Blue);

        let h = UnGraph::from_edges([
            (0, 1),
            (0, 2),
            (0, 3),
            (2, 3),
            (4, 5),
            (5, 6),
        ]);

        let res = has_isomorphic_subgraph(&g, &h);
        assert!(!res);
    }

    #[test]
    fn no_subgraph_multiple_components() {
        let mut g = Graph::new_undirected();
        let n0 = g.add_node(());
        let n1 = g.add_node(());
        let n2 = g.add_node(());
        let n3 = g.add_node(());
        let n4 = g.add_node(());
        let n5 = g.add_node(());
        let n6 = g.add_node(());
        let n7 = g.add_node(());
        let n8 = g.add_node(());
        let n9 = g.add_node(());
        g.add_edge(n0, n1, Color::Red);
        g.add_edge(n1, n2, Color::Red);
        g.add_edge(n2, n3, Color::Blue);
        g.add_edge(n3, n4, Color::Blue);
        g.add_edge(n3, n5, Color::Blue);
        g.add_edge(n5, n6, Color::Red);
        g.add_edge(n5, n7, Color::Red);
        g.add_edge(n6, n7, Color::Red);
        g.add_edge(n7, n8, Color::Blue);
        g.add_edge(n7, n9, Color::Blue);

        let h = UnGraph::from_edges([
            (0, 1),
            (0, 2),
            (0, 3),
            (2, 3),
            (4, 5),
            (4, 6),
            (4, 7),
            (6, 7),
        ]);

        let res = has_isomorphic_subgraph(&g, &h);
        assert!(!res);
    }
}
