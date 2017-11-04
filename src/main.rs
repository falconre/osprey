extern crate clap;
extern crate osprey;

use std::fs::File;
use std::io::Read;


fn osprey () {
    let matches = clap::App::new("osprey")
        .version("0.1.0")
        .about("Program Analysis with Falcon")
        .author("Alex Eubanks")
        .arg(clap::Arg::with_name("script")
             .short("s")
             .long("script")
             .value_name("FILE")
             .help("Path to script to run")
             .required(true))
        .get_matches();

    let mut fh = File::open(matches.value_of("script").unwrap()).unwrap();
    let mut script = String::new();
    fh.read_to_string(&mut script).unwrap();

    let _ = osprey::run_code(&script);
}


fn main () {
    osprey();
}