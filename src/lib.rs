#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

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
    std::str::from_utf8(&buf[start..end])
        .unwrap()
        .trim_end_matches('\u{0}')
        .to_string()
}

pub fn parse_header(raw: &hnd_header_raw_t) -> Result<Box<hnd_header_t>, io::Error> {
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
    let hnd_head = parse_header(&raw)?;
    //println!("DEBUG: {:?}", hnd_head);
    println!("{}", hnd_head);
    Ok(())
}

pub fn read_header(f: &mut File) -> Result<Box<hnd_header_t>, io::Error> {
    let raw = read_header_to_raw(f)?;
    let hnd_head = parse_header(&raw)?;
    //println!("DEBUG: {:?}", hnd_head);

    Ok(hnd_head)
}

pub struct hnd_data_t {
    data: Vec<u8>,
}

//fn parse_data(raw: &mut hnd_data_t) -> Result<Vec<u8>, io::Error> {}

fn read_hnd_data(f: &mut File) -> Result<(Box<hnd_data_t>), io::Error> {
    let raw_header = read_header_to_raw(f)?;
    let header = parse_header(&raw_header)?;

    let w = header.SizeX;
    let h = header.SizeY;
    let len = w * h;
    let mut buf = hnd_data_t { data: Vec::new() };

    // Skip HND header
    let n = f.seek(SeekFrom::Start(1024));
    let s = f.read_to_end(&mut buf.data)?;

    Ok(Box::new(buf))
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
        // test hnd file
        let test_file_1 = String::from("test/test_data_1.hnd");
        let mut f_test = std::fs::File::open(test_file_1).unwrap();

        // raw file to compare with
        let raw_file_1 = String::from("test/test_data_1.raw");
        let mut f_raw = std::fs::File::open(raw_file_1).unwrap();

        let mut f_out = tempfile::tempfile().unwrap();
    }
}
