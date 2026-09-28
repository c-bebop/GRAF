use std::collections::HashMap;
use std::fmt;

use crate::graph_ds::{self, Degree, Edge, VertexID};

// ╔══════════════════════════════════════╗
// ║            MaxNNZVertex              ║
// ╚══════════════════════════════════════╝

#[derive(Debug, PartialEq)]
pub struct MaxNNZVertex {
    pub id: VertexID,
    pub degree: Degree,
}

impl From<(VertexID, Degree)> for MaxNNZVertex {
    fn from((u, deg): (VertexID, Degree)) -> Self {
        Self { id: u, degree: deg }
    }
}

pub const MAX_NNZ_VERTEX_NOT_AVAIL: MaxNNZVertex = MaxNNZVertex{id: graph_ds::INVALID_VERTEX, degree: 0};

impl fmt::Display for MaxNNZVertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id {
            graph_ds::INVALID_VERTEX => write!(f, "deg(<InvalidVertex>)=n/a"),
            _ => write!(f, "deg({})={}", self.id, self.degree),
        }       
    }
}

pub fn max_nnz(edges: &[Edge]) -> MaxNNZVertex {
    assert!(edges.iter().count() != 0);

    let mut vertices: HashMap<VertexID, Degree> = HashMap::new(); 
    
    for edge in edges {
        *vertices.entry(edge.source).or_insert(0) += 1;
    }

    let max_deg_vertex: Option<(&VertexID, &Degree)> = vertices.iter().max_by_key(|(_, v)| *v);

    match max_deg_vertex {
        Some((max_deg_vertex_id, deg)) => MaxNNZVertex { id: *max_deg_vertex_id, degree: *deg },
        None => MAX_NNZ_VERTEX_NOT_AVAIL,
    }
}

// ╔══════════════════════════════════════╗
// ║         1st Order Random Walk        ║
// ╚══════════════════════════════════════╝

pub mod rw {
    pub mod order {
        pub mod first {
            use std::range;

            use crate::{graph::Graph, graph_ds::VertexID};
            use crate::graph_ds::{Degree, Path};
            
            pub fn step<Rng>(neighborhood: &[VertexID], rng: &mut Rng) -> Option<VertexID>
            where 
                Rng: FnMut(Degree) -> usize {
                let chosen_neighbor = match neighborhood.len() {
                    0 => None,
                    1 => Some(neighborhood[0]),
                    _ => {
                        let rand_idx = rng(neighborhood.len() as Degree);
                        assert!(rand_idx < neighborhood.len());
                        Some(neighborhood[rand_idx])
                    },
                };

                chosen_neighbor
            }

            pub fn walk<Step, GraphT: Graph, Rng>(step_f: Step, graph: & GraphT, start_vertex: VertexID, steps: usize, rng: &mut Rng) -> Path
            where 
                Step: Fn(&[GraphT::Target], &mut Rng) -> Option<VertexID> {
                    let mut path = Path{data: vec![start_vertex]};
                    let mut current_vertex = start_vertex;
                    for _step_idx in range::RangeInclusive::from(2..=steps) {
                        match step_f(graph.neighbors(current_vertex), rng) {
                            Some(v) => {
                                current_vertex = v;        
                                path.data.push(current_vertex);
                            }
                            None => {
                                break;
                            }
                        }
                    }

                    path
            }
        }
    }
}


#[cfg(test)]
mod test {
    use rand::{RngExt, SeedableRng};
    use rand::rngs::SmallRng;
    use crate::graph::{AdjList, Graph};
    use super::*;

    #[test]
    fn rw_order_first_step() {
        let e1: Edge = Edge::from((1, 0));
        let e2: Edge = Edge::from((0, 1));
        let e3: Edge = Edge::from((0, 2));
        let graph = AdjList::from(vec![e1, e2, e3]);

        let mut rng = SmallRng::seed_from_u64(42);
        let neighborhood = graph.neighbors(0);
        assert!(neighborhood.len() == 2);

        let mut rng_f = | deg: Degree | -> usize {
            rng.random_range(0..deg as usize)
        };

        let next_v = rw::order::first::step(neighborhood, &mut rng_f);
        assert!(next_v.is_some());
        let next_v = next_v.unwrap();

        assert!((next_v >= 1 && next_v <= 2));
    }

    #[test]
    fn max_nnz_finding_max() {
        let e1: Edge = Edge::from((1, 0));
        let e2: Edge = Edge::from((0, 1));
        let e3: Edge = Edge::from((0, 2));
        let edges = vec![e1, e2, e3];

        let expected_nnz = MaxNNZVertex::from((0, 2));
        assert_eq!(expected_nnz, max_nnz(&edges));
    }
}
