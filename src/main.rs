use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }

    let mut f = File::open(&args[1])?;
    let metadata = f.metadata()?;
    println!("{:?}", metadata.file_type());
    hnd::print_header(&mut f);

    Ok(())
}
