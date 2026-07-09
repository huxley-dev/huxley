use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct HuxleyCli {
    // Comma delimited list of roles the Huxley Process will run
    #[arg(short, long)]
    role: String,
}
