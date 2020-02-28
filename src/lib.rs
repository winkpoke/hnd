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

const RAWMEMDATA_DEFAULT_SIZE: usize = 1024;
struct RawMemData {
    data: Box<[u8]>,
    len: usize,
}

impl RawMemData {
    fn new() -> RawMemData {
        let mut v = Vec::with_capacity(RAWMEMDATA_DEFAULT_SIZE);
        RawMemData {
            data: v.into_boxed_slice(),
            len: RAWMEMDATA_DEFAULT_SIZE,
        }
    }
}

#[derive(Debug)]
pub enum ImageConvError {
    SomeErr,
}

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

struct LutIter<'a> {
    table: &'a [u8],
    size: usize,
    pos: usize,
    idx: usize,
    offset: usize,
}

impl<'a> LutIter<'a> {
    fn new(part: &'a [u8], size: usize) -> LutIter<'a> {
        LutIter {
            table: part,
            size: size,
            pos: 0,
            idx: 0,
            offset: 0,
        }
    }
}

impl<'a> Iterator for LutIter<'a> {
    type Item = u8;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.size {
            self.pos += 1;
            let v = match self.offset {
                0 => {
                    self.offset += 1;
                    self.table[self.idx] & 0x03
                }
                1 => {
                    self.offset += 1;
                    (self.table[self.idx] & 0x0c) >> 2
                }
                2 => {
                    self.offset += 1;
                    (self.table[self.idx] & 0x30) >> 4
                }
                3 => {
                    self.offset = 0;
                    let idx = self.idx;
                    self.idx += 1;
                    (self.table[idx] & 0xc0) >> 6
                }
                _ => {
                    panic!("cannot reach here!");
                }
            };
            Some(v)
        } else {
            None
        }
    }
}

