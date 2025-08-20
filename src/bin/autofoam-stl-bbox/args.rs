use clap::Parser;

#[derive(Parser)]
#[command(about = "Prints the bbox of input stl file(s)")]
pub struct Args {
    #[arg(help = "Path(s) to .vtp file(s)", required = true)]
    pub files: Vec<String>,
}
