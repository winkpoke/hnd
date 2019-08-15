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

    //let mut reader = BufReader::new(f);
    //let mut buf: [u8; 1024] = [0; 1024];
    //let n = reader.read(&mut buf[..10]).unwrap();
    //for c in &buf[..n] {
    //    println!("{:?} ", c);
    //}

    use hnd;
    hnd::read_header(&mut f);

    Ok(())
}
