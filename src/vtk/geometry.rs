use std::error::Error;

use vtkio::model::IOBuffer;
use vtkio::Vtk;

use super::reader::get_poly_data;

pub type GeometryResult = Result<(Vec<f64>, Vec<usize>, Vec<usize>), Box<dyn Error>>;

pub struct GeometryExtractor;

impl GeometryExtractor {
    pub fn extract_geometry(vtk: &Vtk) -> GeometryResult {
        let poly_data = get_poly_data(vtk)?;

        let points = Self::extract_points(&poly_data.points)?;
        let (connectivity, offsets) = Self::extract_connectivity(poly_data);

        Ok((points, connectivity, offsets))
    }

    fn extract_points(points_buffer: &IOBuffer) -> Result<Vec<f64>, Box<dyn Error>> {
        match points_buffer {
            IOBuffer::F32(data) => Ok(data.iter().map(|&x| x as f64).collect()),
            IOBuffer::F64(data) => Ok(data.clone()),
            _ => Err("Unsupported point data format".into()),
        }
    }

    fn extract_connectivity(poly_data: &vtkio::model::PolyDataPiece) -> (Vec<usize>, Vec<usize>) {
        poly_data
            .polys
            .as_ref()
            .map(|polys| {
                let (conn_data, offs_data) = polys.clone().into_xml();
                (
                    conn_data.iter().map(|&i| i as usize).collect(),
                    offs_data.iter().map(|&i| i as usize).collect(),
                )
            })
            .unwrap_or_default()
    }
}
