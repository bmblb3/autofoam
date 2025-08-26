use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn process_ascii_iter(
    file: File,
) -> impl Iterator<Item = Result<[f32; 3], Box<dyn std::error::Error>>> {
    let reader = BufReader::new(file);
    reader.lines().filter_map(|line_result| match line_result {
        Ok(line) => line.trim().strip_prefix("vertex").and_then(|coords| {
            if coords.starts_with(char::is_whitespace) {
                // enter branch if line is [:space:]*vertex[:space:]*.*
                let mut parts = coords.split_whitespace();
                match (parts.next(), parts.next(), parts.next()) {
                    (Some(x_str), Some(y_str), Some(z_str)) => {
                        match (
                            x_str.parse::<f32>(),
                            y_str.parse::<f32>(),
                            z_str.parse::<f32>(),
                        ) {
                            (Ok(x), Ok(y), Ok(z)) => Some(Ok([x, y, z])),
                            _ => Some(Err("Invalid vertex coordinate".into())),
                        }
                    }
                    _ => Some(Err("Incomplete vertex coordinate".into())),
                }
            } else {
                None
            }
        }),
        Err(e) => Some(Err(e.into())),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> File {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        let mut file = temp_file.into_file();
        file.seek(SeekFrom::Start(0)).unwrap();
        file
    }

    #[test]
    fn test_valid_vertices() {
        let content = "\
vertex 1.0 2.0 3.0
vertex -1.5 0.0 4.2";
        let file = create_test_file(content);

        let vertices: Result<Vec<[f32; 3]>, _> = process_ascii_iter(file).collect();
        let vertices = vertices.unwrap();

        assert_eq!(vertices.len(), 2);
        assert_eq!(vertices[0], [1.0, 2.0, 3.0]);
        assert_eq!(vertices[1], [-1.5, 0.0, 4.2]);
    }

    #[test]
    fn test_mixed_content() {
        let content = "\
solid test
vertex 1.0 2.0 3.0
facet normal 0.0 0.0 1.0
vertex 4.0 5.0 6.0
endsolid";
        let file = create_test_file(content);

        let vertices: Result<Vec<[f32; 3]>, _> = process_ascii_iter(file).collect();
        let vertices = vertices.unwrap();

        assert_eq!(vertices.len(), 2);
        assert_eq!(vertices[0], [1.0, 2.0, 3.0]);
        assert_eq!(vertices[1], [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_invalid_coordinates() {
        let content = "\
vertex 1.0 invalid 3.0
vertex 4.0 5.0 6.0";
        let file = create_test_file(content);

        let results: Vec<_> = process_ascii_iter(file).collect();

        assert_eq!(results.len(), 2);
        assert!(results[0].is_err());
        assert_eq!(
            results[0].as_ref().unwrap_err().to_string(),
            "Invalid vertex coordinate"
        );
        assert!(results[1].is_ok());
        assert_eq!(results[1].as_ref().unwrap(), &[4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_incomplete_coordinates() {
        let content = "\
vertex 1.0 2.0
vertex 4.0 5.0 6.0";
        let file = create_test_file(content);

        let results: Vec<_> = process_ascii_iter(file).collect();

        assert_eq!(results.len(), 2);
        assert!(results[0].is_err());
        assert_eq!(
            results[0].as_ref().unwrap_err().to_string(),
            "Incomplete vertex coordinate"
        );
        assert!(results[1].is_ok());
        assert_eq!(results[1].as_ref().unwrap(), &[4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_whitespace_handling() {
        let content = "\
vertex 1.0 2.0 3.0
vertex  4.0 5.0 6.0
vertex 7.0  8.0 9.0
vertex 10.0 11.0 12.0
\tvertex\t13.0\t14.0\t15.0\t";
        let file = create_test_file(content);

        let vertices: Result<Vec<[f32; 3]>, _> = process_ascii_iter(file).collect();
        let vertices = vertices.unwrap();

        assert_eq!(vertices.len(), 5);
        assert_eq!(vertices[0], [1.0, 2.0, 3.0]);
        assert_eq!(vertices[1], [4.0, 5.0, 6.0]);
        assert_eq!(vertices[2], [7.0, 8.0, 9.0]);
        assert_eq!(vertices[3], [10.0, 11.0, 12.0]);
        assert_eq!(vertices[4], [13.0, 14.0, 15.0]);
    }

    #[test]
    fn test_empty_file() {
        let content = "";
        let file = create_test_file(content);

        let vertices: Result<Vec<[f32; 3]>, _> = process_ascii_iter(file).collect();
        let vertices = vertices.unwrap();

        assert_eq!(vertices.len(), 0);
    }

    #[test]
    fn test_no_vertices() {
        let content = "\
solid test
facet normal 0.0 0.0 1.0
endsolid";
        let file = create_test_file(content);

        let vertices: Result<Vec<[f32; 3]>, _> = process_ascii_iter(file).collect();
        let vertices = vertices.unwrap();

        assert_eq!(vertices.len(), 0);
    }
}
