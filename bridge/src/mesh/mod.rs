mod face;

use std::collections::BTreeMap;

use face::{TopFace, Vertex};
use sec_loader::sec::Sec;

/// C2 SEC to Mesh, which then can be used by Bevy.
#[derive(Debug, Clone)]
pub struct Mesh {
    /// face id -> face
    faces: BTreeMap<i32, TopFace>,
}

impl From<&Sec> for Mesh {
    fn from(sec: &Sec) -> Self {
        let mut faces = BTreeMap::new();

        let plumb_lines = sec.get_plumb_lines();
        let sides = sec.get_sides();
        let polygons = sec.get_polygons();

        for (idx, polygon) in polygons.into_iter().enumerate() {
            let mut top_face = TopFace::new();

            polygon.side_indexes.iter().for_each(|&side_idx| {
                let side = sides.get(side_idx as usize).unwrap();
                let start = plumb_lines.get(side.start as usize).unwrap();
                let end = plumb_lines.get(side.end as usize).unwrap();

                let start = Vertex::new(start.x, start.y, 0.0);
                let end = Vertex::new(end.x, end.y, 0.0);

                top_face
                    .insert_vertex(start, side.start)
                    .insert_vertex(end, side.end);
            });

            assert_eq!(polygon.sides_num, top_face.len() as i32);

            faces.insert(idx as i32, top_face);
        }

        Self { faces }
    }
}

impl From<Mesh> for Sec {
    fn from(mesh: Mesh) -> Self {
        todo!()
    }
}

impl Mesh {
    /// compare the mesh metadata with original sec header to verify
    pub fn verify(&self, sec: &Sec) -> bool {
        todo!()
    }
}
