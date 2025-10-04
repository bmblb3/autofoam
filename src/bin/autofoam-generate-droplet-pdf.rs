use autofoam::rosin_rammler::RosinRammler;
use clap::Parser;

#[derive(Parser)]
#[command(about = "Computes and prints droplet size PDF given SMD and Dv90")]
pub struct Args {
    #[arg(long, help = "SMD [μm]")]
    pub d32: f64,

    #[arg(long, help = "Dv90 [μm]")]
    pub dv90: f64,
}

fn main() {
    let args = Args::parse();
    let smd = args.d32 * 1e-6;
    let dv90 = args.dv90 * 1e-6;

    let dist = RosinRammler::from_smd_and_dv90(smd, dv90).unwrap();
    println!(
        "// desired_d32: {}, desired_dv90: {}, calculated_d32: {}",
        smd,
        dv90,
        dist.smd()
    );
    for i in 0..100 {
        let x = (1.5 * dv90 / 100.0) * i as f64 + 1e-9;
        let p_x = dist.cdf(x);
        println!("( {:.4} {:.4} )", x, p_x);
    }
}