fn decode(raw: &hnd_data_t, width: usize, height: usize) -> Result<Vec<u32>, ImageConvError> {
    let mut output = Vec::with_capacity(width * height * 4);

    // Read LUT
    let lut_begin = 0;
    let lut_len = width * (height - 1) / 4;
    let lut_end = lut_begin + lut_len;
    let lut = &raw[lut_begin..lut_end];

    // first Row and the first pixel of the second row are uncompressed data,
    // which can be copied to output directly.
    let mut pos = lut_end + (width + 1) * 4;
    for i in (lut_end..pos).step_by(4) {
        let start = i;
        let end = start + 4;
        let v = u32::from_ne_bytes(raw[start..end].try_into().unwrap());
        output.push(v);
    }

    let lut_size = width * (height - 1) - 1;
    let mut lut_iter = LutIter::new(&lut, lut_size);

    // Decompress the rest
    let mut i = width + 1;
    while i < width * height {
        let v = lut_iter.next();
        let r11 = output[i - width - 1];
        let r12 = output[i - width];
        let r21 = output[i - 1];

        let start = pos;
        let diff: i32 = match v {
            Some(0) => {
                let end = start + 1;
                pos += 1;
                i8::from_ne_bytes(raw[start..end].try_into().unwrap()).into()
            }
            Some(1) => {
                let end = start + 2;
                pos += 2;
                i16::from_ne_bytes(raw[start..end].try_into().unwrap()).into()
            }
            Some(2) => {
                let end = start + 4;
                pos += 4;
                i32::from_ne_bytes(raw[start..end].try_into().unwrap()).into()
            }
            None => {
                break;
            }
            _ => {
                panic!("cannot reach here!");
            }
        };
        // println!(
        //   "lut[idx] = {} lut = {} i = {} r12 = {} r21 = {} r11 = {} diff = {}",
        //    lut[lut_idx], v, i, r12, r21, r11, diff
        // );
        let pixel_value: u32 = (r12 as i64 + r21 as i64 - r11 as i64 + diff as i64)
            .try_into()
            .unwrap();
        output.push(pixel_value);
        i += 1;
    }

    Ok(output)
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

fn compress_data_impl(hnd_data: &mut Vec<u8>, diff: i64, lut_off: &mut usize, lut_idx: &mut usize) {
    let mut v: u8 = 0;
    if diff >= i8::min_value().into() && diff <= i8::max_value().into() {
        (diff as i8)
            .to_ne_bytes()
            .iter()
            .for_each(|x| hnd_data.push(*x));
        v = 0;
    } else if diff >= i16::min_value().into() && diff <= i16::max_value().into() {
        (diff as i16)
            .to_ne_bytes()
            .iter()
            .for_each(|x| hnd_data.push(*x));
        v = 1;
    } else if diff >= i32::min_value().into() && diff <= i32::max_value().into() {
        (diff as i32)
            .to_ne_bytes()
            .iter()
            .for_each(|x| hnd_data.push(*x));
        v = 2;
    } else {
        panic!("shouldn't get here!");
    }

    // append the v value to the LUT table
    match lut_off {
        0 => {
            hnd_data[*lut_idx] = v;
            *lut_off += 1;
        }
        1 => {
            hnd_data[*lut_idx] |= v << 2;
            *lut_off += 1;
        }
        2 => {
            hnd_data[*lut_idx] |= v << 4;
            *lut_off += 1;
        }
        3 => {
            hnd_data[*lut_idx] |= v << 6;
            *lut_off = 0;
            *lut_idx += 1;
        }
        _ => {
            panic!("shouldn't get here!");
        }
    }
}

pub fn encode_u32(
    img: &Vec<u32>,
    width: usize,
    height: usize,
) -> Result<hnd_data_t, ImageConvError> {
    // Initialize the hnd_data_t structure
    const pixel_size: usize = mem::size_of::<u32>();
    let lut_size: usize = (height - 1) * width / 4;
    let mut hnd_data: hnd_data_t = Vec::with_capacity(width * height * pixel_size + lut_size);

    // LUT
    hnd_data.resize(lut_size, 0);

    // Copy the first line and first pixel of the second line of the raw image
    img[..(width + 1)]
        .iter()
        .for_each(|x| x.to_ne_bytes().iter().for_each(|x| hnd_data.push(*x)));

    // Go through the rest of the pixels and encode into hnd format
    let mut lut_off: usize = 0;
    let mut lut_idx: usize = 0;
    for i in (width + 1)..(width * height) {
        let r11 = img[i - width - 1];
        let r12 = img[i - width];
        let r21 = img[i - 1];
        // println!("{} {} {} {} {}", i, img[1], r11, r21, r12);
        let diff: i64 = img[i] as i64 + r11 as i64 - r21 as i64 - r12 as i64;
        compress_data_impl(&mut hnd_data, diff, &mut lut_off, &mut lut_idx);
    }

    Ok(hnd_data)
}

pub fn encode_u16(
    img: &Vec<u16>,
    width: usize,
    height: usize,
) -> Result<hnd_data_t, ImageConvError> {
    // Initialize the hnd_data_t structure
    const pixel_size: usize = mem::size_of::<u16>();
    let lut_size: usize = (height - 1) * width / 4;
    let mut hnd_data: hnd_data_t = Vec::with_capacity(width * height * pixel_size + lut_size);

    // LUT
    hnd_data.resize(lut_size, 0);

    // Copy the first line and first pixel of the second line of the raw image
    img[..(width + 1)]
        .iter()
        .for_each(|x| (*x as u32).to_ne_bytes().iter().for_each(|x| hnd_data.push(*x)));

    // Go through the rest of the pixels and encode into hnd format
    let mut lut_off: usize = 0;
    let mut lut_idx: usize = 0;
    for i in (width + 1)..(width * height) {
        let r11 = img[i - width - 1];
        let r12 = img[i - width];
        let r21 = img[i - 1];
        // println!("{} {} {} {} {}", i, img[1], r11, r21, r12);
        let diff: i64 = img[i] as i64 + r11 as i64 - r21 as i64 - r12 as i64;

        compress_data_impl(&mut hnd_data, diff, &mut lut_off, &mut lut_idx);
    }
    Ok(hnd_data)
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
    let hnd_head: hnd_header_t = hnd_header_t::from_slice_buf(&raw);

    //println!("DEBUG: {:?}", hnd_head);
    println!("{}", hnd_head);

    Ok(())
}

pub fn read_header(f: &mut File) -> Result<hnd_header_t, io::Error> {
    let raw = read_header_to_raw(f)?;
    let hnd_head: hnd_header_t = hnd_header_t::from_slice_buf(&raw);
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
    let mut buf: modal::hnd_header_buf_t = [0; 1024];
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
    let raw_header: modal::hnd_header_buf_t = hnd.header.to_slice_buf();
    f.write(&raw_header)?;
    f.write(hnd.data.as_ref())?;
    Ok(())
}

pub fn convert_to_raw(fin: &mut File, fout: &mut File) -> Result<(), io::Error> {
    let mut hnd_header_raw: modal::hnd_header_buf_t = [0; 1024];
    fin.read(&mut hnd_header_raw);

    let mut hnd_data_buf: Vec<u8> = Vec::new();
    fin.read_to_end(&mut hnd_data_buf);

    let hnd_header = hnd_header_t::from_slice_buf(&hnd_header_raw);
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
    unsafe {
        std::mem::forget(raw_image);
    }
    // let raw_image_buf: Vec<u8> = unsafe { std::mem::transmute(raw_image) };
    println!("{}", raw_image_buf.len());

    fout.write(raw_image_buf.as_slice());
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

        let mut f_out = tempfile::tempfile().unwrap();

        // Read in the raw data
        let mut raw = Vec::new();
        f_raw.read_to_end(&mut raw);
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
        use std::convert::TryInto;
        use std::io::{BufReader, Read, Seek, SeekFrom};

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
        let mut raw_vec_u32: Vec<u32>;
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
        let raw_header_buf: modal::hnd_header_buf_t = header.to_slice_buf();
        let header2: hnd_header_t = hnd_header_t::from_slice_buf(&raw_header_buf);

        println!("{:?}", header);
        println!("{:?}", header2);
        raw_header_buf
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

// export functions 
#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
pub extern "C" fn encode(img: *mut u8, width: usize, height: usize, channel: usize, size: *mut usize) -> *mut u8 {
    let len = width * height;
    unsafe {
        match channel {
            2 => {
                let v = Vec::from_raw_parts(img as *mut u16, len, len);
                let mut data = encode_u16(&v, width, height).unwrap();
                data.shrink_to_fit();
                unsafe {*size = data.len();}
                let raw_ptr = data.as_mut_ptr();
                std::mem::forget(data);
                return raw_ptr;
            }
            4 => {
                let v = Vec::from_raw_parts(img as *mut u32, len, len);
                let mut data = encode_u32(&v, width, height).unwrap();
                data.shrink_to_fit();
                unsafe {*size = data.len();}
                let raw_ptr = data.as_mut_ptr();
                std::mem::forget(data);
                return raw_ptr;
            }
            _ => panic!()
        }
    }
}