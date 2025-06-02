/// 1. Load SEC file.
///
/// 2. Extract plumb lines and its top-face equations.
///
/// 3. Calculate the coordinates of all the points from the plumb lines and the top-face equations. (Include the intersections of the plumb lines and horizontal plane and polygon's top-face)
///
/// 4. Then use Bevy to render the mesh.
///
/// Then the mesh must convert to original commandos SEC format.
pub mod mesh;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mesh::Mesh;
    use sec_loader::sec::Sec;

    use super::*;

    #[test]
    fn test_mesh() {
        let sec = Sec::from_file(PathBuf::from("../test_sec/TU01EX.SEC")).unwrap();
        let mesh = Mesh::from(&sec);

        println!("{:#?}", mesh);
    }
}
