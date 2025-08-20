pub fn update_coordinate_bounds(vertex: [f32; 3], min: &mut [f32; 3], max: &mut [f32; 3]) {
    min[0] = min[0].min(vertex[0]);
    min[1] = min[1].min(vertex[1]);
    min[2] = min[2].min(vertex[2]);

    max[0] = max[0].max(vertex[0]);
    max[1] = max[1].max(vertex[1]);
    max[2] = max[2].max(vertex[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_bounds_basic() {
        let mut min = [0.0, 0.0, 0.0];
        let mut max = [0.0, 0.0, 0.0];
        let vertex = [1.0, 2.0, 3.0];

        update_coordinate_bounds(vertex, &mut min, &mut max);

        assert_eq!(min, [0.0, 0.0, 0.0]);
        assert_eq!(max, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_update_bounds_negative_vertex() {
        let mut min = [0.0, 0.0, 0.0];
        let mut max = [0.0, 0.0, 0.0];
        let vertex = [-1.0, -2.0, -3.0];

        update_coordinate_bounds(vertex, &mut min, &mut max);

        assert_eq!(min, [-1.0, -2.0, -3.0]);
        assert_eq!(max, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_update_bounds_mixed_coordinates() {
        let mut min = [1.0, -1.0, 0.0];
        let mut max = [2.0, 1.0, 3.0];
        let vertex = [0.5, 2.0, -1.0];

        update_coordinate_bounds(vertex, &mut min, &mut max);

        assert_eq!(min, [0.5, -1.0, -1.0]);
        assert_eq!(max, [2.0, 2.0, 3.0]);
    }

    #[test]
    fn test_update_bounds_no_change() {
        let mut min = [-5.0, -5.0, -5.0];
        let mut max = [5.0, 5.0, 5.0];
        let vertex = [0.0, 0.0, 0.0];

        update_coordinate_bounds(vertex, &mut min, &mut max);

        assert_eq!(min, [-5.0, -5.0, -5.0]);
        assert_eq!(max, [5.0, 5.0, 5.0]);
    }

    #[test]
    fn test_update_bounds_sequential_vertices() {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];

        let vertices = [[1.0, 2.0, 3.0], [-1.0, 4.0, 1.0], [3.0, -2.0, 5.0]];

        for vertex in vertices {
            update_coordinate_bounds(vertex, &mut min, &mut max);
        }

        assert_eq!(min, [-1.0, -2.0, 1.0]);
        assert_eq!(max, [3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_update_bounds_edge_values() {
        let mut min = [0.0, 0.0, 0.0];
        let mut max = [0.0, 0.0, 0.0];
        let vertex = [f32::MAX, f32::MIN, 0.0];

        update_coordinate_bounds(vertex, &mut min, &mut max);

        assert_eq!(min, [0.0, f32::MIN, 0.0]);
        assert_eq!(max, [f32::MAX, 0.0, 0.0]);
    }
}
