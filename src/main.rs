use petgraph::graph::DiGraph;

fn main() {
    let k = 4;
    let g = DiGraph::<u32, ()>::from_edges([
        (1, 4),
        (1, 12),
        (4, 2),
        (4, 6),
        (2, 8),
        (2, 3),
        (6, 5),
        (6, 7),
        (12, 10),
        (12, 14),
        (10, 9),
        (10, 11),
        (14, 13),
        (14, 15),
    ]);

    let res = randomized_fpt::find_simple_path(g, k);
    if res {
        println!("the graph contains a simple path of length {k}");
    } else {
        println!("no simple path of length {k} found");
    }
}