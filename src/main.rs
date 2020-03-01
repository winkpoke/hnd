use std::convert::{From, Into, TryFrom, TryInto};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::mem;
use std::str::FromStr;
use clap::clap_app;
use hnd;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(hnd =>
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
        (@subcommand conv =>
            (about: "Convert HND to RAW.")
            (@arg input: +required "Sets the input file")
            (@arg output: +required "Sets the output file"))
        (@subcommand raw =>
            (about: "Create HND from RAW.")
            (@arg input: +required "Sets the input file")
            (@arg output: +required "Sets the output file")
            (@arg width: -w --width <INT> +required +takes_value "Width of the image")
            (@arg height: -h --height <INT> +required +takes_value "Height of the image")
            (@arg x_res: --x_res [DOUBLE] +takes_value "X resolution")
            (@arg y_res: --y_res [DOUBLE] +takes_value "Y resolution")
            (@arg angle: -a --angle [DOUBLE] +takes_value "Projection angle in degree")
            (@arg n_bytes: -b --bytes <SHORT> +takes_value "Bytes per pixel")
            //(@arg n_images: -n <INT> -required +takes_value "number of images in the input file")
        )
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
    } else if let Some(matches) = matches.subcommand_matches("conv") {
        let input = matches.value_of("input").unwrap();
        let mut fin = File::open(input)?;

        let output = matches.value_of("output").unwrap();
        let mut fout = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(output)?;

        hnd::convert_to_raw(&mut fin, &mut fout)?;
    } else if let Some(matches) = matches.subcommand_matches("raw") {
        let arg_usize = |x: &str| usize::from_str_radix(matches.value_of(x).unwrap(), 10).unwrap();

        let arg_u32 = |x| u32::from_str_radix(matches.value_of(x).unwrap(), 10).unwrap();
        let arg_i64 =
            |x: &str| -> i64 { i64::from_str_radix(matches.value_of(x).unwrap(), 10).unwrap() };
        let arg_f64 = |x| f64::from_str(matches.value_of(x).unwrap()).unwrap();

        let input = matches.value_of("input").unwrap();
        let mut fin = File::open(input)?;
        let metadata = fin.metadata()?;
        let output = matches.value_of("output").unwrap();
        let mut fout = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(output)?;

        let mut hnd_header = hnd::hnd_header_t::new();
        let width = arg_usize("width");
        let height = arg_usize("height");
        hnd_header.SizeX = width as u32;
        hnd_header.SizeY = height as u32;
        if matches.is_present("x_res") {
            let x_res = arg_f64("x_res");
            println!("Read in x_res: {} ...OK", x_res);
            hnd_header.dImageResolutionX = x_res;
            hnd_header.dIDUResolutionX = x_res;
        }
        if matches.is_present("y_res") {
            let y_res = arg_f64("y_res");
            println!("Read in y_res: {} ...OK", y_res);
            hnd_header.dImageResolutionY = y_res;
            hnd_header.dIDUResolutionY = y_res;
        }
        if matches.is_present("angle") {
            let angle = arg_f64("angle");
            println!("Read in angle: {} ...OK", angle);
            hnd_header.dCTProjectionAngle = angle;
        }
        let n_bytes: u32 = arg_u32("n_bytes");
        // let n_images = 
        //     if matches.is_present("n_images") {
        //         arg_u32("n_images")
        //     } else {
        //         1
        //     };
        // println!("Read in n_images: {} ...OK", n_images);
        // let input_file_size = width * height * n_bytes as usize * n_images as usize;
        // if input_file_size != metadata.len() as usize {
        //     panic!("fatal error: width * height * nbytes * n_images is not equal file size.");
        // }

        let mut buf: Vec<u8> = Vec::new();
        fin.read_to_end(&mut buf)?;
        match n_bytes {
            2 => {
                buf.shrink_to_fit();
                let len = buf.len() / 2;
                let raw_image = unsafe{ Vec::<u16>::from_raw_parts(buf.as_mut_ptr() as *mut u16, len, len) };
                std::mem::forget(buf);
                let hnd_data = hnd::encode_u16(&raw_image, width, height).unwrap();
                fout.write(&hnd_header.to_raw())?;
                fout.write(&hnd_data)?;
            }
            4 => {
                let mut raw_image: Vec<u32> = unsafe { std::mem::transmute(buf) };
                let hnd_data = hnd::encode_u32(&raw_image, width, height).unwrap();
                fout.write(&hnd_header.to_raw())?;
                fout.write(&hnd_data)?;
            }
            _ => {
                panic!("shouldn't be here.");
            }
        }
    }

    Ok(())
}
