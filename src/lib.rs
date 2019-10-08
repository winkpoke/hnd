#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into, TryInto};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};

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

struct Buf {
    data: Vec<u8>,
    pos: usize,
}

impl Buf {
    fn new() -> Self {
        let mut data = Vec::<u8>::with_capacity(1024);
        data.resize(1024, 0);
        Self { data: data, pos: 0 }
    }

    fn from(d: &[u8]) -> Self {
        let mut data = Vec::<u8>::new();
        data.extend_from_slice(d);
        Self { data: data, pos: 0 }
    }

    fn read_string(&mut self, size: usize) -> String {
        let (start, end) = (self.pos, self.pos + size);
        self.pos += size;
        std::str::from_utf8(&self.data[start..end])
            .unwrap()
            .trim_end_matches('\u{0}')
            .to_string()
    }

    fn read_u32(&mut self) -> u32 {
        let size: usize = 4;
        let (start, end) = (self.pos, self.pos + size);
        self.pos += size;
        u32::from_ne_bytes(self.data[start..end].try_into().unwrap())
    }

    fn read_f64(&mut self) -> f64 {
        let size: usize = 8;
        let (start, end) = (self.pos, self.pos + size);
        self.pos += size;
        f64::from_bits(u64::from_ne_bytes(
            self.data[start..end].try_into().unwrap(),
        ))
    }

    fn write_string(&mut self, data: &str, size: usize) {
        self.data[self.pos..]
            .iter_mut()
            .zip(
                data.as_bytes()
                    .iter()
                    .chain(Vec::with_capacity(size).iter())
                    .take(size),
            )
            .for_each(|(to, from)| *to = *from);
        self.pos += size;
    }
    fn write_u32(&mut self, data: u32) {
        let size: usize = 4;
        self.data[self.pos..]
            .iter_mut()
            .zip(data.to_ne_bytes().iter().take(size))
            .for_each(|(to, from)| *to = *from);
        self.pos += size;
    }

