// This implementation is based on the algorithm described in:
// "Pearls of Functional Algorithm Design" by Richard Bird, Cambridge University Press , ISBN: 9780511763199
// Reference: Chapter 9, Page 56 -- Finding celebrities

mod clique;

use clique::{Clique, clique2digraph, person::Person};
use petgraph::dot::{Config, Dot};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

const DIR_PATH: &str = "output";
const DOT_FILE_PATH: &str = "output/graph.dot";
const PNG_FILE_PATH: &str = "output/graph.png";

fn main() -> std::io::Result<()> {
    let ps = HashSet::from_iter(
        [
            (1_usize, vec![1, 2, 3]),
            (2, vec![1, 3]),
            (3, vec![1, 2]),
            (4, vec![1, 2, 3, 42]),
            (5, vec![1, 2, 3, 4, 5]),
            (6, vec![1, 2, 3, 7]),
            (7, vec![1, 2, 3, 5, 6]),
        ]
        .map(Person::from),
    );

    let css = ps.cclique().unwrap_or_default();
    println!(
        "{:#?}",
        css.iter().map(ToString::to_string).collect::<Vec<_>>()
    );

    let graph = clique2digraph(&ps);

    if !Path::new(DIR_PATH).exists() {
        fs::create_dir_all(DIR_PATH)?;
    }

    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    let mut dot_file = File::create(DOT_FILE_PATH)?;
    write!(dot_file, "{dot:?}")?;

    // convert to png by Graphviz
    let output = Command::new("dot")
        .args(["-Tpng", DOT_FILE_PATH, "-o", PNG_FILE_PATH])
        .output()?;

    if !output.status.success() {
        eprintln!(
            "conversion failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
