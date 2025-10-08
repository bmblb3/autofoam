use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use vtkio::model::DataSet;
use vtkio::model::Piece;
use vtkio::model::PolyDataPiece;
use vtkio::Vtk;

pub struct VtkReader {
    vtk: Vtk,
}

impl VtkReader {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let vtk = Vtk::parse_xml(reader)?;
        Ok(VtkReader { vtk })
    }

    pub fn vtk(&self) -> &Vtk {
        &self.vtk
    }

    pub fn vtk_mut(&mut self) -> &mut Vtk {
        &mut self.vtk
    }

    pub fn write_to_file(self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        self.vtk.write_xml(&mut file)?;
        Ok(())
    }
}

pub fn get_poly_data(vtk: &Vtk) -> Result<&PolyDataPiece, Box<dyn Error>> {
    let piece = match &vtk.data {
        DataSet::PolyData { pieces, .. } => pieces.iter().next().ok_or("No pieces found")?,
        _ => return Err("Expected PolyData".into()),
    };

    match piece {
        Piece::Inline(data) => Ok(data),
        _ => Err("Expected inline piece".into()),
    }
}

pub fn get_poly_data_mut(vtk: &mut Vtk) -> Result<&mut PolyDataPiece, Box<dyn Error>> {
    let piece = match &mut vtk.data {
        DataSet::PolyData { pieces, .. } => pieces.iter_mut().next().ok_or("No pieces found")?,
        _ => return Err("Expected PolyData".into()),
    };

    match piece {
        Piece::Inline(data) => Ok(data),
        _ => Err("Expected inline piece".into()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_vtk_reader_creation() {
        let test_data = r#"<?xml version="1.0"?>
<VTKFile type="PolyData" version="1.0">
  <PolyData>
    <Piece NumberOfPoints="3" NumberOfPolys="1">
      <Points>
        <DataArray type="Float64" NumberOfComponents="3" format="ascii">
          0 0 0 1 1 0 0 1 1
        </DataArray>
      </Points>
      <Polys>
        <DataArray type="Int32" Name="connectivity" format="ascii">
          0 1 2
        </DataArray>
        <DataArray type="Int32" Name="offsets" format="ascii">
          3
        </DataArray>
      </Polys>
    </Piece>
  </PolyData>
</VTKFile>"#;

        fs::write("test.vtp", test_data).unwrap();

        let reader = VtkReader::from_file("test.vtp");
        assert!(reader.is_ok());

        fs::remove_file("test.vtp").unwrap();
    }
}
