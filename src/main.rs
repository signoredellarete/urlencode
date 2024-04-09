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
    // println!("value={}", args.value);
    //let joined = args.value;
    let urlencoded: String = byte_serialize(args.value.as_bytes()).collect();
    println!("{}", urlencoded);
}

// fn main() {
//     let urlencoded: String = byte_serialize("$$".as_bytes()).collect();
//     //assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
//     println!("urlencoded:'{}'", urlencoded);

//     let decoded: String = parse(urlencoded.as_bytes())
//         .map(|(key, val)| [key, val].concat())
//         .collect();
//     //assert_eq!(decoded, "What is ❤?");
//     println!("decoded:'{}'", decoded);
// }


// fn main() {
//     let args: Vec<String> = env::args().skip(1).collect();

//     if args.len() == 0 {
//         //println!("");
//     } else {
//         let joined = args.join(" ");
//         let encoded = encode(&joined);
//         // println!("{}", args.join(" "));
//         println!("{}", encoded);
//     }
// }






