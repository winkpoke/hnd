#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into, TryInto};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub enum ImageConvError {}

#[derive(Default, Debug, Clone)]
pub struct hnd_header_t {
    sFileType: String, //[u8; 32],
    FileLength: u32,
    chasChecksumSpec: String, //[u8; 4],
    nCheckSum: u32,
    sCreationDate: String, //[u8; 8],
    sCreationTime: String, //[u8; 8],
    sPatientID: String,    //[u8; 16],
    nPatientSer: u32,
    sSeriesID: String, //[u8; 16],
    nSeriesSer: u32,
    sSliceID: String, //[u8; 16],
    nSliceSer: u32,
    SizeX: u32,
    SizeY: u32,
    dSliceZPos: f64,
    sModality: String, //[u8; 16],
    nWindow: u32,
    nLevel: u32,
    nPixelOffset: u32,
    sImageType: String, //[u8; 4],
    dGantryRtn: f64,
    dSAD: f64,
    dSFD: f64,
    dCollX1: f64,
    dCollX2: f64,
    dCollY1: f64,
    dCollY2: f64,
    dCollRtn: f64,
    dFieldX: f64,
    dFieldY: f64,
    dBladeX1: f64,
    dBladeX2: f64,
    dBladeY1: f64,
    dBladeY2: f64,
    dIDUPosLng: f64,
    dIDUPosLat: f64,
    dIDUPosVrt: f64,
    dIDUPosRtn: f64,
    dPatientSupportAngle: f64,
    dTableTopEccentricAngle: f64,
    dCouchVrt: f64,
    dCouchLng: f64,
    dCouchLat: f64,
    dIDUResolutionX: f64,
    dIDUResolutionY: f64,
    dImageResolutionX: f64,
    dImageResolutionY: f64,
    dEnergy: f64,
    dDoseRate: f64,
    dXRayKV: f64,
    dXRayMA: f64,
    dMetersetExposure: f64,
    dAcqAdjustment: f64,
    dCTProjectionAngle: f64,
    dCTNormChamber: f64,
    dGatingTimeTag: f64,
    dGating4DInfoX: f64,
    dGating4DInfoY: f64,
    dGating4DInfoZ: f64,
    dGating4DInfoTime: f64,
}

type hnd_header_raw_t = [u8; 1024];
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

