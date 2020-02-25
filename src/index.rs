use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use structopt::StructOpt;
use trec_text::Parser;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(help = "Collection files in TREC format")]
    input: Vec<PathBuf>,
}

fn main() {
    let empty: Box<dyn Read> = Box::new(std::io::empty());
    let input = Opt::from_args()
        .input
        .into_iter()
        .map(|path| File::open(path).map(BufReader::new))
        .fold(empty, |acc, f| match f {
            Ok(f) => Box::new(acc.chain(f)),
            Err(err) => {
                eprintln!("Unable to read input file: {}. Skipping...", err);
                Box::new(std::io::empty())
            }
        });
    for document in Parser::new(input) {
        if let Ok(doc) = document {
            println!(
                "{}: {}",
                String::from_utf8_lossy(doc.docno()),
                String::from_utf8_lossy(doc.content())
            );
        }
    }
}
