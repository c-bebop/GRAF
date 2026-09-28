use std::fmt;

use crate::graph_ds::{Edge, VertexID, Weight, WeightedEdge, WeightedTarget};

pub trait Graph {
    type Target: PartialEq;

    fn neighbors(&self, u: VertexID) -> &[Self::Target];
}


pub trait WeightStatic {
    fn weight(&self, e: Edge) -> Weight;
}

pub trait WeightDynamic {
    fn weight(&mut self, e: WeightedEdge) -> bool;
}

pub trait NeighborDynamic {
    fn insert(&mut self, e: Edge) -> bool;
}

#[derive(Debug)]
pub struct AdjList {
    list: Vec<Vec<VertexID>>,
}

impl From<Vec<Edge>> for AdjList {
    fn from(mut edges: Vec<Edge>) -> Self {
        let mut list = Vec::new();

        edges.sort_by(|a, b| (a.source).cmp(&b.source));

        let mut u_current = 0;
        list.push(Vec::new());
        for e in edges {
            while e.source > u_current {
                u_current += 1;
                list.push(Vec::new());
            }
            
            let idx: usize = u_current as usize;
            list[idx].push(e.target);
        }

        Self { list: list }
    }
}

impl Graph for AdjList {
    type Target = VertexID;

    fn neighbors(&self, u: VertexID) -> &[VertexID] {
        assert!(self.list.len() >= u as usize);

        &self.list[u as usize]
    }
}

impl fmt::Display for AdjList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (u, neighbors) in self.list.iter().enumerate() {
            write!(f, "N({}): ", u)?;
            for t in neighbors {
                write!(f, "{}, ", t)?;
            }
            write!(f, "\n")?;
        }

        Ok(())      
    }
}

// ╔══════════════════════════════════════╗
// ║            Test Suite                ║
// ╚══════════════════════════════════════╝
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_and_from() {
        let t: WeightedTarget = WeightedTarget{vertex: 0, weight: 1.0};

        assert_eq!(t, WeightedTarget::from((0, 1.0)));
    }

    #[test]
    fn edge_and_from() {
        let e = Edge{source: 0, target: 1};

        assert_eq!(e, Edge::from((0, 1)));
    }

    fn test_adj_list() -> AdjList{
        let edges = vec![Edge::from((0, 1)), Edge::from((0, 2)), Edge::from((1, 0)), Edge::from((2, 1))];

        AdjList::from(edges)
    }

    fn impl_static_graph_neighbors(graph: &AdjList) -> bool {
        vec![1 as VertexID, 2 as VertexID] == graph.neighbors(0)
    }

    #[test]
    fn adj_list_graph_trait() {
        let graph = test_adj_list();

        assert!(impl_static_graph_neighbors(&graph));
    }

    fn impl_weight_dynamic_graph_weight(graph: &mut (impl WeightStatic + WeightDynamic)) -> bool {
        let new_weight = 2.0;        
        let dynamic_weight_set_successful =WeightDynamic::weight(graph, WeightedEdge::from((Edge::from((0, 1)), new_weight)));
        assert!(dynamic_weight_set_successful);

        let last_weight: f32 = WeightStatic::weight(graph, Edge::from((0, 1)));
        println!("{last_weight} and {new_weight}");
        last_weight == new_weight
    }

    // TODO: not working as of yet?
    #[test]
    fn adj_list_weight_dynamic_trait() {
        // let mut graph: AdjList = test_adj_list();
        // assert!(impl_weight_dynamic_graph_weight(&mut graph));
    }

}
