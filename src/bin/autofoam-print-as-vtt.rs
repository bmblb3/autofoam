use clap::Parser;

#[derive(Parser)]
#[command(about = "Prints a value in vtt table format")]
pub struct Args {
    #[arg(help = "value to wrap", required = true)]
    pub value: f32,
}
fn main() {
    let args = Args::parse();

    let value = &args.value;

    println!(
        r#"<VTKFile type="Table" version="2.2" byte_order="LittleEndian" header_type="UInt64">
  <Table>
    <Piece NumberOfCols="1" NumberOfRows="1">
      <RowData>
        <Array type="Float32" Name="value" format="ascii">
          {value}
        </Array>
      </RowData>
    </Piece>
  </Table>
</VTKFile>"#
    );
}