impl std::fmt::Display for hnd_header_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File Type:\t{}", self.sFileType)?;
        writeln!(f, "File Length:\t{}", self.FileLength)?;
        writeln!(f, "chasChecksumSpec:\t{}", self.chasChecksumSpec)?;
        writeln!(f, "Check Sum:\t{}", self.nCheckSum)?;
        writeln!(f, "Creation Date:\t{}", self.sCreationDate)?;
        writeln!(f, "Creation Time:\t{}", self.sCreationTime)?;
        writeln!(f, "Patient ID:\t{}", self.sPatientID)?;
        writeln!(f, "Patient Ser:\t{}", self.nPatientSer)?;
        writeln!(f, "Series ID:\t{}", self.sSeriesID)?;
        writeln!(f, "Series Ser:\t{}", self.nSeriesSer)?;
        writeln!(f, "Slice ID:\t{}", self.sSliceID)?;
        writeln!(f, "Slice Ser:\t{}", self.nSliceSer)?;
        writeln!(f, "SizeX:\t{}", self.SizeX)?;
        writeln!(f, "SizeY:\t{}", self.SizeY)?;

        writeln!(f, "dSliceZPos:\t{:e}", self.dSliceZPos)?;
        writeln!(f, "sModality:\t{}", self.sModality)?;
        writeln!(f, "nWindow:\t{}", self.nWindow)?;
        writeln!(f, "nLevel:\t{}", self.nLevel)?;
        writeln!(f, "nPixelOffset:\t{}", self.nPixelOffset)?;
        writeln!(f, "sImageType:\t{}", self.sImageType)?;
        writeln!(f, "dGantryRtn:\t{}", self.dGantryRtn)?;
        writeln!(f, "dSAD:\t{}", self.dSAD)?;
        writeln!(f, "dSFD:\t{}", self.dSFD)?;
        writeln!(f, "dCollX1:\t{}", self.dCollX1)?;
        writeln!(f, "dCollX2:\t{}", self.dCollX2)?;
        writeln!(f, "dCollY1:\t{}", self.dCollY1)?;
        writeln!(f, "dCollY2:\t{}", self.dCollY2)?;
        writeln!(f, "dCollRtn:\t{}", self.dCollRtn)?;
        writeln!(f, "dFieldX:\t{}", self.dFieldX)?;
        writeln!(f, "dFieldY:\t{}", self.dFieldY)?;
        writeln!(f, "dBladeX1:\t{}", self.dBladeX1)?;
        writeln!(f, "dBladeX2:\t{}", self.dBladeX2)?;
        writeln!(f, "dBladeY1:\t{}", self.dBladeY1)?;
        writeln!(f, "dBladeY2:\t{}", self.dBladeY2)?;
        writeln!(f, "dIDUPosLng:\t{}", self.dIDUPosLng)?;
        writeln!(f, "dIDUPosLat:\t{}", self.dIDUPosLat)?;
        writeln!(f, "dIDUPosVrt:\t{}", self.dIDUPosVrt)?;
        writeln!(f, "dIDUPosRtn:\t{}", self.dIDUPosRtn)?;

        writeln!(f, "dPatientSupportAngle:\t{:e}", self.dPatientSupportAngle)?;
        writeln!(
            f,
            "dTableTopEccentricAngle:\t{:e}",
            self.dTableTopEccentricAngle
        )?;
        writeln!(f, "dCouchVrt:\t{:e}", self.dCouchVrt)?;
        writeln!(f, "dCouchLng:\t{:e}", self.dCouchLng)?;
        writeln!(f, "dCouchLat:\t{:e}", self.dCouchLat)?;
        writeln!(f, "dIDUResolutionX:\t{:e}", self.dIDUResolutionX)?;
        writeln!(f, "dIDUResolutionY:\t{:e}", self.dIDUResolutionY)?;
        writeln!(f, "dImageResolutionX:\t{:e}", self.dImageResolutionX)?;
        writeln!(f, "dImageResolutionY:\t{:e}", self.dImageResolutionY)?;
        writeln!(f, "dEnergy:\t{:e}", self.dEnergy)?;
        writeln!(f, "dDoseRate:\t{:e}", self.dDoseRate)?;
        writeln!(f, "dXRayKV:\t{:e}", self.dXRayKV)?;
        writeln!(f, "dXRayMA:\t{:e}", self.dXRayMA)?;
        writeln!(f, "dMetersetExposure:\t{:e}", self.dMetersetExposure)?;
        writeln!(f, "dAcqAdjustment:\t{:e}", self.dAcqAdjustment)?;
        writeln!(f, "dCTProjectionAngle:\t{:e}", self.dCTProjectionAngle)?;
        writeln!(f, "dCTNormChamber:\t{:e}", self.dCTNormChamber)?;
        writeln!(f, "dGatingTimeTag:\t{:e}", self.dGatingTimeTag)?;
        writeln!(f, "dGating4DInfoX:\t{:e}", self.dGating4DInfoX)?;
        writeln!(f, "dGating4DInfoY:\t{:e}", self.dGating4DInfoY)?;
        writeln!(f, "dGating4DInfoZ:\t{:e}", self.dGating4DInfoZ)?;
        writeln!(f, "dGating4DInfoTime:\t{:e}", self.dGating4DInfoTime)?;

        Ok(())
    }
}

pub fn read_header_to_raw(f: &File) -> Result<hnd_header_raw_t, io::Error> {
    let mut reader = BufReader::new(f);
    let mut buf: hnd_header_raw_t = [0; 1024];
    let n: usize = reader.read(&mut buf[..1024])?;
    println!("DEBUG: read in {} bytes in total.", n);
    return Ok(buf);
}

fn parse_u32(buf: &[u8], pos: &mut usize) -> u32 {
    let (start, end) = (*pos, *pos + 4);
    *pos += 4;
    u32::from_ne_bytes(buf[start..end].try_into().unwrap())
}

