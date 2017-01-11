extern crate clap;

use clap::{App, Arg};

fn main() {
    App::new("godaddns")
        .version("1.0.0")
        .author("Fernando Paredes <nano@fdp.io>")
        .about("Update GoDaddy records to current IP")
        .arg(Arg::with_name("DOMAIN")
                .short("d")
                .long("domain")
                .required(true)
                .help("Domain to update"))
        .arg(Arg::with_name("RECORD_TYPE")
                .required(true)
                .short("r")
                .long("record")
                .help("Record type to set entry to"))
        .get_matches();

}
