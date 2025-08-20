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