fn parse_f64(buf: &[u8], pos: &mut usize) -> f64 {
    let (start, end) = (*pos, *pos + 8);
    *pos += 8;
    f64::from_bits(u64::from_ne_bytes(buf[start..end].try_into().unwrap()))
}

fn parse_string(buf: &[u8], pos: &mut usize, len: usize) -> String {
    let (start, end) = (*pos, *pos + len);
    *pos += len;
    std::str::from_utf8(&buf[start..end])
        .unwrap()
        .trim_end_matches('\u{0}')
        .to_string()
}

impl Into<hnd_header_t> for hnd_header_raw_t {
    fn into(self) -> hnd_header_t {
        let mut pos: usize = 0;
        hnd_header_t {
            sFileType: { parse_string(&self, &mut pos, 32) },
            FileLength: { parse_u32(&self, &mut pos) },
            chasChecksumSpec: { parse_string(&self, &mut pos, 4) },
            nCheckSum: { parse_u32(&self, &mut pos) },
            sCreationDate: { parse_string(&self, &mut pos, 8) },
            sCreationTime: { parse_string(&self, &mut pos, 8) },
            sPatientID: { parse_string(&self, &mut pos, 16) },
            nPatientSer: { parse_u32(&self, &mut pos) },
            sSeriesID: { parse_string(&self, &mut pos, 16) },
            nSeriesSer: { parse_u32(&self, &mut pos) },
            sSliceID: { parse_string(&self, &mut pos, 16) },
            nSliceSer: { parse_u32(&self, &mut pos) },
            SizeX: { parse_u32(&self, &mut pos) },
            SizeY: { parse_u32(&self, &mut pos) },
            dSliceZPos: { parse_f64(&self, &mut pos) },
            sModality: { parse_string(&self, &mut pos, 16) },
            nWindow: { parse_u32(&self, &mut pos) },
            nLevel: { parse_u32(&self, &mut pos) },
            nPixelOffset: { parse_u32(&self, &mut pos) },
            sImageType: { parse_string(&self, &mut pos, 4) },
            dGantryRtn: { parse_f64(&self, &mut pos) }, //f64,
            dSAD: { parse_f64(&self, &mut pos) },       //f64,
            dSFD: { parse_f64(&self, &mut pos) },       //f64,
            dCollX1: { parse_f64(&self, &mut pos) },    //f64,
            dCollX2: { parse_f64(&self, &mut pos) },    //f64,
            dCollY1: { parse_f64(&self, &mut pos) },    //f64,
            dCollY2: { parse_f64(&self, &mut pos) },    //f64,
            dCollRtn: { parse_f64(&self, &mut pos) },   //f64,
            dFieldX: { parse_f64(&self, &mut pos) },    //f64,
            dFieldY: { parse_f64(&self, &mut pos) },    //f64,
            dBladeX1: { parse_f64(&self, &mut pos) },   //f64,
            dBladeX2: { parse_f64(&self, &mut pos) },   //f64,
            dBladeY1: { parse_f64(&self, &mut pos) },   //f64,
            dBladeY2: { parse_f64(&self, &mut pos) },   //f64,
            dIDUPosLng: { parse_f64(&self, &mut pos) }, //f64,
            dIDUPosLat: { parse_f64(&self, &mut pos) }, //f64,
            dIDUPosVrt: { parse_f64(&self, &mut pos) }, //f64,
            dIDUPosRtn: { parse_f64(&self, &mut pos) }, //f64,
            dPatientSupportAngle: { parse_f64(&self, &mut pos) }, //f64,
            dTableTopEccentricAngle: { parse_f64(&self, &mut pos) }, //f64,
            dCouchVrt: { parse_f64(&self, &mut pos) },  //f64,
            dCouchLng: { parse_f64(&self, &mut pos) },  //f64,
            dCouchLat: { parse_f64(&self, &mut pos) },  //f64,
            dIDUResolutionX: { parse_f64(&self, &mut pos) }, //f64,
            dIDUResolutionY: { parse_f64(&self, &mut pos) }, //f64,
            dImageResolutionX: { parse_f64(&self, &mut pos) }, //f64,
            dImageResolutionY: { parse_f64(&self, &mut pos) }, //f64,
            dEnergy: { parse_f64(&self, &mut pos) },    //f64,
            dDoseRate: { parse_f64(&self, &mut pos) },  //f64,
            dXRayKV: { parse_f64(&self, &mut pos) },    //f64,
            dXRayMA: { parse_f64(&self, &mut pos) },    //f64,
            dMetersetExposure: { parse_f64(&self, &mut pos) }, //f64,
            dAcqAdjustment: { parse_f64(&self, &mut pos) }, //f64,
            dCTProjectionAngle: { parse_f64(&self, &mut pos) }, //f64,
            dCTNormChamber: { parse_f64(&self, &mut pos) }, //f64,
            dGatingTimeTag: { parse_f64(&self, &mut pos) }, //f64,
            dGating4DInfoX: { parse_f64(&self, &mut pos) }, //f64,
            dGating4DInfoY: { parse_f64(&self, &mut pos) }, //f64,
            dGating4DInfoZ: { parse_f64(&self, &mut pos) }, //f64,
            dGating4DInfoTime: { parse_f64(&self, &mut pos) }, //f64,
        }
    }
}

