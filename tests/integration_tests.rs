use autofoam::vtk::VtpProcessor;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    static TEST_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn create_test_vtp_file() -> String {
        let test_data = r#"<?xml version="1.0"?>
<VTKFile type="PolyData" version="1.0" byte_order="LittleEndian">
  <PolyData>
    <Piece NumberOfPoints="4" NumberOfPolys="1">
      <Points>
        <DataArray type="Float64" NumberOfComponents="3" format="ascii">
          0.0 0.0 0.0
          1.0 0.0 0.0
          1.0 1.0 0.0
          0.0 1.0 0.0
        </DataArray>
      </Points>
      <Polys>
        <DataArray type="Int32" Name="connectivity" format="ascii">
          0 1 2 3
        </DataArray>
        <DataArray type="Int32" Name="offsets" format="ascii">
          4
        </DataArray>
      </Polys>
      <CellData>
        <DataArray type="Float64" Name="test_field" NumberOfComponents="1" format="ascii">
          42.0
        </DataArray>
      </CellData>
    </Piece>
  </PolyData>
</VTKFile>"#;

        let count = TEST_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let test_file = format!("test_data_{}.vtp", count);
        fs::write(&test_file, test_data).unwrap();
        test_file
    }

    fn cleanup_test_file(path: &str) {
        if Path::new(path).exists() {
            fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn test_vtk_reader_from_file() {
        let test_file = create_test_vtp_file();

        let result = VtpProcessor::from_file(&test_file);
        assert!(result.is_ok());

        cleanup_test_file(&test_file);
    }

    #[test]
    fn test_geometry_extraction() {
        let test_file = create_test_vtp_file();
        let reader = VtpProcessor::from_file(&test_file).unwrap();

        let result = reader.geometry();
        assert!(result.is_ok());

        let (points, connectivity, offsets) = result.unwrap();
        assert_eq!(points.len(), 12); // 4 points * 3 coordinates
        assert_eq!(connectivity, vec![0, 1, 2, 3]);
        assert_eq!(offsets, vec![4]);

        cleanup_test_file(&test_file);
    }

    #[test]
    fn test_field_operations() {
        let test_file = create_test_vtp_file();
        let reader = VtpProcessor::from_file(&test_file).unwrap();

        assert!(reader.field_exists("test_field").unwrap());
        assert!(!reader.field_exists("nonexistent_field").unwrap());

        let field_data = reader.field("test_field").unwrap();
        assert_eq!(field_data, vec![42.0]);

        let fields = reader.list_fields().unwrap();
        assert!(fields.contains(&"test_field".to_string()));

        cleanup_test_file(&test_file);
    }

    #[test]
    fn test_add_and_remove_field() {
        let test_file = create_test_vtp_file();
        let reader = VtpProcessor::from_file(&test_file).unwrap();

        let new_data = vec![100.0];
        let reader = reader.add_field("new_field", &new_data).unwrap();
        assert!(reader.field_exists("new_field").unwrap());

        let retrieved_data = reader.field("new_field").unwrap();
        assert_eq!(retrieved_data, new_data);

        let reader = reader.remove_field("new_field").unwrap();
        assert!(!reader.field_exists("new_field").unwrap());

        cleanup_test_file(&test_file);
    }

    #[test]
    fn test_write_to_file() {
        let test_file = create_test_vtp_file();
        let output_file = "test_output.vtp";

        let reader = VtpProcessor::from_file(&test_file).unwrap();
        let result = reader.write_to_file(output_file);

        assert!(result.is_ok());
        assert!(Path::new(output_file).exists());

        cleanup_test_file(&test_file);
        cleanup_test_file(output_file);
    }

    #[test]
    fn test_error_handling() {
        let result = VtpProcessor::from_file("nonexistent.vtp");
        assert!(result.is_err());

        let test_file = create_test_vtp_file();
        let reader = VtpProcessor::from_file(&test_file).unwrap();
        let result = reader.field("nonexistent_field");
        assert!(result.is_err());

        cleanup_test_file(&test_file);
    }
}
