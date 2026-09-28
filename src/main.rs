use std::env;
use std::path::Path;

use rand::prelude::*;

mod graph;
mod graph_io;
mod graph_ds;
mod algorithm;

// Quick Run: 
// $ cargo run src/graphs/ENZYMES_g1.edges 10
fn main() {
    let args: Vec<String> = env::args().collect();

    let (graph_file_name, walk_length_input) = match args.len() {
        x if x > 2 => {
            (args[1].clone(), (&args[2]).parse::<usize>())
        },
        _ => {
            (String::from("src/graphs/ENZYMES_g1.edges"), Ok(5 as usize))
        },
    };

    let graph_file = Path::new(&graph_file_name);
    if !graph_file.exists() {
        println!("Error! Graph file name does not resolve to a path!");
    }

    let buffer = graph_io::read::lines_buffer(graph_file);
    let walk_length = walk_length_input.unwrap_or(5);

    match buffer {
        Ok(lines) => {
            let edges = graph_io::read::unweighted(lines);
            let graph = graph::AdjList::from(edges);

            println!("Graph: {}", graph);

            let mut rng = rand::rng();
            let mut rng_fun = |deg: graph_ds::Degree| -> usize {
                rng.random_range(0..deg) as usize
            };

            let path = algorithm::rw::order::first::walk(algorithm::rw::order::first::step, &graph, 30, walk_length, &mut rng_fun);

            println!("Path: {path}");
        },
        Err(some_error) => println!("Can't read in buffer with error: {some_error}")
    }
    
}


