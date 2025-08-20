pub fn calculate_polygon_areas(
    points: &[f64],
    connectivity: &[usize],
    offsets: &[usize],
) -> Vec<f64> {
    let mut areas = Vec::with_capacity(offsets.len());
    let mut conn_idx = 0;

    for &offset in offsets {
        let poly_indices = &connectivity[conn_idx..offset];
        conn_idx = offset;

        if poly_indices.len() < 3 {
            areas.push(0.0);
            continue;
        }

        let p0_idx = poly_indices[0] * 3;
        let p0 = (points[p0_idx], points[p0_idx + 1], points[p0_idx + 2]);
        let mut area = 0.0;

        for i in 1..(poly_indices.len() - 1) {
            let p1_idx = poly_indices[i] * 3;
            let p2_idx = poly_indices[i + 1] * 3;

            let p1 = (points[p1_idx], points[p1_idx + 1], points[p1_idx + 2]);
            let p2 = (points[p2_idx], points[p2_idx + 1], points[p2_idx + 2]);

            let v1 = (p1.0 - p0.0, p1.1 - p0.1, p1.2 - p0.2);
            let v2 = (p2.0 - p0.0, p2.1 - p0.1, p2.2 - p0.2);

            let cross = (
                v1.1 * v2.2 - v1.2 * v2.1,
                v1.2 * v2.0 - v1.0 * v2.2,
                v1.0 * v2.1 - v1.1 * v2.0,
            );

            area += 0.5 * (cross.0.powi(2) + cross.1.powi(2) + cross.2.powi(2)).sqrt();
        }
        areas.push(area);
    }

    areas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area() {
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0
            1.0, 0.0, 0.0, // vertex 1
            0.0, 1.0, 0.0, // vertex 2
        ];
        let connectivity = vec![0, 1, 2];
        let offsets = vec![3];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        assert!((areas[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_xnorm_square_area() {
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0
            0.0, 1.0, 0.0, // vertex 1
            0.0, 1.0, 1.0, // vertex 2
            0.0, 0.0, 1.0, // vertex 3
        ];
        let connectivity = vec![0, 1, 2, 3];
        let offsets = vec![4];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        assert!((areas[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ynorm_square_area() {
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0
            0.0, 0.0, 1.0, // vertex 1
            1.0, 0.0, 1.0, // vertex 2
            1.0, 0.0, 0.0, // vertex 3
        ];
        let connectivity = vec![0, 1, 2, 3];
        let offsets = vec![4];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        assert!((areas[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_znorm_square_area() {
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0
            1.0, 0.0, 0.0, // vertex 1
            1.0, 1.0, 0.0, // vertex 2
            0.0, 1.0, 0.0, // vertex 3
        ];
        let connectivity = vec![0, 1, 2, 3];
        let offsets = vec![4];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        assert!((areas[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiple_polygons() {
        // Two triangles
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0 - first triangle, 0.5 area
            1.0, 0.0, 0.0, // vertex 1
            0.0, 1.0, 0.0, // vertex 2
            2.0, 0.0, 0.0, // vertex 3 - second triangle, 1.0 area
            4.0, 0.0, 0.0, // vertex 4
            3.0, 1.0, 0.0, // vertex 5
        ];
        let connectivity = vec![0, 1, 2, 3, 4, 5];
        let offsets = vec![3, 6];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 2);
        assert!((areas[0] - 0.5).abs() < 1e-10);
        assert!((areas[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_degenerate_polygon_less_than_3_vertices() {
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0
            1.0, 0.0, 0.0, // vertex 1
        ];
        let connectivity = vec![0, 1];
        let offsets = vec![2];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        assert_eq!(areas[0], 0.0);
    }

    #[test]
    fn test_empty_polygon() {
        let points = vec![];
        let connectivity = vec![];
        let offsets = vec![0];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        assert_eq!(areas[0], 0.0);
    }

    #[test]
    fn test_3d_triangle() {
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0
            1.0, 0.0, 1.0, // vertex 1
            0.0, 1.0, 1.0, // vertex 2
        ];
        let connectivity = vec![0, 1, 2];
        let offsets = vec![3];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        let expected_area = (3.0_f64).sqrt() / 2.0; // area verified in blender
        assert!((areas[0] - expected_area).abs() < 1e-10);
    }

    #[test]
    fn test_collinear_points() {
        // Three collinear points should have zero area
        let points = vec![
            0.0, 0.0, 0.0, // vertex 0
            1.0, 0.0, 0.0, // vertex 1
            2.0, 0.0, 0.0, // vertex 2
        ];
        let connectivity = vec![0, 1, 2];
        let offsets = vec![3];

        let areas = calculate_polygon_areas(&points, &connectivity, &offsets);
        assert_eq!(areas.len(), 1);
        assert!(areas[0].abs() < 1e-10);
    }
}