    fn write_f64(&mut self, data: f64) {
        let size: usize = 8;
        self.data[self.pos..]
            .iter_mut()
            .zip(data.to_bits().to_ne_bytes().iter().take(size))
            .for_each(|(to, from)| *to = *from);
        self.pos += size;
    }
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

impl Into<hnd_header_t> for hnd_header_raw_t {
    fn into(self) -> hnd_header_t {
        let mut pos: usize = 0;
        let mut buf = Buf::from(&self);
        hnd_header_t {
            sFileType: buf.read_string(32),
            FileLength: buf.read_u32(),
            chasChecksumSpec: buf.read_string(4),
            nCheckSum: buf.read_u32(),
            sCreationDate: buf.read_string(8),
            sCreationTime: buf.read_string(8),
            sPatientID: buf.read_string(16),
            nPatientSer: buf.read_u32(),
            sSeriesID: buf.read_string(16),
            nSeriesSer: buf.read_u32(),
            sSliceID: buf.read_string(16),
            nSliceSer: buf.read_u32(),
            SizeX: buf.read_u32(),
            SizeY: buf.read_u32(),
            dSliceZPos: buf.read_f64(),
            sModality: buf.read_string(16),
            nWindow: buf.read_u32(),
            nLevel: buf.read_u32(),
            nPixelOffset: buf.read_u32(),
            sImageType: buf.read_string(4),
            dGantryRtn: buf.read_f64(),              //f64,
            dSAD: buf.read_f64(),                    //f64,
            dSFD: buf.read_f64(),                    //f64,
            dCollX1: buf.read_f64(),                 //f64,
            dCollX2: buf.read_f64(),                 //f64,
            dCollY1: buf.read_f64(),                 //f64,
            dCollY2: buf.read_f64(),                 //f64,
            dCollRtn: buf.read_f64(),                //f64,
            dFieldX: buf.read_f64(),                 //f64,
            dFieldY: buf.read_f64(),                 //f64,
            dBladeX1: buf.read_f64(),                //f64,
            dBladeX2: buf.read_f64(),                //f64,
            dBladeY1: buf.read_f64(),                //f64,
            dBladeY2: buf.read_f64(),                //f64,
            dIDUPosLng: buf.read_f64(),              //f64,
            dIDUPosLat: buf.read_f64(),              //f64,
            dIDUPosVrt: buf.read_f64(),              //f64,
            dIDUPosRtn: buf.read_f64(),              //f64,
            dPatientSupportAngle: buf.read_f64(),    //f64,
            dTableTopEccentricAngle: buf.read_f64(), //f64,
            dCouchVrt: buf.read_f64(),               //f64,
            dCouchLng: buf.read_f64(),               //f64,
            dCouchLat: buf.read_f64(),               //f64,
            dIDUResolutionX: buf.read_f64(),         //f64,
            dIDUResolutionY: buf.read_f64(),         //f64,
            dImageResolutionX: buf.read_f64(),       //f64,
            dImageResolutionY: buf.read_f64(),       //f64,
            dEnergy: buf.read_f64(),                 //f64,
            dDoseRate: buf.read_f64(),               //f64,
            dXRayKV: buf.read_f64(),                 //f64,
            dXRayMA: buf.read_f64(),                 //f64,
            dMetersetExposure: buf.read_f64(),       //f64,
            dAcqAdjustment: buf.read_f64(),          //f64,
            dCTProjectionAngle: buf.read_f64(),      //f64,
            dCTNormChamber: buf.read_f64(),          //f64,
            dGatingTimeTag: buf.read_f64(),          //f64,
            dGating4DInfoX: buf.read_f64(),          //f64,
            dGating4DInfoY: buf.read_f64(),          //f64,
            dGating4DInfoZ: buf.read_f64(),          //f64,
            dGating4DInfoTime: buf.read_f64(),       //f64,
        }
    }
}

impl Into<hnd_header_raw_t> for hnd_header_t {
    fn into(self) -> hnd_header_raw_t {
        let mut buf = Buf::new();

        // iter!(String, buf_iter, self.sFileType, 32);
        buf.write_string(&self.sFileType, 32);
        buf.write_u32(self.FileLength);
        buf.write_string(&self.chasChecksumSpec, 4);
        buf.write_u32(self.nCheckSum);
        buf.write_string(&self.sCreationDate, 8);
        buf.write_string(&self.sCreationTime, 8);
        buf.write_string(&self.sPatientID, 16); //[u8; 16],
        buf.write_u32(self.nPatientSer);
        buf.write_string(&self.sSeriesID, 16); //[u8; 16],
        buf.write_u32(self.nSeriesSer);
        buf.write_string(&self.sSliceID, 16); //[u8; 16],
        buf.write_u32(self.nSliceSer);
        buf.write_u32(self.SizeX);
        buf.write_u32(self.SizeY);
        buf.write_f64(self.dSliceZPos);
        buf.write_string(&self.sModality, 16); //[u8; 16],
        buf.write_u32(self.nWindow);
        buf.write_u32(self.nLevel);
        buf.write_u32(self.nPixelOffset);
        buf.write_string(&self.sImageType, 4); //[u8; 4],
        buf.write_f64(self.dGantryRtn);
        buf.write_f64(self.dSAD);
        buf.write_f64(self.dSFD);
        buf.write_f64(self.dCollX1);
        buf.write_f64(self.dCollX2);
        buf.write_f64(self.dCollY1);
        buf.write_f64(self.dCollY2);
        buf.write_f64(self.dCollRtn);
        buf.write_f64(self.dFieldX);
        buf.write_f64(self.dFieldY);
        buf.write_f64(self.dBladeX1);
        buf.write_f64(self.dBladeX2);
        buf.write_f64(self.dBladeY1);
        buf.write_f64(self.dBladeY2);
        buf.write_f64(self.dIDUPosLng);
        buf.write_f64(self.dIDUPosLat);
        buf.write_f64(self.dIDUPosVrt);
        buf.write_f64(self.dIDUPosRtn);
        buf.write_f64(self.dPatientSupportAngle);
        buf.write_f64(self.dTableTopEccentricAngle);
        buf.write_f64(self.dCouchVrt);
        buf.write_f64(self.dCouchLng);
        buf.write_f64(self.dCouchLat);
        buf.write_f64(self.dIDUResolutionX);
        buf.write_f64(self.dIDUResolutionY);
        buf.write_f64(self.dImageResolutionX);
        buf.write_f64(self.dImageResolutionY);
        buf.write_f64(self.dEnergy);
        buf.write_f64(self.dDoseRate);
        buf.write_f64(self.dXRayKV);
        buf.write_f64(self.dXRayMA);
        buf.write_f64(self.dMetersetExposure);
        buf.write_f64(self.dAcqAdjustment);
        buf.write_f64(self.dCTProjectionAngle);
        buf.write_f64(self.dCTNormChamber);
        buf.write_f64(self.dGatingTimeTag);
        buf.write_f64(self.dGating4DInfoX);
        buf.write_f64(self.dGating4DInfoY);
        buf.write_f64(self.dGating4DInfoZ);
        buf.write_f64(self.dGating4DInfoTime);
        // );
        let mut array: [u8; 1024] = [0; 1024];
        array.copy_from_slice(&buf.data[0..1024]);
        array
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
    f.write(h.sFileType.as_ref())?;
    Ok(())
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

// struct LutIterMut<'a> {
//     table: &'a mut [u8],
//     size: usize,
//     pos: usize,
//     idx: usize,
//     offset: usize,
// }

// impl<'a> LutIterMut<'a> {
//     fn new(part: &'a mut [u8], size: usize) -> LutIterMut<'a> {
//         LutIterMut {
//             table: part,
//             size: size,
//             pos: 0,
//             idx: 0,
//             offset: 0,
//         }
//     }
// }

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
        let data = parse_data(&self.data, width, height)?;

        Ok(RawImage {
            width: width,
            height: height,
            data,
        })
    }
}

fn encode_data_u32(
    img: Vec<u32>,
    width: usize,
    height: usize,
) -> Result<hnd_data_t, ImageConvError> {
    // Initialize the hnd_data_t structure
    let lut_size: usize = (height - 1) * width / 4;
    let mut hnd_data: hnd_data_t = Vec::with_capacity(width * height * 4 + lut_size);
    hnd_data.resize(lut_size, 0);

    // Create LUT
    let lut = hnd_data.as_mut_slice();
    assert_eq!(lut.len(), lut_size);

    // Copy the first line and first pixel of the second line of the raw image
    img[..(width + 1)]
        .iter()
        .for_each(|x| x.to_ne_bytes().iter().for_each(|x| hnd_data.push(*x)));

    // Go through the rest of the pixels and encode into hnd format
    Err(ImageConvError::SomeErr)
}

fn encode_data_u16(
    img: Vec<u32>,
    width: usize,
    height: usize,
) -> Result<hnd_data_t, ImageConvError> {
    Err(ImageConvError::SomeErr)
}

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
    fn test_read_raw_32() {
        use std::convert::TryInto;
        use std::io::{BufReader, Read, Seek, SeekFrom};

        // test hnd file
        let test_file_1 = String::from("test/test_data_1.hnd");
        let mut f_test = std::fs::File::open(test_file_1).unwrap();
        let test_data = crate::read_hnd_data(&mut f_test).unwrap();

        // raw file to compare with
        let raw_file_1 = String::from("test/test_data_1.raw");
        let mut f_raw = std::fs::File::open(raw_file_1).unwrap();
    }

    #[test]
    fn test_read_raw_16() {}

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
