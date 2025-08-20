pub mod polygon_areas;
pub use polygon_areas::calculate_polygon_areas;

pub mod field_manager;
pub mod geometry;
pub mod reader;

use field_manager::FieldManager;
use geometry::GeometryExtractor;
use reader::VtkReader;
use std::error::Error;

pub struct VtpProcessor {
    reader: VtkReader,
}

impl VtpProcessor {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let reader = VtkReader::from_file(path)?;
        Ok(VtpProcessor { reader })
    }

    pub fn geometry(&self) -> Result<(Vec<f64>, Vec<usize>, Vec<usize>), Box<dyn Error>> {
        GeometryExtractor::extract_geometry(self.reader.vtk())
    }

    pub fn field(&self, field_name: &str) -> Result<Vec<f64>, Box<dyn Error>> {
        FieldManager::get_field(self.reader.vtk(), field_name)
    }

    pub fn list_fields(&self) -> Result<Vec<String>, Box<dyn Error>> {
        FieldManager::list_fields(self.reader.vtk())
    }

    pub fn field_exists(&self, field_name: &str) -> Result<bool, Box<dyn Error>> {
        FieldManager::field_exists(self.reader.vtk(), field_name)
    }

    pub fn remove_field(mut self, field_name: &str) -> Result<Self, Box<dyn Error>> {
        FieldManager::remove_field(self.reader.vtk_mut(), field_name)?;
        Ok(self)
    }

    pub fn add_field(mut self, field_name: &str, data: &[f64]) -> Result<Self, Box<dyn Error>> {
        FieldManager::add_field(self.reader.vtk_mut(), field_name, data)?;
        Ok(self)
    }

    pub fn write_to_file(self, path: &str) -> Result<(), Box<dyn Error>> {
        self.reader.write_to_file(path)
    }
}
