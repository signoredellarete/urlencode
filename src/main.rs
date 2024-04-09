use clap::Parser;
use clap_stdin::MaybeStdin;
use url::form_urlencoded::{byte_serialize, parse};

#[derive(Debug, Parser)]
struct Args {
    //#[clap(default_value = "-")]
    value: MaybeStdin<String>,
}

fn main() {
    let args = Args::parse();
    let urlencoded: String = byte_serialize(args.value.as_bytes()).collect();
    println!("{}", urlencoded);
}






