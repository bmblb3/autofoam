use clap::Parser;

#[derive(Parser)]
#[command(about = "Prints the bbox of input stl file(s)")]
pub struct Args {
    #[arg(help = "Path(s) to .vtp file(s)", required = true, value_hint = clap::ValueHint::FilePath)]
    pub files: Vec<String>,
}
