use std::error::Error;

use vtkio::model::Attribute;
use vtkio::model::DataArray;
use vtkio::model::ElementType;
use vtkio::model::IOBuffer;
use vtkio::Vtk;

use super::reader::get_poly_data;
use super::reader::get_poly_data_mut;

pub struct FieldManager;

impl FieldManager {
    pub fn get_field(vtk: &Vtk, field_name: &str) -> Result<Vec<f64>, Box<dyn Error>> {
        let poly_data = get_poly_data(vtk)?;

        let field_data = poly_data
            .data
            .cell
            .iter()
            .find_map(|attr| match attr {
                Attribute::DataArray(arr) if arr.name == field_name => Some(&arr.data),
                _ => None,
            })
            .ok_or_else(|| format!("Field '{}' not found", field_name))?;

        Self::convert_to_f64(field_data, field_name)
    }

    pub fn list_fields(vtk: &Vtk) -> Result<Vec<String>, Box<dyn Error>> {
        let poly_data = get_poly_data(vtk)?;
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

    pub fn field_exists(vtk: &Vtk, field_name: &str) -> Result<bool, Box<dyn Error>> {
        let poly_data = get_poly_data(vtk)?;
        Ok(poly_data
            .data
            .cell
            .iter()
            .any(|attr| matches!(attr, Attribute::DataArray(arr) if arr.name == field_name)))
    }

    pub fn remove_field(vtk: &mut Vtk, field_name: &str) -> Result<(), Box<dyn Error>> {
        let poly_data = get_poly_data_mut(vtk)?;
        if let Some(pos) =
            poly_data.data.cell.iter().position(
                |attr| matches!(attr, Attribute::DataArray(arr) if arr.name == field_name),
            )
        {
            poly_data.data.cell.remove(pos);
        }
        Ok(())
    }

    pub fn add_field(vtk: &mut Vtk, field_name: &str, data: &[f64]) -> Result<(), Box<dyn Error>> {
        if Self::field_exists(vtk, field_name)? {
            return Err(format!("Field '{}' already exists", field_name).into());
        }

        let poly_data = get_poly_data_mut(vtk)?;
        let data_array = DataArray {
            name: field_name.to_string(),
            elem: ElementType::Scalars {
                num_comp: 1,
                lookup_table: None,
            },
            data: IOBuffer::F64(data.to_vec()),
        };

        poly_data.data.cell.push(Attribute::DataArray(data_array));
        Ok(())
    }

    fn convert_to_f64(buffer: &IOBuffer, field_name: &str) -> Result<Vec<f64>, Box<dyn Error>> {
        match buffer {
            IOBuffer::F64(data) => Ok(data.clone()),
            IOBuffer::F32(data) => Ok(data.iter().map(|&x| x as f64).collect()),
            _ => Err(format!("Unsupported data type for field '{}'", field_name).into()),
        }
    }
}
