use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(help = "Collection file in TREC format")]
    input: PathBuf,
}

#[paw::main]
fn main(_opt: Opt) {
    unimplemented!()
}
