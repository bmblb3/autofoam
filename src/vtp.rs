pub mod polygon_areas;
pub use polygon_areas::calculate_polygon_areas;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use vtkio::model::{Attribute, DataArray, DataSet, ElementType, IOBuffer, Piece, PolyDataPiece};
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

    pub fn geometry(&self) -> Result<(Vec<f64>, Vec<usize>, Vec<usize>), Box<dyn Error>> {
        let poly_data = get_poly_data(&self.vtk)?;

        let points = match &poly_data.points {
            IOBuffer::F32(data) => data.iter().map(|&x| x as f64).collect(),
            IOBuffer::F64(data) => data.clone(),
            _ => return Err("Unsupported point data format".into()),
        };

        let (connectivity, offsets) = poly_data
            .polys
            .as_ref()
            .map(|polys| {
                let (conn_data, offs_data) = polys.clone().into_xml();
                (
                    conn_data.iter().map(|&i| i as usize).collect(),
                    offs_data.iter().map(|&i| i as usize).collect(),
                )
            })
            .unwrap_or_default();

        Ok((points, connectivity, offsets))
    }

    pub fn field(&self, field_name: &str) -> Result<Vec<f64>, Box<dyn Error>> {
        let poly_data = get_poly_data(&self.vtk)?;

        let field_data = poly_data
            .data
            .cell
            .iter()
            .find_map(|attr| match attr {
                Attribute::DataArray(arr) if arr.name == field_name => Some(&arr.data),
                _ => None,
            })
            .ok_or_else(|| format!("Field '{}' not found", field_name))?;

        match field_data {
            IOBuffer::F64(data) => Ok(data.clone()),
            IOBuffer::F32(data) => Ok(data.iter().map(|&x| x as f64).collect()),
            _ => Err(format!("Unsupported data type for field '{}'", field_name).into()),
        }
    }

    #[allow(dead_code)]
    pub fn list_fields(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let poly_data = get_poly_data(&self.vtk)?;
        Ok(poly_data
            .data
            .cell
            .iter()
            .filter_map(|attr| match attr {
                Attribute::DataArray(arr) => Some(arr.name.clone()),
                _ => None,
            })
            .collect())
    }

    pub fn field_exists(&self, field_name: &str) -> Result<bool, Box<dyn Error>> {
        let poly_data = get_poly_data(&self.vtk)?;
        Ok(poly_data
            .data
            .cell
            .iter()
            .any(|attr| matches!(attr, Attribute::DataArray(arr) if arr.name == field_name)))
    }

    pub fn remove_field(mut self, field_name: &str) -> Result<Self, Box<dyn Error>> {
        if self.field_exists(field_name)? {
            let poly_data = get_poly_data_mut(&mut self.vtk)?;
            if let Some(pos) = poly_data.data.cell.iter().position(
                |attr| matches!(attr, Attribute::DataArray(arr) if arr.name == field_name),
            ) {
                poly_data.data.cell.remove(pos);
            }
        }
        Ok(self)
    }

    pub fn add_field(mut self, field_name: &str, data: &[f64]) -> Result<Self, Box<dyn Error>> {
        if self.field_exists(field_name)? {
            return Err(format!("Field '{}' already exists", field_name).into());
        }

        let poly_data = get_poly_data_mut(&mut self.vtk)?;
        let data_array = DataArray {
            name: field_name.to_string(),
            elem: ElementType::Scalars {
                num_comp: 1,
                lookup_table: None,
            },
            data: IOBuffer::F64(data.to_vec()),
        };

        poly_data.data.cell.push(Attribute::DataArray(data_array));
        Ok(self)
    }

    pub fn write_to_file(self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        self.vtk.write_xml(&mut file)?;
        Ok(())
    }
}

fn get_poly_data(vtk: &Vtk) -> Result<&PolyDataPiece, Box<dyn Error>> {
    let piece = match &vtk.data {
        DataSet::PolyData { pieces, .. } => pieces.iter().next().ok_or("No pieces found")?,
        _ => return Err("Expected PolyData".into()),
    };

    match piece {
        Piece::Inline(data) => Ok(data),
        _ => Err("Expected inline piece".into()),
    }
}

fn get_poly_data_mut(vtk: &mut Vtk) -> Result<&mut PolyDataPiece, Box<dyn Error>> {
    let piece = match &mut vtk.data {
        DataSet::PolyData { pieces, .. } => pieces.iter_mut().next().ok_or("No pieces found")?,
        _ => return Err("Expected PolyData".into()),
    };

    match piece {
        Piece::Inline(data) => Ok(data),
        _ => Err("Expected inline piece".into()),
    }
}
