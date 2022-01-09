use clap::Parser;

#[derive(Parser)]
#[clap(about)]
struct Cli {
    #[clap(short='i', long)]
    input: String,

    #[clap(long="ibase")]
    input_base: String,

    #[clap(long="obase")]
    output_base: String,
}

fn main() {
    let cli = Cli::parse();

    println!("input={}", cli.input);
    println!("input_base={}", cli.input_base);
    println!("output_base={}", cli.output_base);
}