// fn copy_string_by_size<'a>(buf: &'a mut [u8], s: &str, start: usize, len: usize) -> usize {
//     let mut tmp = s.into_bytes();
//     tmp.resize(len, 0);
//     buf[start..start + len].copy_from_slice(&tmp);
//     return start + len;
// }

// use std::iter::{Chain, Take};
// use std::str::Bytes;
// fn str_to_iter<const N: usize>(s: &str) -> Take<T> {
//     s.bytes().chain([0; N].iter()).take(N)
// }
//
struct Buf {
    data: [u8; 1024],
    pos: usize,
}

impl Buf {
    fn new() -> Buf {
        Buf {
            data: [0; 1024],
            pos: 0,
        }
    }
}

#[macro_export]
macro_rules! iter {
    ( String , $buf_iter: expr, $name: expr , $n: expr ) => {
        // $name
        //     .clone()
        //     .into_bytes()
        //     .iter()
        //     .chain([0; $n].iter())
        //     .take($n);
        print!("{} ----> ", $name);
        let tmp_internal_8293 = $name.into_bytes();
        let mut i = 0;
        $buf_iter.data[$buf_iter.pos..]
            .iter_mut()
            .zip(tmp_internal_8293.iter().chain([0; $n].iter()).take($n))
            .for_each(|(to, from)| {
                print!("{:X} ", *from);
                i += 1;
                *to = *from
            });
        println!(" --- {}", i);
        $buf_iter.pos += $n
    };

    ( u32 , $buf_iter: expr , $name: expr , $n: expr ) => {
        // $name.to_ne_bytes().iter().take(4);
        print!("{} ----> ", $name);
        let tmp_internal_7293 = $name.to_ne_bytes();
        let mut i = 0;
        $buf_iter.data[$buf_iter.pos..]
            .iter_mut()
            .zip(tmp_internal_7293.iter().take(4))
            .for_each(|(to, from)| {
                print!("{:X} ", *from);
                i += 1;
                *to = *from
            });
        println!(" --- {}", i);
        $buf_iter.pos += 4;
    };

    ( f64 , $buf_iter: expr , $name: expr , $n: expr ) => {
        // $name.to_bits().to_ne_bytes().iter().take(8);
        let tmp_internal_8693 = $name.to_bits().to_ne_bytes();
        $buf_iter.data[$buf_iter.pos..]
            .iter_mut()
            .zip(tmp_internal_8693.iter().take(8))
            .for_each(|(to, from)| *to = *from);
        $buf_iter.pos += 8;
    };
}

