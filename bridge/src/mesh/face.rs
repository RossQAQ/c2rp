use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub px: f32,

    pub py: f32,

    pub pz: f32,
}

impl Vertex {
    pub(crate) fn new(px: f32, py: f32, pz: f32) -> Self {
        Self { px, py, pz }
    }
}

/// Only top-face makes sense to be rendered.
#[derive(Debug, Clone)]
pub(crate) struct TopFace {
    /// vertices, counter-clockwise
    vertices: Vec<Vertex>,

    /// vertex id -> array idx
    lookup: BTreeMap<i32, usize>,
}

impl TopFace {
    pub(crate) fn new() -> Self {
        Self {
            vertices: Vec::new(),
            lookup: BTreeMap::new(),
        }
    }

    pub(crate) fn insert_vertex(&mut self, vertex: Vertex, idx: i32) -> &mut Self {
        if !self.lookup.contains_key(&idx) {
            self.vertices.push(vertex);
            self.lookup.insert(idx, self.vertices.len() - 1);
        }
        self
    }

    pub(crate) fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl IntoIterator for TopFace {
    type Item = Vertex;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vertices.into_iter()
    }
}
