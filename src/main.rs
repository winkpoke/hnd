use std::env;
use std::error::Error;
use std::fs::File;
// use std::io::{BufReader, Read};

#[macro_use]
extern crate clap;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Phil C. <chen.weihai@gmail.com>")
        (about: "Handle Varian .HND files.")
        //(@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        //(@arg INPUT: +required "Sets the input file to use")
        //(@arg debug: -d ... "Sets the level of debugging information")
        // (@subcommand test =>
        //     (about: "controls testing features")
        //     (@arg verbose: -v --verbose "Print test information verbosely")
        // )
        (@subcommand show =>
            (about: "print out header information.")
            (@arg filename: +required "Sets the input file"))
        (@subcommand convert =>
            (about: "print out header information.")
            (@arg input: +required "Sets the input file")
            (@arg output: +required "Sets the output file"))
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("show") {
        let filename = matches.value_of("filename").unwrap();
        let mut f = File::open(filename)?;
        let metadata = f.metadata()?;
        println!("{:?}", metadata.file_type());
        hnd::print_header(&mut f)?;
    } else if let Some(matches) = matches.subcommand_matches("test") {
        println!("handling test subcommand!");
    } else if let Some(matches) = matches.subcommand_matches("convert") {
        let input = matches.value_of("input").unwrap();
        let mut fin = File::open(input)?;
        let output = matches.value_of("output").unwrap();
        let mut fout = File::open(output)?;
        hnd::convert_to_raw(&mut fin, &mut fout)?;
    }

    // let args: Vec<_> = env::args().collect();
    // if args.len() > 1 {
    //     println!("The first argument is {}", args[1]);
    // }

    Ok(())
}