macro_rules! chain {
    ( $e1: expr , $( $e: expr ),* ) => {
        {
            let mut tmp = $e1;
            $(
                tmp.chain($e);
            )*
            tmp
        }
    };
}
use std::iter::Take;
impl Into<hnd_header_raw_t> for hnd_header_t {
    fn into(self) -> hnd_header_raw_t {
        let mut buf_iter = Buf::new();
        // let mut buf: hnd_header_raw_t = [0; 1024];

        // let buf_iter = &mut buf.iter_mut();
        // let tmp = self.sFileType.into_bytes();
        // buf_iter
        //     .zip(tmp.iter().chain([0; 32].iter()).take(32))
        //     .for_each(|(to, from)| *to = *from);
        // let mut src = chain!(
        iter!(String, buf_iter, self.sFileType, 32);
        iter!(u32, buf_iter, self.FileLength, 4);
        iter!(String, buf_iter, self.chasChecksumSpec, 4);
        iter!(u32, buf_iter, self.nCheckSum, 4);
        iter!(String, buf_iter, self.sCreationDate, 8);
        iter!(String, buf_iter, self.sCreationTime, 8);
        iter!(String, buf_iter, self.sPatientID, 16); //[u8; 16],
        iter!(u32, buf_iter, self.nPatientSer, 4);
        iter!(String, buf_iter, self.sSeriesID, 16); //[u8; 16],
        iter!(u32, buf_iter, self.nSeriesSer, 4);
        iter!(String, buf_iter, self.sSliceID, 16); //[u8; 16],
        iter!(u32, buf_iter, self.nSliceSer, 4);
        iter!(u32, buf_iter, self.SizeX, 4);
        iter!(u32, buf_iter, self.SizeY, 4);
        iter!(f64, buf_iter, self.dSliceZPos, 8);
        iter!(String, buf_iter, self.sModality, 16); //[u8; 16],
        iter!(u32, buf_iter, self.nWindow, 4);
        iter!(u32, buf_iter, self.nLevel, 4);
        iter!(u32, buf_iter, self.nPixelOffset, 4);
        iter!(String, buf_iter, self.sImageType, 4); //[u8; 4],
        iter!(f64, buf_iter, self.dGantryRtn, 8);
        iter!(f64, buf_iter, self.dSAD, 8);
        iter!(f64, buf_iter, self.dSFD, 8);
        iter!(f64, buf_iter, self.dCollX1, 8);
        iter!(f64, buf_iter, self.dCollX2, 8);
        iter!(f64, buf_iter, self.dCollY1, 8);
        iter!(f64, buf_iter, self.dCollY2, 8);
        iter!(f64, buf_iter, self.dCollRtn, 8);
        iter!(f64, buf_iter, self.dFieldX, 8);
        iter!(f64, buf_iter, self.dFieldY, 8);
        iter!(f64, buf_iter, self.dBladeX1, 8);
        iter!(f64, buf_iter, self.dBladeX2, 8);
        iter!(f64, buf_iter, self.dBladeY1, 8);
        iter!(f64, buf_iter, self.dBladeY2, 8);
        iter!(f64, buf_iter, self.dIDUPosLng, 8);
        iter!(f64, buf_iter, self.dIDUPosLat, 8);
        iter!(f64, buf_iter, self.dIDUPosVrt, 8);
        iter!(f64, buf_iter, self.dIDUPosRtn, 8);
        iter!(f64, buf_iter, self.dPatientSupportAngle, 8);
        iter!(f64, buf_iter, self.dTableTopEccentricAngle, 8);
        iter!(f64, buf_iter, self.dCouchVrt, 8);
        iter!(f64, buf_iter, self.dCouchLng, 8);
        iter!(f64, buf_iter, self.dCouchLat, 8);
        iter!(f64, buf_iter, self.dIDUResolutionX, 8);
        iter!(f64, buf_iter, self.dIDUResolutionY, 8);
        iter!(f64, buf_iter, self.dImageResolutionX, 8);
        iter!(f64, buf_iter, self.dImageResolutionY, 8);
        iter!(f64, buf_iter, self.dEnergy, 8);
        iter!(f64, buf_iter, self.dDoseRate, 8);
        iter!(f64, buf_iter, self.dXRayKV, 8);
        iter!(f64, buf_iter, self.dXRayMA, 8);
        iter!(f64, buf_iter, self.dMetersetExposure, 8);
        iter!(f64, buf_iter, self.dAcqAdjustment, 8);
        iter!(f64, buf_iter, self.dCTProjectionAngle, 8);
        iter!(f64, buf_iter, self.dCTNormChamber, 8);
        iter!(f64, buf_iter, self.dGatingTimeTag, 8);
        iter!(f64, buf_iter, self.dGating4DInfoX, 8);
        iter!(f64, buf_iter, self.dGating4DInfoY, 8);
        iter!(f64, buf_iter, self.dGating4DInfoZ, 8);
        iter!(f64, buf_iter, self.dGating4DInfoTime, 8);
        // );
        return buf_iter.data;
    }
}

