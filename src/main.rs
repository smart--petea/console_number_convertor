use clap::Parser;

use conv;

#[derive(Parser)]
#[clap(about)]
struct Cli {
    #[clap(short='i', long)]
    input: String,

    #[clap(long="ibase")]
    input_base: u32,

    #[clap(long="obase")]
    output_base: u32,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let result = conv::convert_base(cli.input, cli.input_base, cli.output_base);

    match result {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(message) => Err(message)
    }
}
