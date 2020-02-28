use std::convert::{From, Into, TryFrom, TryInto};

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct hnd_header_t {
    pub sFileType: String, //[u8; 32],
    pub FileLength: u32,
    pub chasChecksumSpec: String, //[u8; 4],
    pub nCheckSum: u32,
    pub sCreationDate: String, //[u8; 8],
    pub sCreationTime: String, //[u8; 8],
    pub sPatientID: String,    //[u8; 16],
    pub nPatientSer: u32,
    pub sSeriesID: String, //[u8; 16],
    pub nSeriesSer: u32,
    pub sSliceID: String, //[u8; 16],
    pub nSliceSer: u32,
    pub SizeX: u32,
    pub SizeY: u32,
    pub dSliceZPos: f64,
    pub sModality: String, //[u8; 16],
    pub nWindow: u32,
    pub nLevel: u32,
    pub nPixelOffset: u32,
    pub sImageType: String, //[u8; 4],
    pub dGantryRtn: f64,
    pub dSAD: f64,
    pub dSFD: f64,
    pub dCollX1: f64,
    pub dCollX2: f64,
    pub dCollY1: f64,
    pub dCollY2: f64,
    pub dCollRtn: f64,
    pub dFieldX: f64,
    pub dFieldY: f64,
    pub dBladeX1: f64,
    pub dBladeX2: f64,
    pub dBladeY1: f64,
    pub dBladeY2: f64,
    pub dIDUPosLng: f64,
    pub dIDUPosLat: f64,
    pub dIDUPosVrt: f64,
    pub dIDUPosRtn: f64,
    pub dPatientSupportAngle: f64,
    pub dTableTopEccentricAngle: f64,
    pub dCouchVrt: f64,
    pub dCouchLng: f64,
    pub dCouchLat: f64,
    pub dIDUResolutionX: f64,
    pub dIDUResolutionY: f64,
    pub dImageResolutionX: f64,
    pub dImageResolutionY: f64,
    pub dEnergy: f64,
    pub dDoseRate: f64,
    pub dXRayKV: f64,
    pub dXRayMA: f64,
    pub dMetersetExposure: f64,
    pub dAcqAdjustment: f64,
    pub dCTProjectionAngle: f64,
    pub dCTNormChamber: f64,
    pub dGatingTimeTag: f64,
    pub dGating4DInfoX: f64,
    pub dGating4DInfoY: f64,
    pub dGating4DInfoZ: f64,
    pub dGating4DInfoTime: f64,
}

pub type hnd_header_buf_t = [u8; 1024];

impl hnd_header_t {
    pub fn new() -> hnd_header_t {
        hnd_header_t {
            ..Default::default()
        }
    }
 
    pub fn to_slice_buf(&self) -> hnd_header_buf_t {
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

    pub fn from_slice_buf(raw_header: &hnd_header_buf_t) -> hnd_header_t {
        let mut pos: usize = 0;
        let mut buf = Buf::from(&raw_header[..1024]);
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