pub fn print_header(f: &mut File) -> Result<(), io::Error> {
    let raw = read_header_to_raw(f)?;
    //let hnd_head = parse_header(&raw)?;
    let hnd_head: hnd_header_t = raw.into();

    //println!("DEBUG: {:?}", hnd_head);
    println!("{}", hnd_head);

    Ok(())
}

pub fn read_header(f: &mut File) -> Result<hnd_header_t, io::Error> {
    let raw = read_header_to_raw(f)?;
    let hnd_head: hnd_header_t = raw.into();
    //println!("DEBUG: {:?}", hnd_head);

    Ok(hnd_head)
}

fn write_header(f: &mut File, h: &hnd_header_t) -> Result<(), io::Error> {
    f.write(h.sFileType.as_ref());
    Ok(())
}

fn parse_data(raw: &hnd_data_t, width: usize, height: usize) -> Result<Vec<u32>, ImageConvError> {
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

    // Decompress the rest
    let mut lut_idx = 0;
    let mut lut_off = 0;
    let mut i = width + 1;
    while i < width * height {
        let r11 = output[i - width - 1];
        let r12 = output[i - width];
        let r21 = output[i - 1];
        let v = match lut_off {
            0 => {
                lut_off += 1;
                lut[lut_idx] & 0x03
            }
            1 => {
                lut_off += 1;
                (lut[lut_idx] & 0x0c) >> 2
            }
            2 => {
                lut_off += 1;
                (lut[lut_idx] & 0x30) >> 4
            }
            3 => {
                lut_off = 0;
                let idx = lut_idx;
                lut_idx += 1;
                (lut[idx] & 0xc0) >> 6
            }
            _ => {
                panic!("cannot reach here!");
            }
        };
        let diff: i32 = match v {
            0 => {
                let start = pos;
                let end = start + 1;
                pos += 1;
                i8::from_ne_bytes(raw[start..end].try_into().unwrap()).into()
            }
            1 => {
                let start = pos;
                let end = start + 2;
                pos += 2;
                i16::from_ne_bytes(raw[start..end].try_into().unwrap()).into()
            }
            2 => {
                let start = pos;
                let end = start + 4;
                pos += 4;
                i16::from_ne_bytes(raw[start..end].try_into().unwrap()).into()
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
        let data = parse_data(&self.data, width, height)?;

        Ok(RawImage {
            width: width,
            height: height,
            data,
        })
    }
}

//fn from_raw(img: &[u8], width: u32, height: u32) -> Result<Box> {}

fn read_hnd_data(f: &mut File) -> Result<hnd_data_t, io::Error> {
    let raw_header = read_header_to_raw(f)?;
    let header: hnd_header_t = raw_header.into();

    let w = header.SizeX;
    let h = header.SizeY;
    let len = w * h;
    let mut buf = Vec::new();

    // Skip HND header
    let n = f.seek(SeekFrom::Start(1024));
    let s = f.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn convert_to_raw(fin: &mut File, fout: &mut File) -> Result<(), io::Error> {
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
        let test_data = crate::read_hnd_data(&mut f_test).unwrap();

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
        let parsed = crate::parse_data(&test_data, 1024, 768).unwrap();

        //compare the results
        for i in 0..1024 * 768 {
            let x = parsed[i];
            let y = u32::from_ne_bytes(raw[i * 4..i * 4 + 4].try_into().unwrap());
            assert_eq!(x, y);
            print!(".");
        }
    }

    #[test]
    fn test_header_convertion() {
        use crate::*;
        let test_file_1 = String::from("test/test_data_1.hnd");
        let mut f = std::fs::File::open(test_file_1).unwrap();
        let header: hnd_header_t = crate::read_header(&mut f).unwrap();
        let raw_header_buf: hnd_header_raw_t = header.clone().into();
        let header2: hnd_header_t = raw_header_buf.clone().into();

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
