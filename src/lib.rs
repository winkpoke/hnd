#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into, TryFrom, TryInto};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::mem;

mod modal;
mod control;

pub use modal::hnd_header_t;
pub use modal::ImageConvError;
pub use modal::decode;
pub use modal::{encode_u16, encode_u32};

type hnd_data_t = Vec<u8>;

pub struct HndImage {
    header: hnd_header_t,
    data: hnd_data_t,
}

pub struct RawImage<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub trait Size2D {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl Size2D for HndImage {
    fn width(&self) -> usize {
        self.header.SizeX as usize
    }
    fn height(&self) -> usize {
        self.header.SizeY as usize
    }
}

impl<T> Size2D for RawImage<T> {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl Size2D for hnd_header_t {
    fn width(&self) -> usize {
        self.SizeX as usize
    }
    fn height(&self) -> usize {
        self.SizeY as usize
    }
}

impl TryInto<RawImage<u32>> for HndImage {
    type Error = ImageConvError;
    fn try_into(self) -> Result<RawImage<u32>, Self::Error> {
        let width = self.width();
        let height = self.height();
        let data = decode(&self.data, width, height)?;

        Ok(RawImage {
            width: width,
            height: height,
            data,
        })
    }
}
// impl HndEncode for RawImage<u32> {}

impl TryInto<HndImage> for RawImage<u32> {
    type Error = ImageConvError;
    fn try_into(self) -> Result<HndImage, Self::Error> {
        Err(ImageConvError::SomeErr)
    }
}

impl TryInto<HndImage> for RawImage<u16> {
    type Error = ImageConvError;
    fn try_into(self) -> Result<HndImage, Self::Error> {
        Err(ImageConvError::SomeErr)
    }
}

//fn from_raw(img: &[u8], width: u32, height: u32) -> Result<Box> {}
//
pub fn print_header(f: &mut File) -> Result<(), io::Error> {
    let raw = read_header_to_raw(f)?;
    //let hnd_head = parse_header(&raw)?;
    let hnd_head: hnd_header_t = hnd_header_t::from_raw(raw);

    //println!("DEBUG: {:?}", hnd_head);
    println!("{}", hnd_head);

    Ok(())
}

pub fn read_header(f: &mut File) -> Result<hnd_header_t, io::Error> {
    let raw = read_header_to_raw(f)?;
    let hnd_head: hnd_header_t = hnd_header_t::from_raw(raw);
    //println!("DEBUG: {:?}", hnd_head);

    Ok(hnd_head)
}

fn write_header(f: &mut File, h: &hnd_header_t) -> Result<(), io::Error> {
    f.write(h.sFileType.as_ref())?;
    Ok(())
}

fn read_data(f: &mut File) -> Result<hnd_data_t, io::Error> {
    let header: hnd_header_t = read_header(f)?;

    let w = header.SizeX;
    let h = header.SizeY;
    let len = w * h;
    let mut buf = Vec::new();

    // Skip HND header
    let n = f.seek(SeekFrom::Start(1024));
    let s = f.read_to_end(&mut buf)?;

    Ok(buf)
}

fn read_header_to_raw(f: &File) -> Result<modal::hnd_header_buf_t, io::Error> {
    let mut reader = BufReader::new(f);
    let mut buf: modal::hnd_header_buf_t = Vec::with_capacity(1024); 
    buf.resize(1024, 0);
    let n: usize = reader.read(&mut buf[..1024])?;
    println!("DEBUG: read in {} bytes in total.", n);
    return Ok(buf);
}

pub fn read_file(f: &mut File) -> Result<HndImage, io::Error> {
    Ok(HndImage {
        header: read_header(f).unwrap(),
        data: read_data(f).unwrap(),
    })
}

pub fn write_file(f: &mut File, hnd: &HndImage) -> Result<(), io::Error> {
    let raw_header: modal::hnd_header_buf_t = hnd.header.to_raw();
    f.write(&raw_header)?;
    f.write(hnd.data.as_ref())?;
    Ok(())
}

pub fn convert_to_raw(fin: &mut File, fout: &mut File) -> Result<(), io::Error> {
    let mut hnd_header_raw: modal::hnd_header_buf_t = Vec::with_capacity(1024);
    hnd_header_raw.resize(1024, 0);
    fin.read(&mut hnd_header_raw[..1024])?;

    let mut hnd_data_buf: Vec<u8> = Vec::new();
    fin.read_to_end(&mut hnd_data_buf)?;

    let hnd_header = hnd_header_t::from_raw(hnd_header_raw);
    let height = hnd_header.height();
    let width = hnd_header.width();

    // let hnd_data: Vec<u32> = unsafe { std::mem::transmute(hnd_data_buf) };
    let mut raw_image = decode(&hnd_data_buf, width, height).unwrap();
    // print!("{} ", raw_image.len());
    let raw_image_buf: Vec<u8> = unsafe {
        Vec::from_raw_parts(
            raw_image.as_mut_ptr() as *mut u8,
            width * height * 4,
            width * height * 4,
        )
    };
    std::mem::forget(raw_image);
    // let raw_image_buf: Vec<u8> = unsafe { std::mem::transmute(raw_image) };
    println!("{}", raw_image_buf.len());

    fout.write(raw_image_buf.as_slice())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_read_header() {
        let test_file_1 = String::from("test/test_data_1.hnd");
        let mut f = std::fs::File::open(test_file_1).unwrap();
        let header = crate::read_header(&mut f).unwrap();
        assert_eq!(header.sFileType, "VARIAN_VA_INTERNAL_HND_1.0");
        assert_eq!(header.sCreationDate, "20190610");
        assert_eq!(header.FileLength, 3146752);
        assert_eq!(header.SizeX, 1024);
        assert_eq!(header.SizeY, 768);
        assert_eq!(header.dCTProjectionAngle, -71.01111111111112);
        assert_eq!(header.dCTNormChamber, 1164.0);
    }

    #[test]
    fn test_write_raw() {
        use std::convert::TryInto;
        use std::io::{BufReader, Read, Seek, SeekFrom};

        // test hnd file
        let test_file_1 = String::from("test/test_data_1.hnd");
        let mut f_test = std::fs::File::open(test_file_1).unwrap();
        let test_data = crate::read_data(&mut f_test).unwrap();

        // raw file to compare with
        let raw_file_1 = String::from("test/test_data_1.raw");
        let mut f_raw = std::fs::File::open(raw_file_1).unwrap();

        // let mut f_out = tempfile::tempfile().unwrap();

        // Read in the raw data
        let mut raw = Vec::new();
        f_raw.read_to_end(&mut raw).unwrap();
        for i in 0..100 {
            println!(
                "raw {} {}",
                i,
                u32::from_ne_bytes(raw[i * 4..i * 4 + 4].try_into().unwrap())
            );
        }

        // parse the hnd data
        let parsed = crate::decode(&test_data, 1024, 768).unwrap();

        //compare the results
        for i in 0..1024 * 768 {
            let x = parsed[i];
            let y = u32::from_ne_bytes(raw[i * 4..i * 4 + 4].try_into().unwrap());
            assert_eq!(x, y);
            print!(".");
        }
    }

    #[test]
    fn test_read_raw_32() {
        // use std::convert::TryInto;
        use std::io::Read;
        // use std::io::{BufReader, Read, Seek, SeekFrom};

        let width = 1024;
        let height = 768;

        // test hnd file
        let test_file_1 = String::from("test/test_data_1.hnd");
        let mut f_test = std::fs::File::open(test_file_1).unwrap();
        let test_data_hnd = crate::read_data(&mut f_test).unwrap();

        // raw file to compare with
        let raw_file_1 = String::from("test/test_data_1.raw");
        let mut f_raw = std::fs::File::open(raw_file_1).unwrap();

        // Read in the raw data
        // let mut raw_vec_u32: Vec<u32> = Vec::with_capacity(1024 * 768);
        // let mut buf: [u8; 4] = [0; 4];
        // while f_raw.read(&mut buf).unwrap() != 0 {
        //     let v = u32::from_ne_bytes(buf[..].try_into().unwrap());
        //     raw_vec_u32.push(v);
        // }
        let raw_vec_u32: Vec<u32>;
        let mut buf: Vec<u8> = Vec::new();
        f_raw.read_to_end(&mut buf).unwrap();
        unsafe {
            raw_vec_u32 = std::mem::transmute(buf);
        }

        // endcode the raw 32 bits data
        let encoded: crate::hnd_data_t = crate::encode_u32(&raw_vec_u32, 1024, 768).unwrap();

        // parse the hnd data
        let parsed = crate::decode(&test_data_hnd, width, height).unwrap();

        // compare the len of the compressed data
        assert_eq!(encoded.len(), test_data_hnd.len());

        println!(
            "Orignial size = {}, encoded size = {}",
            test_data_hnd.len(),
            encoded.len()
        );

        // compare LUT
        let lut_size: usize = (height - 1) * width / 4;

        for i in 0..lut_size {
            assert_eq!(encoded[i], test_data_hnd[i]);
            println!("{} {}", encoded[i], test_data_hnd[i]);
        }

        // compare the data
        for i in lut_size..encoded.len() {
            assert_eq!(encoded[i], test_data_hnd[i]);
        }
    }

    #[test]
    fn test_read_raw_16() {}

    #[test]
    fn test_header_convertion() {
        use crate::*;
        let test_file_1 = String::from("test/test_data_1.hnd");
        let mut f = std::fs::File::open(test_file_1).unwrap();
        let header: hnd_header_t = crate::read_header(&mut f).unwrap();
        let raw_header_buf: modal::hnd_header_buf_t = header.to_raw();
        let raw_header_buf_clone = raw_header_buf.clone();
        let header2: hnd_header_t = hnd_header_t::from_raw(raw_header_buf);

        println!("{:?}", header);
        println!("{:?}", header2);
        raw_header_buf_clone
            .iter()
            .for_each(|item| print!("{:2X} ", item));
        println!();
        println!("{:?} --- {:?}", header.sFileType, header2.sFileType);
        assert_eq!(header.sFileType, header2.sFileType);
        assert_eq!(header.sCreationDate, header2.sCreationDate);
        assert_eq!(header2.FileLength, 3146752);
        assert_eq!(header2.SizeX, 1024);
        assert_eq!(header2.SizeY, 768);
        assert_eq!(header2.dCTProjectionAngle, -71.01111111111112);
        assert_eq!(header2.dCTNormChamber, 1164.0);
    }
}

// #[repr(C)]
// pub struct image_u16_t{_private: [u8; 0]}

// extern {
//     fn image_u16_get_width(image: *const image_u16_t) -> usize;
//     fn image_u16_get_height(image: *const image_u16_t) -> usize;
// }

// export functions 
#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 {
    a + b
}


// #[no_mangle]
// pub extern "C" fn func(image: *const image_u16_t) -> usize {
//     unsafe {image_u16_get_height(image)}
// }


#[no_mangle]
pub extern "C" fn encode(img: *mut u8, width: usize, height: usize, bytes_per_pixel: usize, size: *mut usize) -> *mut u8 {
    let len = width * height;
    unsafe {
        match bytes_per_pixel {
            2 => {
                let v = Vec::from_raw_parts(img as *mut u16, len, len);
                let mut data = encode_u16(&v, width, height).unwrap();
                data.shrink_to_fit();
                *size = data.len();
                let raw_ptr = data.as_mut_ptr();
                std::mem::forget(v);
                std::mem::forget(data);
                return raw_ptr;
            }
            4 => {
                let v = Vec::from_raw_parts(img as *mut u32, len, len);
                let mut data = encode_u32(&v, width, height).unwrap();
                data.shrink_to_fit();
                *size = data.len();
                let raw_ptr = data.as_mut_ptr();
                std::mem::forget(v);
                std::mem::forget(data);
                return raw_ptr;
            }
            _ => panic!()
        }
    }
}

#[no_mangle]
pub extern "C" fn hnd_header_build() -> *mut modal::hnd_header_t {
    let header = Box::new(modal::hnd_header_t::new());
    Box::into_raw(header)
}

#[no_mangle]
pub extern "C" fn hnd_header_drop(ptr: *mut modal::hnd_header_t) {
    if !ptr.is_null() {
        let header = unsafe {Box::from_raw(ptr)};
        drop(header);
    }
}

#[no_mangle]
pub extern "C" fn hnd_header_set_width(ptr: *mut modal::hnd_header_t, width: u32) {
    if !ptr.is_null() {
        unsafe {(*ptr).SizeX = width;}
    }
}

#[no_mangle]
pub extern "C" fn hnd_header_set_height(ptr: *mut modal::hnd_header_t, height: u32) {
    if !ptr.is_null() {
        unsafe {(*ptr).SizeY = height;}
    }
}

#[no_mangle]
pub extern "C" fn hnd_header_set_x_res(ptr: *mut modal::hnd_header_t, x_res: f64) {
    if !ptr.is_null() {
        unsafe {
            (*ptr).dImageResolutionX = x_res;
            (*ptr).dIDUResolutionX = x_res;
        }
    }
}

#[no_mangle]
pub extern "C" fn hnd_header_set_y_res(ptr: *mut modal::hnd_header_t, y_res: f64) {
    if !ptr.is_null() {
        unsafe {
            (*ptr).dImageResolutionY = y_res;
            (*ptr).dIDUResolutionY = y_res;
        }
    }
}

#[no_mangle]
pub extern "C" fn hnd_header_set_angle(ptr: *mut modal::hnd_header_t, angle: f64) {
    if !ptr.is_null() {
        unsafe {
            (*ptr).dCTProjectionAngle = angle;
            // println!("{}", (*ptr));
        }
    }
}

// use libc::{size_t, };

#[no_mangle]
pub extern "C" fn hnd_header_to_raw(ptr: *mut modal::hnd_header_t) -> *const u8 {
    if !ptr.is_null() {
        let mut raw_header_buf = unsafe {(*ptr).to_raw()};
        let raw_ptr = raw_header_buf.as_mut_ptr();
        std::mem::forget(raw_header_buf);
        return raw_ptr;
    }
    return std::ptr::null(); 
}

#[no_mangle]
pub extern "C" fn hnd_header_raw_drop(ptr: *mut u8) {
    if !ptr.is_null() {
        let buf = unsafe {Vec::<u8>::from_raw_parts(ptr, 1024, 1024)};
        drop(buf);
    }
}