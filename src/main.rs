use std::path::PathBuf;

use clap::{Parser, Subcommand};
use petgraph::{graph::DiGraph, prelude::UnGraph};
use randomized_fpt::color_coding;
use randomized_fpt::feedback_vertex_set;
use randomized_fpt::random_separation;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    graph: PathBuf,

    #[arg(short, long)]
    repeats: u32,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ColorCoding {
        #[arg(short)]
        k: u32,
    },
    FeedbackVertexSet {
        #[arg(short)]
        k: u32,
    },
    RandomSeparation {
        #[arg(short, long, value_name = "FILE")]
        pattern_graph: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let graph_file_contents =
        fs::read_to_string(cli.graph).expect("unable to read graph file");

    let get_edges_from_file_content = |s: String| {
        s.lines()
            .filter_map(|l| {
                l.split_once(',').and_then(|(s1, s2)| {
                    Some((
                        s1.trim().parse::<u32>().ok()?,
                        s2.trim().parse::<u32>().ok()?,
                    ))
                })
            })
            .collect::<Vec<_>>()
    };

    let edges = get_edges_from_file_content(graph_file_contents);

    match &cli.command {
        Command::ColorCoding { k } => {
            let g = DiGraph::from_edges(edges);
            let res = color_coding::find_simple_path(g, *k, cli.repeats);

            if res {
                println!("the graph contains a simple path of length {k}");
            } else {
                println!("no simple path of length {k} found");
            }
        }
        Command::FeedbackVertexSet { k } => {
            let mut g = UnGraph::from_edges(edges);
            let res = feedback_vertex_set::find_feedback_vertex_set(
                &mut g,
                *k,
                cli.repeats,
            );

            if res {
                println!(
                    "the graph contains a feedback vertex set of size {k}"
                );
            } else {
                println!("no feedback vertex set of size {k} found");
            }
        }
        Command::RandomSeparation { pattern_graph } => {
            let g = UnGraph::from_edges(edges);

            let pattern_graph_file_contents = fs::read_to_string(pattern_graph)
                .expect("unable to read pattern graph file");
            let pattern_graph_edges =
                get_edges_from_file_content(pattern_graph_file_contents);
            let h = UnGraph::from_edges(pattern_graph_edges);

            let res = random_separation::find_subgraph(g, &h, cli.repeats);

            if res {
                println!("the graph contains the subgraph");
            } else {
                println!("subgraph not found");
            }
        }
    }
}
