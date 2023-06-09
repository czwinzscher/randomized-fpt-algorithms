use std::path::PathBuf;

use clap::{Parser, Subcommand};
use petgraph::{graph::DiGraph, prelude::UnGraph};
use randomized_fpt::color_coding;
use randomized_fpt::feedback_vertex_set;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    graph: PathBuf,

    #[arg(short)]
    k: u32,

    #[arg(short, long)]
    repeats: u32,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ColorCoding,
    FeedbackVertexSet,
}

fn main() {
    let cli = Cli::parse();

    let graph_file_contents =
        fs::read_to_string(cli.graph).expect("unable to read graph file");

    let edges = graph_file_contents.lines().filter_map(|l| {
        l.split_once(',').and_then(|(s1, s2)| {
            Some((
                s1.trim().parse::<u32>().ok()?,
                s2.trim().parse::<u32>().ok()?,
            ))
        })
    });

    match &cli.command {
        Command::ColorCoding => {
            let g = DiGraph::from_edges(edges);
            let k = cli.k;
            let res = color_coding::find_simple_path(g, k, cli.repeats);
            if res {
                println!("the graph contains a simple path of length {k}");
            } else {
                println!("no simple path of length {k} found");
            }
        }
        Command::FeedbackVertexSet => {
            let mut g = UnGraph::from_edges(edges);
            let k = cli.k;
            let res = feedback_vertex_set::find_feedback_vertex_set(
                &mut g,
                k,
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
    }
}
