use std::fmt;

pub type VertexID = u64;
pub type Weight = f32;
pub type Degree = u64;

pub const INVALID_VERTEX: VertexID = std::u64::MAX;
pub const INVALID_WEIGHT: Weight = std::f32::INFINITY;

// ╔══════════════════════════════════════╗
// ║            WeightedTarget            ║
// ╚══════════════════════════════════════╝

#[derive(Debug, PartialEq, PartialOrd)]
pub struct WeightedTarget {
    pub vertex: VertexID,
    pub weight: Weight,
}

impl From<(VertexID, Weight)> for WeightedTarget {
    fn from((v, w): (VertexID, Weight)) -> Self {
        Self { vertex: v, weight: w }
    }
}

impl fmt::Display for WeightedTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.vertex, self.weight) {
            (INVALID_VERTEX, INVALID_WEIGHT) => write!(f, "<InvalidVertex>:{}", INVALID_WEIGHT),
            _ => write!(f, "{}:{}", self.vertex, self.weight),
        }
    }
}

// ╔══════════════════════════════════════╗
// ║                EDGE                  ║
// ╚══════════════════════════════════════╝

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Edge {
    pub source: VertexID,
    pub target: VertexID,
}

impl From<(VertexID, VertexID)> for Edge {
    fn from((s, t): (VertexID, VertexID)) -> Self {
        Self { source: s, target: t }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}=>{})", self.source, self.target)
    }
}


// ╔══════════════════════════════════════╗
// ║            WeightedEdge              ║
// ╚══════════════════════════════════════╝

#[derive(Debug, PartialEq, PartialOrd)]
pub struct WeightedEdge {
    pub edge: Edge,
    pub weight: Weight,
}

impl From<(Edge, Weight)> for WeightedEdge {
    fn from((e, w): (Edge, Weight)) -> Self {
        Self { edge: e, weight: w }
    }
}

impl fmt::Display for WeightedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}=>{}:{})", self.edge.source, self.edge.target, self.weight)
    }
}

// ╔══════════════════════════════════════╗
// ║            Timestamped               ║
// ╚══════════════════════════════════════╝

pub type Timestamp = u64;

pub struct TimestampedEdge {
    pub edge: Edge,
    pub timestamp: Timestamp,
}

impl fmt::Display for TimestampedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.edge, self.timestamp)
    }
}


// ╔══════════════════════════════════════╗
// ║                Path                  ║
// ╚══════════════════════════════════════╝

pub struct Path {
    pub data: Vec<VertexID>
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.len() == 0 {
            write!(f, "[]")
        } else {
            write!(f, "[")?;
            for (idx, v) in self.data.iter().enumerate() {
                if idx == self.data.len() - 1 {
                    break;
                } else {
                    write!(f, "{}, ", v)?;
                }
            }
    
            write!(f, "{}]", self.data.last().unwrap())
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_path() {
        let path = Path{data: vec![1, 2, 3]};

        assert_eq!(path.to_string(), "[1, 2, 3]");
    }

}
