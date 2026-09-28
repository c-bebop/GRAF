pub mod read {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    use crate::graph_ds::Edge;

    pub fn unweighted(lines: io::Lines<io::BufReader<File>>) -> Vec<Edge> {
        let mut edges = Vec::new();

        for line in lines.map_while(Result::ok) {
            let mut input_fields = line.split_whitespace().map(str::parse::<u64>);

            let u_expected = input_fields.next();
            match u_expected {
                Some(u_valid) =>
                    match u_valid {
                        Ok(u) => {
                            let v_expected = input_fields.next();
                            match v_expected {
                                Some(v_valid) =>
                                    match v_valid {
                                        Ok(v) => {
                                            edges.push(Edge::from((u, v)));
                                        }
                                        Err(_) => println!("Could not get result for field u"),
                                    },
                                None => println!("Could parse field v!"),
                            }
                        },
                        Err(_) => println!("Could not get parse field u."),
                    },
                None => println!("Could not read in field!"),
            }
        }

        edges
    }

    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    pub fn lines_buffer<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

