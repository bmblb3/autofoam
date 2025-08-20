use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn process_binary_iter(
    file: File,
) -> impl Iterator<Item = Result<[f32; 3], Box<dyn std::error::Error>>> {
    BinaryVertexIterator::new(file)
}

struct BinaryVertexIterator {
    file: File,
    triangle_count: usize,
    current_triangle: usize,
    current_vertex: usize,
    error: Option<Box<dyn std::error::Error>>,
}

impl BinaryVertexIterator {
    fn new(mut file: File) -> Self {
        let (triangle_count, error) = match Self::read_header(&mut file) {
            Ok(count) => (count, None),
            Err(e) => (0, Some(e)),
        };

        Self {
            file,
            triangle_count,
            current_triangle: 0,
            current_vertex: 0,
            error,
        }
    }

    fn read_header(file: &mut File) -> Result<usize, Box<dyn std::error::Error>> {
        // https://en.wikipedia.org/wiki/STL_(file_format)
        // "A binary STL file has an 80-character header that is generally ignored"
        file.seek(SeekFrom::Start(80))?;

        // "Following the header is a 4-byte
        // little-endian unsigned integer
        // indicating the number of triangular facets
        // in the file"
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf) as usize)
    }
}

impl Iterator for BinaryVertexIterator {
    type Item = Result<[f32; 3], Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(error) = self.error.take() {
            return Some(Err(error));
        }

        // "Following that is data describing each triangle in turn.
        // The file simply ends after the last triangle."
        if self.current_triangle >= self.triangle_count {
            return None;
        }

        // "Each triangle is described by 12 32-bit floating-point numbers:
        // 3 for the normal"
        // 3 * 32-bit = 3 * (4 * 8-bit) = 3 * 4 bytes = 12 bytes
        // Normal can be ignored
        if self.current_vertex == 0 {
            if let Err(e) = self.file.seek(SeekFrom::Current(12)) {
                return Some(Err(e.into()));
            }
        }

        // "... and then 3 for the X/Y/Z coordinate of each vertex"
        // Each coordinate component is 32-bit (4 bytes)
        let mut buf = [0u8; 4];

        let x = match self.file.read_exact(&mut buf) {
            Ok(()) => f32::from_le_bytes(buf),
            Err(e) => return Some(Err(e.into())),
        };

        let y = match self.file.read_exact(&mut buf) {
            Ok(()) => f32::from_le_bytes(buf),
            Err(e) => return Some(Err(e.into())),
        };

        let z = match self.file.read_exact(&mut buf) {
            Ok(()) => f32::from_le_bytes(buf),
            Err(e) => return Some(Err(e.into())),
        };

        self.current_vertex += 1;

        // If we've read all 3 vertices of this triangle, move to next triangle
        if self.current_vertex >= 3 {
            self.current_vertex = 0;
            self.current_triangle += 1;

            // "After these follows a 2-byte ('short') unsigned integer
            // that is the 'attribute byte count'"
            // We skip this
            if let Err(e) = self.file.seek(SeekFrom::Current(2)) {
                return Some(Err(e.into()));
            }
        }

        Some(Ok([x, y, z]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    fn create_test_stl(triangle_count: u32, vertices: &[[f32; 3]]) -> File {
        let mut file = tempfile().unwrap();

        // 80-byte header
        file.write_all(&[0u8; 80]).unwrap();
        // Triangle count (4 bytes, little endian)
        file.write_all(&triangle_count.to_le_bytes()).unwrap();

        for chunk in vertices.chunks(3) {
            // Normal vector (3 f32s)
            file.write_all(&[0u8; 12]).unwrap();
            // 3 vertices
            for v in chunk {
                for f in v {
                    file.write_all(&f.to_le_bytes()).unwrap();
                }
            }
            // Attribute byte count (2 bytes)
            file.write_all(&[0u8; 2]).unwrap();
        }

        file.seek(SeekFrom::Start(0)).unwrap();
        file
    }

    #[test]
    fn test_process_binary_iter_reads_vertices() {
        let vertices = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let file = create_test_stl(1, &vertices);
        let mut iter = process_binary_iter(file);

        for expected in vertices.iter() {
            let v = iter.next().unwrap().unwrap();
            assert_eq!(&v, expected);
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_process_binary_iter_handles_invalid_header() {
        let mut file = tempfile().unwrap();
        // Write less than 84 bytes to simulate a corrupt header
        file.write_all(&[0u8; 10]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut iter = process_binary_iter(file);
        assert!(iter.next().unwrap().is_err());
    }
}
