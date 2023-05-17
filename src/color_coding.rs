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
