extern crate libflate;

use libflate::gzip::Decoder;
use pisa_jr::parser;
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(help = "Collection file in TREC format")]
    input: PathBuf,
}

#[paw::main]
fn main(opt: Opt) -> Result<(), io::Error> {
    let mut entries = fs::read_dir(opt.input)?
        .map(|res| res.map(|e| e.path()))
        .map(|res| fs::File::open(res.unwrap()).expect("TODO"))
        .map(|res| Decoder::new(res).unwrap())
        .map(|res| io::BufReader::new(res))
        .map(|res| parser::split_trec_records(res))
        .map(|res| res.map(|bytes| String::from_utf8(bytes.unwrap())));

    let records: Result<Vec<_>, _> = entries.next().unwrap().collect();
    println!("{:?}", records.unwrap());
    Ok(())
}
