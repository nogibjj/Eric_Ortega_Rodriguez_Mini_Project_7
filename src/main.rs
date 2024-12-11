use clap::Parser;
/// A basic example of using clap
#[derive(Parser)]
#[command(name = "basic_cli")]
#[command(about = "A basic CLI example using clap", long_about = None)]
struct Cli {
    /// Name of the person to greet
    name: String,
}
fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);
}