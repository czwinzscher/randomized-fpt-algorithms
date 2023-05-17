use petgraph::{graph::DiGraph, graph::NodeIndex};
use rand::Rng;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn find_colorful_path(g: &DiGraph<u32, ()>, k: u32) -> bool {
    for s in g.node_indices() {
        let has_simple_path = find_colorful_path_starting_at(g, k, s);
        if has_simple_path {
            return true;
        }
    }

    false
}

fn find_colorful_path_starting_at(
    g: &DiGraph<u32, ()>,
    k: u32,
    s: NodeIndex,
) -> bool {
    if k == 1 {
        return true;
    }

    let mut mem = HashMap::new();
    let c = BTreeSet::from([g[s]]);
    let p = HashSet::from([c]);
    mem.insert((s, 1u32), p);

    let mut queue = VecDeque::from([(s, 1)]);

    while let Some((v, i)) = queue.pop_front() {
        let j = i + 1;
        if j > k {
            break;
        }

        for u in g.neighbors(v) {
            let cs = mem[&(v, i)].clone();
            let entry = mem.entry((u, j)).or_insert(HashSet::new());
            let color = g[u];
            for mut c in cs {
                if !c.contains(&color) {
                    c.insert(color);
                    if j == k {
                        return true;
                    }

                    entry.insert(c);
                }
            }

            if !entry.is_empty() && !queue.contains(&(u, j)) {
                queue.push_back((u, j));
            }
        }
    }

    false
}

fn color_graph(g: &mut DiGraph<u32, ()>, k: u32) {
    let mut rng = rand::thread_rng();

    g.node_indices().for_each(|i| g[i] = rng.gen_range(1..=k));
}

pub fn find_simple_path(mut g: DiGraph<u32, ()>, k: u32) -> bool {
    color_graph(&mut g, k);

    find_colorful_path(&g, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        let g = DiGraph::new();
        let res = find_colorful_path(&g, 1);
        assert!(!res);
    }

    #[test]
    fn simple_has_path() {
        let mut g = DiGraph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(1);
        g.extend_with_edges([(n1, n2), (n1, n3)]);
        let res = find_colorful_path(&g, 2);
        assert!(res);
    }

    #[test]
    fn simple_no_path() {
        let mut g = DiGraph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(1);
        let n3 = g.add_node(1);
        g.extend_with_edges([(n1, n2), (n1, n3)]);
        let res = find_colorful_path(&g, 2);
        assert!(!res);
    }

    #[test]
    fn large_has_path() {
        let mut g = DiGraph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(4);
        let n4 = g.add_node(3);
        let n5 = g.add_node(4);
        let n6 = g.add_node(2);
        let n7 = g.add_node(1);
        let n8 = g.add_node(4);
        let n9 = g.add_node(2);
        let n10 = g.add_node(3);
        let n11 = g.add_node(1);
        let n12 = g.add_node(2);
        let n13 = g.add_node(3);
        let n14 = g.add_node(3);
        let n15 = g.add_node(4);
        g.extend_with_edges([
            (n1, n4),
            (n1, n12),
            (n4, n2),
            (n4, n6),
            (n2, n8),
            (n2, n3),
            (n6, n5),
            (n6, n7),
            (n12, n10),
            (n12, n14),
            (n10, n9),
            (n10, n11),
            (n14, n13),
            (n14, n15),
        ]);
        let res = find_colorful_path(&g, 4);
        assert!(res);
    }

    #[test]
    fn large_no_path() {
        let mut g = DiGraph::new();
        let n1 = g.add_node(1);
        let n2 = g.add_node(2);
        let n3 = g.add_node(4);
        let n4 = g.add_node(3);
        let n5 = g.add_node(4);
        let n6 = g.add_node(2);
        let n7 = g.add_node(1);
        let n8 = g.add_node(4);
        let n9 = g.add_node(2);
        let n10 = g.add_node(3);
        let n11 = g.add_node(1);
        let n12 = g.add_node(2);
        let n13 = g.add_node(3);
        let n14 = g.add_node(3);
        let n15 = g.add_node(4);
        g.extend_with_edges([
            (n1, n4),
            (n1, n12),
            (n4, n2),
            (n4, n6),
            (n2, n8),
            (n2, n3),
            (n6, n5),
            (n6, n7),
            (n12, n10),
            (n12, n14),
            (n10, n9),
            (n10, n11),
            (n14, n13),
            (n14, n15),
        ]);
        let res = find_colorful_path(&g, 5);
        assert!(!res);
    }
}
