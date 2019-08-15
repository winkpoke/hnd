#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::convert::TryInto;
use std::fs::File;
use std::io;


#[derive(Default, Debug)]
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

pub struct hnd_header_raw_t {
    data: Box<[u8; 1024]>,
}

fn display_header(h: &hnd_header_t) {
    println!("{}", h.sFileType);
    println!("{}", h.FileLength);
    println!("{}", h.chasChecksumSpec);
    println!("{}", h.nCheckSum);
    println!("{}", h.sCreationDate);
    println!("{}", h.sCreationTime);
    println!("{}", h.sPatientID);
}

pub fn read_header_to_raw(f: &File) -> Result<Box<hnd_header_raw_t>, io::Error> {
    use std::io::{BufReader, Read};

    let mut reader = BufReader::new(f);
    let mut buf = Box::new([0; 1024]);
    let n: usize = reader.read(&mut (*buf)[..1024])?;
    println!("DEBUG: read in {} bytes in total.", n);
    return Ok(Box::new(hnd_header_raw_t { data: buf }));
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
    std::str::from_utf8(&buf[start..end]).unwrap().to_string()
}

pub fn parse_raw_data(raw: &hnd_header_raw_t) -> Result<Box<hnd_header_t>, io::Error> {
    let mut pos: usize = 0;
    let header = Box::new(hnd_header_t {
        sFileType: { parse_string(&*raw.data, &mut pos, 32) },
        FileLength: { parse_u32(&*raw.data, &mut pos) },
        chasChecksumSpec: { parse_string(&*raw.data, &mut pos, 4) },
        nCheckSum: { parse_u32(&*raw.data, &mut pos) },
        sCreationDate: { parse_string(&*raw.data, &mut pos, 8) },
        sCreationTime: { parse_string(&*raw.data, &mut pos, 8) },
        sPatientID: { parse_string(&*raw.data, &mut pos, 16) },
        nPatientSer: { parse_u32(&*raw.data, &mut pos) },
        sSeriesID: { parse_string(&*raw.data, &mut pos, 16) },
        nSeriesSer: { parse_u32(&*raw.data, &mut pos) },
        sSliceID: { parse_string(&*raw.data, &mut pos, 16) },
        nSliceSer: { parse_u32(&*raw.data, &mut pos) },
        SizeX: { parse_u32(&*raw.data, &mut pos) },
        SizeY: { parse_u32(&*raw.data, &mut pos) },
        dSliceZPos: { parse_f64(&*raw.data, &mut pos) },
        sModality: { parse_string(&*raw.data, &mut pos, 16) },
        nWindow: { parse_u32(&*raw.data, &mut pos) },
        nLevel: { parse_u32(&*raw.data, &mut pos) },
        nPixelOffset: { parse_u32(&*raw.data, &mut pos) },
        sImageType: { parse_string(&*raw.data, &mut pos, 4) },
        dGantryRtn: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dSAD: { parse_f64(&*raw.data, &mut pos) },       //f64,
        dSFD: { parse_f64(&*raw.data, &mut pos) },       //f64,
        dCollX1: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dCollX2: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dCollY1: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dCollY2: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dCollRtn: { parse_f64(&*raw.data, &mut pos) },   //f64,
        dFieldX: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dFieldY: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dBladeX1: { parse_f64(&*raw.data, &mut pos) },   //f64,
        dBladeX2: { parse_f64(&*raw.data, &mut pos) },   //f64,
        dBladeY1: { parse_f64(&*raw.data, &mut pos) },   //f64,
        dBladeY2: { parse_f64(&*raw.data, &mut pos) },   //f64,
        dIDUPosLng: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dIDUPosLat: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dIDUPosVrt: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dIDUPosRtn: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dPatientSupportAngle: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dTableTopEccentricAngle: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dCouchVrt: { parse_f64(&*raw.data, &mut pos) },  //f64,
        dCouchLng: { parse_f64(&*raw.data, &mut pos) },  //f64,
        dCouchLat: { parse_f64(&*raw.data, &mut pos) },  //f64,
        dIDUResolutionX: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dIDUResolutionY: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dImageResolutionX: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dImageResolutionY: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dEnergy: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dDoseRate: { parse_f64(&*raw.data, &mut pos) },  //f64,
        dXRayKV: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dXRayMA: { parse_f64(&*raw.data, &mut pos) },    //f64,
        dMetersetExposure: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dAcqAdjustment: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dCTProjectionAngle: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dCTNormChamber: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dGatingTimeTag: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dGating4DInfoX: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dGating4DInfoY: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dGating4DInfoZ: { parse_f64(&*raw.data, &mut pos) }, //f64,
        dGating4DInfoTime: { parse_f64(&*raw.data, &mut pos) }, //f64,
    });
    Ok(header)
}

pub fn print_header(f: &mut File) -> Result<(), io::Error> {
    let raw = read_header_to_raw(f)?;
    let hnd_head = parse_raw_data(&raw)?;
    println!("DEBUG: {:?}", hnd_head);

    Ok(())
}

pub fn read_header(f: &mut File) -> Result<Box<hnd_header_t>, io::Error> {
    let raw = read_header_to_raw(f)?;
    let hnd_head = parse_raw_data(&raw)?;
    println!("DEBUG: {:?}", hnd_head);

    Ok(hnd_head)
}
