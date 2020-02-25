use libflate::gzip::Decoder;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use structopt::StructOpt;
use trec_text::Parser;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(help = "Collection files in TREC format")]
    input: Vec<PathBuf>,
    #[structopt(short, long, help = "Use GZIP to decode input")]
    zip: bool,
}

fn main() {
    let empty: Box<dyn Read> = Box::new(std::io::empty());
    let args = Opt::from_args();
    let input = args
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
    let parser = if args.zip {
        let input: Box<dyn Read> =
            Box::new(Decoder::new(input).expect("Failed to init gzip decoder"));
        Parser::new(input)
    } else {
        Parser::new(input)
    };
    for document in parser {
        if let Ok(doc) = document {
            println!(
                "{}: {}",
                String::from_utf8_lossy(doc.docno()),
                String::from_utf8_lossy(doc.content())
            );
        }
    }
}
