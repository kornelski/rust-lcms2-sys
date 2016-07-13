#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::os::raw::{c_char, c_int, c_long, c_void};
use libc::{wchar_t, tm, FILE};

pub type cmsSignature = u32;
pub type cmsS15Fixed16Number = i32;
pub type cmsBool = c_int;


// D50 XYZ normalized to Y=1.0
pub const cmsD50X:f64 =             0.9642;
pub const cmsD50Y:f64 =             1.0;
pub const cmsD50Z:f64 =             0.8249;

// V4 perceptual black
pub const cmsPERCEPTUAL_BLACK_X:f64 =  0.00336;
pub const cmsPERCEPTUAL_BLACK_Y:f64 =  0.0034731;
pub const cmsPERCEPTUAL_BLACK_Z:f64 =  0.00287;

// Definitions in ICC spec
pub const cmsMagicNumber:u32 =      0x61637370;     // 'acsp'
pub const lcmsSignature:u32 =       0x6c636d73;     // 'lcms'


#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsTagTypeSignature {
    cmsSigChromaticityType                  = 0x6368726D,  // 'chrm'
    cmsSigColorantOrderType                 = 0x636C726F,  // 'clro'
    cmsSigColorantTableType                 = 0x636C7274,  // 'clrt'
    cmsSigCrdInfoType                       = 0x63726469,  // 'crdi'
    cmsSigCurveType                         = 0x63757276,  // 'curv'
    cmsSigDataType                          = 0x64617461,  // 'data'
    cmsSigDictType                          = 0x64696374,  // 'dict'
    cmsSigDateTimeType                      = 0x6474696D,  // 'dtim'
    cmsSigDeviceSettingsType                = 0x64657673,  // 'devs'
    cmsSigLut16Type                         = 0x6d667432,  // 'mft2'
    cmsSigLut8Type                          = 0x6d667431,  // 'mft1'
    cmsSigLutAtoBType                       = 0x6d414220,  // 'mAB '
    cmsSigLutBtoAType                       = 0x6d424120,  // 'mBA '
    cmsSigMeasurementType                   = 0x6D656173,  // 'meas'
    cmsSigMultiLocalizedUnicodeType         = 0x6D6C7563,  // 'mluc'
    cmsSigMultiProcessElementType           = 0x6D706574,  // 'mpet'
    cmsSigNamedColorType                    = 0x6E636f6C,  // 'ncol' -- DEPRECATED!
    cmsSigNamedColor2Type                   = 0x6E636C32,  // 'ncl2'
    cmsSigParametricCurveType               = 0x70617261,  // 'para'
    cmsSigProfileSequenceDescType           = 0x70736571,  // 'pseq'
    cmsSigProfileSequenceIdType             = 0x70736964,  // 'psid'
    cmsSigResponseCurveSet16Type            = 0x72637332,  // 'rcs2'
    cmsSigS15Fixed16ArrayType               = 0x73663332,  // 'sf32'
    cmsSigScreeningType                     = 0x7363726E,  // 'scrn'
    cmsSigSignatureType                     = 0x73696720,  // 'sig '
    cmsSigTextType                          = 0x74657874,  // 'text'
    cmsSigTextDescriptionType               = 0x64657363,  // 'desc'
    cmsSigU16Fixed16ArrayType               = 0x75663332,  // 'uf32'
    cmsSigUcrBgType                         = 0x62666420,  // 'bfd '
    cmsSigUInt16ArrayType                   = 0x75693136,  // 'ui16'
    cmsSigUInt32ArrayType                   = 0x75693332,  // 'ui32'
    cmsSigUInt64ArrayType                   = 0x75693634,  // 'ui64'
    cmsSigUInt8ArrayType                    = 0x75693038,  // 'ui08'
    cmsSigVcgtType                          = 0x76636774,  // 'vcgt'
    cmsSigViewingConditionsType             = 0x76696577,  // 'view'
    cmsSigXYZType                           = 0x58595A20   // 'XYZ '
}

pub const cmsSigBlueMatrixColumnTag: cmsTagSignature = cmsTagSignature::cmsSigBlueColorantTag;
pub const cmsSigGreenMatrixColumnTag: cmsTagSignature = cmsTagSignature::cmsSigGreenColorantTag;
pub const cmsSigRedMatrixColumnTag: cmsTagSignature = cmsTagSignature::cmsSigRedColorantTag;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsTagSignature {
    cmsSigAToB0Tag                          = 0x41324230,  // 'A2B0'
    cmsSigAToB1Tag                          = 0x41324231,  // 'A2B1'
    cmsSigAToB2Tag                          = 0x41324232,  // 'A2B2'
    cmsSigBlueColorantTag                   = 0x6258595A,  // 'bXYZ'
    // cmsSigBlueMatrixColumnTag               = 0x6258595A,  // 'bXYZ'
    cmsSigBlueTRCTag                        = 0x62545243,  // 'bTRC'
    cmsSigBToA0Tag                          = 0x42324130,  // 'B2A0'
    cmsSigBToA1Tag                          = 0x42324131,  // 'B2A1'
    cmsSigBToA2Tag                          = 0x42324132,  // 'B2A2'
    cmsSigCalibrationDateTimeTag            = 0x63616C74,  // 'calt'
    cmsSigCharTargetTag                     = 0x74617267,  // 'targ'
    cmsSigChromaticAdaptationTag            = 0x63686164,  // 'chad'
    cmsSigChromaticityTag                   = 0x6368726D,  // 'chrm'
    cmsSigColorantOrderTag                  = 0x636C726F,  // 'clro'
    cmsSigColorantTableTag                  = 0x636C7274,  // 'clrt'
    cmsSigColorantTableOutTag               = 0x636C6F74,  // 'clot'
    cmsSigColorimetricIntentImageStateTag   = 0x63696973,  // 'ciis'
    cmsSigCopyrightTag                      = 0x63707274,  // 'cprt'
    cmsSigCrdInfoTag                        = 0x63726469,  // 'crdi'
    cmsSigDataTag                           = 0x64617461,  // 'data'
    cmsSigDateTimeTag                       = 0x6474696D,  // 'dtim'
    cmsSigDeviceMfgDescTag                  = 0x646D6E64,  // 'dmnd'
    cmsSigDeviceModelDescTag                = 0x646D6464,  // 'dmdd'
    cmsSigDeviceSettingsTag                 = 0x64657673,  // 'devs'
    cmsSigDToB0Tag                          = 0x44324230,  // 'D2B0'
    cmsSigDToB1Tag                          = 0x44324231,  // 'D2B1'
    cmsSigDToB2Tag                          = 0x44324232,  // 'D2B2'
    cmsSigDToB3Tag                          = 0x44324233,  // 'D2B3'
    cmsSigBToD0Tag                          = 0x42324430,  // 'B2D0'
    cmsSigBToD1Tag                          = 0x42324431,  // 'B2D1'
    cmsSigBToD2Tag                          = 0x42324432,  // 'B2D2'
    cmsSigBToD3Tag                          = 0x42324433,  // 'B2D3'
    cmsSigGamutTag                          = 0x67616D74,  // 'gamt'
    cmsSigGrayTRCTag                        = 0x6b545243,  // 'kTRC'
    cmsSigGreenColorantTag                  = 0x6758595A,  // 'gXYZ'
    // cmsSigGreenMatrixColumnTag              = 0x6758595A,  // 'gXYZ'
    cmsSigGreenTRCTag                       = 0x67545243,  // 'gTRC'
    cmsSigLuminanceTag                      = 0x6C756d69,  // 'lumi'
    cmsSigMeasurementTag                    = 0x6D656173,  // 'meas'
    cmsSigMediaBlackPointTag                = 0x626B7074,  // 'bkpt'
    cmsSigMediaWhitePointTag                = 0x77747074,  // 'wtpt'
    cmsSigNamedColorTag                     = 0x6E636f6C,  // 'ncol' // Deprecated by the ICC
    cmsSigNamedColor2Tag                    = 0x6E636C32,  // 'ncl2'
    cmsSigOutputResponseTag                 = 0x72657370,  // 'resp'
    cmsSigPerceptualRenderingIntentGamutTag = 0x72696730,  // 'rig0'
    cmsSigPreview0Tag                       = 0x70726530,  // 'pre0'
    cmsSigPreview1Tag                       = 0x70726531,  // 'pre1'
    cmsSigPreview2Tag                       = 0x70726532,  // 'pre2'
    cmsSigProfileDescriptionTag             = 0x64657363,  // 'desc'
    cmsSigProfileDescriptionMLTag           = 0x6473636d,  // 'dscm'
    cmsSigProfileSequenceDescTag            = 0x70736571,  // 'pseq'
    cmsSigProfileSequenceIdTag              = 0x70736964,  // 'psid'
    cmsSigPs2CRD0Tag                        = 0x70736430,  // 'psd0'
    cmsSigPs2CRD1Tag                        = 0x70736431,  // 'psd1'
    cmsSigPs2CRD2Tag                        = 0x70736432,  // 'psd2'
    cmsSigPs2CRD3Tag                        = 0x70736433,  // 'psd3'
    cmsSigPs2CSATag                         = 0x70733273,  // 'ps2s'
    cmsSigPs2RenderingIntentTag             = 0x70733269,  // 'ps2i'
    cmsSigRedColorantTag                    = 0x7258595A,  // 'rXYZ'
    // cmsSigRedMatrixColumnTag                = 0x7258595A,  // 'rXYZ'
    cmsSigRedTRCTag                         = 0x72545243,  // 'rTRC'
    cmsSigSaturationRenderingIntentGamutTag = 0x72696732,  // 'rig2'
    cmsSigScreeningDescTag                  = 0x73637264,  // 'scrd'
    cmsSigScreeningTag                      = 0x7363726E,  // 'scrn'
    cmsSigTechnologyTag                     = 0x74656368,  // 'tech'
    cmsSigUcrBgTag                          = 0x62666420,  // 'bfd '
    cmsSigViewingCondDescTag                = 0x76756564,  // 'vued'
    cmsSigViewingConditionsTag              = 0x76696577,  // 'view'
    cmsSigVcgtTag                           = 0x76636774,  // 'vcgt'
    cmsSigMetaTag                           = 0x6D657461   // 'meta'
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsTechnologySignature {
    cmsSigDigitalCamera                     = 0x6463616D,  // 'dcam'
    cmsSigFilmScanner                       = 0x6673636E,  // 'fscn'
    cmsSigReflectiveScanner                 = 0x7273636E,  // 'rscn'
    cmsSigInkJetPrinter                     = 0x696A6574,  // 'ijet'
    cmsSigThermalWaxPrinter                 = 0x74776178,  // 'twax'
    cmsSigElectrophotographicPrinter        = 0x6570686F,  // 'epho'
    cmsSigElectrostaticPrinter              = 0x65737461,  // 'esta'
    cmsSigDyeSublimationPrinter             = 0x64737562,  // 'dsub'
    cmsSigPhotographicPaperPrinter          = 0x7270686F,  // 'rpho'
    cmsSigFilmWriter                        = 0x6670726E,  // 'fprn'
    cmsSigVideoMonitor                      = 0x7669646D,  // 'vidm'
    cmsSigVideoCamera                       = 0x76696463,  // 'vidc'
    cmsSigProjectionTelevision              = 0x706A7476,  // 'pjtv'
    cmsSigCRTDisplay                        = 0x43525420,  // 'CRT '
    cmsSigPMDisplay                         = 0x504D4420,  // 'PMD '
    cmsSigAMDisplay                         = 0x414D4420,  // 'AMD '
    cmsSigPhotoCD                           = 0x4B504344,  // 'KPCD'
    cmsSigPhotoImageSetter                  = 0x696D6773,  // 'imgs'
    cmsSigGravure                           = 0x67726176,  // 'grav'
    cmsSigOffsetLithography                 = 0x6F666673,  // 'offs'
    cmsSigSilkscreen                        = 0x73696C6B,  // 'silk'
    cmsSigFlexography                       = 0x666C6578,  // 'flex'
    cmsSigMotionPictureFilmScanner          = 0x6D706673,  // 'mpfs'
    cmsSigMotionPictureFilmRecorder         = 0x6D706672,  // 'mpfr'
    cmsSigDigitalMotionPictureCamera        = 0x646D7063,  // 'dmpc'
    cmsSigDigitalCinemaProjector            = 0x64636A70   // 'dcpj'
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsColorSpaceSignature {
    cmsSigXYZData                           = 0x58595A20,  // 'XYZ '
    cmsSigLabData                           = 0x4C616220,  // 'Lab '
    cmsSigLuvData                           = 0x4C757620,  // 'Luv '
    cmsSigYCbCrData                         = 0x59436272,  // 'YCbr'
    cmsSigYxyData                           = 0x59787920,  // 'Yxy '
    cmsSigRgbData                           = 0x52474220,  // 'RGB '
    cmsSigGrayData                          = 0x47524159,  // 'GRAY'
    cmsSigHsvData                           = 0x48535620,  // 'HSV '
    cmsSigHlsData                           = 0x484C5320,  // 'HLS '
    cmsSigCmykData                          = 0x434D594B,  // 'CMYK'
    cmsSigCmyData                           = 0x434D5920,  // 'CMY '
    cmsSigMCH1Data                          = 0x4D434831,  // 'MCH1'
    cmsSigMCH2Data                          = 0x4D434832,  // 'MCH2'
    cmsSigMCH3Data                          = 0x4D434833,  // 'MCH3'
    cmsSigMCH4Data                          = 0x4D434834,  // 'MCH4'
    cmsSigMCH5Data                          = 0x4D434835,  // 'MCH5'
    cmsSigMCH6Data                          = 0x4D434836,  // 'MCH6'
    cmsSigMCH7Data                          = 0x4D434837,  // 'MCH7'
    cmsSigMCH8Data                          = 0x4D434838,  // 'MCH8'
    cmsSigMCH9Data                          = 0x4D434839,  // 'MCH9'
    cmsSigMCHAData                          = 0x4D434841,  // 'MCHA'
    cmsSigMCHBData                          = 0x4D434842,  // 'MCHB'
    cmsSigMCHCData                          = 0x4D434843,  // 'MCHC'
    cmsSigMCHDData                          = 0x4D434844,  // 'MCHD'
    cmsSigMCHEData                          = 0x4D434845,  // 'MCHE'
    cmsSigMCHFData                          = 0x4D434846,  // 'MCHF'
    cmsSigNamedData                         = 0x6e6d636c,  // 'nmcl'
    cmsSig1colorData                        = 0x31434C52,  // '1CLR'
    cmsSig2colorData                        = 0x32434C52,  // '2CLR'
    cmsSig3colorData                        = 0x33434C52,  // '3CLR'
    cmsSig4colorData                        = 0x34434C52,  // '4CLR'
    cmsSig5colorData                        = 0x35434C52,  // '5CLR'
    cmsSig6colorData                        = 0x36434C52,  // '6CLR'
    cmsSig7colorData                        = 0x37434C52,  // '7CLR'
    cmsSig8colorData                        = 0x38434C52,  // '8CLR'
    cmsSig9colorData                        = 0x39434C52,  // '9CLR'
    cmsSig10colorData                       = 0x41434C52,  // 'ACLR'
    cmsSig11colorData                       = 0x42434C52,  // 'BCLR'
    cmsSig12colorData                       = 0x43434C52,  // 'CCLR'
    cmsSig13colorData                       = 0x44434C52,  // 'DCLR'
    cmsSig14colorData                       = 0x45434C52,  // 'ECLR'
    cmsSig15colorData                       = 0x46434C52,  // 'FCLR'
    cmsSigLuvKData                          = 0x4C75764B   // 'LuvK'
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsProfileClassSignature {
    cmsSigInputClass                        = 0x73636E72,  // 'scnr'
    cmsSigDisplayClass                      = 0x6D6E7472,  // 'mntr'
    cmsSigOutputClass                       = 0x70727472,  // 'prtr'
    cmsSigLinkClass                         = 0x6C696E6B,  // 'link'
    cmsSigAbstractClass                     = 0x61627374,  // 'abst'
    cmsSigColorSpaceClass                   = 0x73706163,  // 'spac'
    cmsSigNamedColorClass                   = 0x6e6d636c   // 'nmcl'
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsPlatformSignature {
    cmsSigMacintosh                         = 0x4150504C,  // 'APPL'
    cmsSigMicrosoft                         = 0x4D534654,  // 'MSFT'
    cmsSigSolaris                           = 0x53554E57,  // 'SUNW'
    cmsSigSGI                               = 0x53474920,  // 'SGI '
    cmsSigTaligent                          = 0x54474E54,  // 'TGNT'
    cmsSigUnices                            = 0x2A6E6978   // '*nix'   // From argyll -- Not official
}

pub const cmsSigPerceptualReferenceMediumGamut:u32 =         0x70726d67;  //'prmg'

// For cmsSigColorimetricIntentImageStateTag
pub const cmsSigSceneColorimetryEstimates:u32 =              0x73636F65;  //'scoe'
pub const cmsSigSceneAppearanceEstimates:u32 =               0x73617065;  //'sape'
pub const cmsSigFocalPlaneColorimetryEstimates:u32 =         0x66706365;  //'fpce'
pub const cmsSigReflectionHardcopyOriginalColorimetry:u32 =  0x72686F63;  //'rhoc'
pub const cmsSigReflectionPrintOutputColorimetry:u32 =       0x72706F63;  //'rpoc'

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsStageSignature {
    cmsSigCurveSetElemType              = 0x63767374,  //'cvst'
    cmsSigMatrixElemType                = 0x6D617466,  //'matf'
    cmsSigCLutElemType                  = 0x636C7574,  //'clut'

    cmsSigBAcsElemType                  = 0x62414353,  // 'bACS'
    cmsSigEAcsElemType                  = 0x65414353,  // 'eACS'

    // Custom from here, not in the ICC Spec
    cmsSigXYZ2LabElemType               = 0x6C327820,  // 'l2x '
    cmsSigLab2XYZElemType               = 0x78326C20,  // 'x2l '
    cmsSigNamedColorElemType            = 0x6E636C20,  // 'ncl '
    cmsSigLabV2toV4                     = 0x32203420,  // '2 4 '
    cmsSigLabV4toV2                     = 0x34203220,  // '4 2 '

    // Identities
    cmsSigIdentityElemType              = 0x69646E20,  // 'idn '

    // Float to floatPCS
    cmsSigLab2FloatPCS                  = 0x64326C20,  // 'd2l '
    cmsSigFloatPCS2Lab                  = 0x6C326420,  // 'l2d '
    cmsSigXYZ2FloatPCS                  = 0x64327820,  // 'd2x '
    cmsSigFloatPCS2XYZ                  = 0x78326420,  // 'x2d '
    cmsSigClipNegativesElemType         = 0x636c7020   // 'clp '
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsCurveSegSignature {
    cmsSigFormulaCurveSeg               = 0x70617266, // 'parf'
    cmsSigSampledCurveSeg               = 0x73616D66, // 'samf'
    cmsSigSegmentedCurve                = 0x63757266  // 'curf'
}

pub const cmsSigStatusA:u32 =                    0x53746141; //'StaA'
pub const cmsSigStatusE:u32 =                    0x53746145; //'StaE'
pub const cmsSigStatusI:u32 =                    0x53746149; //'StaI'
pub const cmsSigStatusT:u32 =                    0x53746154; //'StaT'
pub const cmsSigStatusM:u32 =                    0x5374614D; //'StaM'
pub const cmsSigDN:u32 =                         0x444E2020; //'DN  '
pub const cmsSigDNP:u32 =                        0x444E2050; //'DN P'
pub const cmsSigDNN:u32 =                        0x444E4E20; //'DNN '
pub const cmsSigDNNP:u32 =                       0x444E4E50; //'DNNP'

// Device attributes, currently defined values correspond to the low 4 bytes
// of the 8 byte attribute quantity
pub const cmsReflective:u32 =     0;
pub const cmsTransparency:u32 =   1;
pub const cmsGlossy:u32 =         0;
pub const cmsMatte:u32 =          2;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsICCData {
    pub len: u32,
    pub flag: u32,
    pub data: [u8; 1usize],
}

impl ::std::default::Default for cmsICCData {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsDateTimeNumber {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}
impl ::std::default::Default for cmsDateTimeNumber {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsEncodedXYZNumber {
    pub X: cmsS15Fixed16Number,
    pub Y: cmsS15Fixed16Number,
    pub Z: cmsS15Fixed16Number,
}
impl ::std::default::Default for cmsEncodedXYZNumber {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsProfileID {
    pub _bindgen_data_: [u32; 4usize],
}
impl cmsProfileID {
    pub unsafe fn ID8(&mut self) -> *mut [u8; 16usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ID16(&mut self) -> *mut [u16; 8usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ID32(&mut self) -> *mut [u32; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for cmsProfileID {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsICCHeader {
    pub size: u32,
    pub cmmId: cmsSignature,
    pub version: u32,
    pub deviceClass: cmsProfileClassSignature,
    pub colorSpace: cmsColorSpaceSignature,
    pub pcs: cmsColorSpaceSignature,
    pub date: cmsDateTimeNumber,
    pub magic: cmsSignature,
    pub platform: cmsPlatformSignature,
    pub flags: u32,
    pub manufacturer: cmsSignature,
    pub model: u32,
    pub attributes: u64,
    pub renderingIntent: u32,
    pub illuminant: cmsEncodedXYZNumber,
    pub creator: cmsSignature,
    pub profileID: cmsProfileID,
    pub reserved: [i8; 28usize],
}
impl ::std::default::Default for cmsICCHeader {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsTagBase {
    pub sig: cmsTagTypeSignature,
    pub reserved: [i8; 4usize],
}
impl ::std::default::Default for cmsTagBase {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsTagEntry {
    pub sig: cmsTagSignature,
    pub offset: u32,
    pub size: u32,
}
impl ::std::default::Default for cmsTagEntry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type cmsHANDLE = *mut c_void;
pub type cmsHPROFILE = *mut c_void;
pub type cmsHTRANSFORM = *mut c_void;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
impl ::std::default::Default for cmsCIEXYZ {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}
impl ::std::default::Default for cmsCIExyY {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}
impl ::std::default::Default for cmsCIELab {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCIELCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}
impl ::std::default::Default for cmsCIELCh {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsJCh {
    pub J: f64,
    pub C: f64,
    pub h: f64,
}
impl ::std::default::Default for cmsJCh {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCIEXYZTRIPLE {
    pub Red: cmsCIEXYZ,
    pub Green: cmsCIEXYZ,
    pub Blue: cmsCIEXYZ,
}
impl ::std::default::Default for cmsCIEXYZTRIPLE {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCIExyYTRIPLE {
    pub Red: cmsCIExyY,
    pub Green: cmsCIExyY,
    pub Blue: cmsCIExyY,
}
impl ::std::default::Default for cmsCIExyYTRIPLE {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsICCMeasurementConditions {
    pub Observer: u32,
    pub Backing: cmsCIEXYZ,
    pub Geometry: u32,
    pub Flare: f64,
    pub IlluminantType: u32,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for cmsICCMeasurementConditions {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsICCViewingConditions {
    pub IlluminantXYZ: cmsCIEXYZ,
    pub SurroundXYZ: cmsCIEXYZ,
    pub IlluminantType: u32,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for cmsICCViewingConditions {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum _cmsContext_struct { }
pub type cmsContext = *mut _cmsContext_struct;
pub type cmsLogErrorHandlerFunction =
    ::std::option::Option<unsafe extern "C" fn(ContextID: cmsContext,
                                               ErrorCode: u32,
                                               Text:
                                                   *const c_char)>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsViewingConditions {
    pub whitePoint: cmsCIEXYZ,
    pub Yb: f64,
    pub La: f64,
    pub surround: c_int,
    pub D_value: f64,
}
impl ::std::default::Default for cmsViewingConditions {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCurveSegment {
    pub x0: f32,
    pub x1: f32,
    pub Type: i32,
    pub Params: [f64; 10usize],
    pub nGridPoints: u32,
    pub SampledPoints: *mut f32,
}
impl ::std::default::Default for cmsCurveSegment {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum _cms_curve_struct { }
pub type cmsToneCurve = _cms_curve_struct;
pub enum _cmsPipeline_struct { }
pub type cmsPipeline = _cmsPipeline_struct;
pub enum _cmsStage_struct { }
pub type cmsStage = _cmsStage_struct;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsStageLoc { cmsAT_BEGIN = 0, cmsAT_END = 1, }
pub type cmsSAMPLER16 =
    ::std::option::Option<unsafe extern "C" fn(In: *mut u16,
                                               Out: *mut u16,
                                               Cargo:
                                                   *mut c_void)
                              -> i32>;
pub type cmsSAMPLERFLOAT =
    ::std::option::Option<unsafe extern "C" fn(In: *mut f32,
                                               Out: *mut f32,
                                               Cargo:
                                                   *mut c_void)
                              -> i32>;
pub enum _cms_MLU_struct { }
pub type cmsMLU = _cms_MLU_struct;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsUcrBg {
    pub Ucr: *mut cmsToneCurve,
    pub Bg: *mut cmsToneCurve,
    pub Desc: *mut cmsMLU,
}
impl ::std::default::Default for cmsUcrBg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsScreeningChannel {
    pub Frequency: f64,
    pub ScreenAngle: f64,
    pub SpotShape: u32,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for cmsScreeningChannel {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsScreening {
    pub Flag: u32,
    pub nChannels: u32,
    pub Channels: [cmsScreeningChannel; 16usize],
}
impl ::std::default::Default for cmsScreening {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum _cms_NAMEDCOLORLIST_struct { }
pub type cmsNAMEDCOLORLIST = _cms_NAMEDCOLORLIST_struct;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsPSEQDESC {
    pub deviceMfg: cmsSignature,
    pub deviceModel: cmsSignature,
    pub attributes: u64,
    pub technology: cmsTechnologySignature,
    pub ProfileID: cmsProfileID,
    pub Manufacturer: *mut cmsMLU,
    pub Model: *mut cmsMLU,
    pub Description: *mut cmsMLU,
}
impl ::std::default::Default for cmsPSEQDESC {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsSEQ {
    pub n: u32,
    pub ContextID: cmsContext,
    pub seq: *mut cmsPSEQDESC,
}
impl ::std::default::Default for cmsSEQ {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _cmsDICTentry_struct {
    pub Next: *mut _cmsDICTentry_struct,
    pub DisplayName: *mut cmsMLU,
    pub DisplayValue: *mut cmsMLU,
    pub Name: *mut wchar_t,
    pub Value: *mut wchar_t,
}
impl ::std::default::Default for _cmsDICTentry_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type cmsDICTentry = _cmsDICTentry_struct;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsInfoType {
    cmsInfoDescription = 0,
    cmsInfoManufacturer = 1,
    cmsInfoModel = 2,
    cmsInfoCopyright = 3,
}
pub enum _cms_io_handler { }
pub type cmsIOHANDLER = _cms_io_handler;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsPSResourceType { cmsPS_RESOURCE_CSA = 0, cmsPS_RESOURCE_CRD = 1, }
extern "C" {
    pub fn cmsGetEncodedCMMversion() -> c_int;
    pub fn cmsstrcasecmp(s1: *const c_char,
                         s2: *const c_char)
     -> c_int;
    pub fn cmsfilelength(f: *mut FILE) -> c_long;
    pub fn cmsCreateContext(Plugin: *mut c_void,
                            UserData: *mut c_void)
     -> cmsContext;
    pub fn cmsDeleteContext(ContexID: cmsContext);
    pub fn cmsDupContext(ContextID: cmsContext,
                         NewUserData: *mut c_void)
     -> cmsContext;
    pub fn cmsGetContextUserData(ContextID: cmsContext)
     -> *mut c_void;
    pub fn cmsPlugin(Plugin: *mut c_void) -> cmsBool;
    pub fn cmsPluginTHR(ContextID: cmsContext,
                        Plugin: *mut c_void) -> cmsBool;
    pub fn cmsUnregisterPlugins();
    pub fn cmsUnregisterPluginsTHR(ContextID: cmsContext);
    pub fn cmsSetLogErrorHandler(Fn: cmsLogErrorHandlerFunction);
    pub fn cmsSetLogErrorHandlerTHR(ContextID: cmsContext,
                                    Fn: cmsLogErrorHandlerFunction);
    pub fn cmsD50_XYZ() -> *const cmsCIEXYZ;
    pub fn cmsD50_xyY() -> *const cmsCIExyY;
    pub fn cmsXYZ2xyY(Dest: *mut cmsCIExyY, Source: *const cmsCIEXYZ);
    pub fn cmsxyY2XYZ(Dest: *mut cmsCIEXYZ, Source: *const cmsCIExyY);
    pub fn cmsXYZ2Lab(WhitePoint: *const cmsCIEXYZ, Lab: *mut cmsCIELab,
                      xyz: *const cmsCIEXYZ);
    pub fn cmsLab2XYZ(WhitePoint: *const cmsCIEXYZ, xyz: *mut cmsCIEXYZ,
                      Lab: *const cmsCIELab);
    pub fn cmsLab2LCh(LCh: *mut cmsCIELCh, Lab: *const cmsCIELab);
    pub fn cmsLCh2Lab(Lab: *mut cmsCIELab, LCh: *const cmsCIELCh);
    pub fn cmsLabEncoded2Float(Lab: *mut cmsCIELab,
                               wLab: *mut u16);
    pub fn cmsLabEncoded2FloatV2(Lab: *mut cmsCIELab,
                                 wLab: *mut u16);
    pub fn cmsFloat2LabEncoded(wLab: *mut u16,
                               Lab: *const cmsCIELab);
    pub fn cmsFloat2LabEncodedV2(wLab: *mut u16,
                                 Lab: *const cmsCIELab);
    pub fn cmsXYZEncoded2Float(fxyz: *mut cmsCIEXYZ,
                               XYZ: *mut u16);
    pub fn cmsFloat2XYZEncoded(XYZ: *mut u16,
                               fXYZ: *const cmsCIEXYZ);
    pub fn cmsDeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab)
     -> f64;
    pub fn cmsCIE94DeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab)
     -> f64;
    pub fn cmsBFDdeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab)
     -> f64;
    pub fn cmsCMCdeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab,
                        l: f64, c: f64)
     -> f64;
    pub fn cmsCIE2000DeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab,
                            Kl: f64, Kc: f64,
                            Kh: f64) -> f64;
    pub fn cmsWhitePointFromTemp(WhitePoint: *mut cmsCIExyY,
                                 TempK: f64) -> cmsBool;
    pub fn cmsTempFromWhitePoint(TempK: *mut f64,
                                 WhitePoint: *const cmsCIExyY) -> cmsBool;
    pub fn cmsAdaptToIlluminant(Result: *mut cmsCIEXYZ,
                                SourceWhitePt: *const cmsCIEXYZ,
                                Illuminant: *const cmsCIEXYZ,
                                Value: *const cmsCIEXYZ) -> cmsBool;
    pub fn cmsCIECAM02Init(ContextID: cmsContext,
                           pVC: *const cmsViewingConditions) -> cmsHANDLE;
    pub fn cmsCIECAM02Done(hModel: cmsHANDLE);
    pub fn cmsCIECAM02Forward(hModel: cmsHANDLE, pIn: *const cmsCIEXYZ,
                              pOut: *mut cmsJCh);
    pub fn cmsCIECAM02Reverse(hModel: cmsHANDLE, pIn: *const cmsJCh,
                              pOut: *mut cmsCIEXYZ);
    pub fn cmsBuildSegmentedToneCurve(ContextID: cmsContext,
                                      nSegments: i32,
                                      Segments: *mut cmsCurveSegment)
     -> *mut cmsToneCurve;
    pub fn cmsBuildParametricToneCurve(ContextID: cmsContext,
                                       Type: i32,
                                       Params: *mut f64)
     -> *mut cmsToneCurve;
    pub fn cmsBuildGamma(ContextID: cmsContext, Gamma: f64)
     -> *mut cmsToneCurve;
    pub fn cmsBuildTabulatedToneCurve16(ContextID: cmsContext,
                                        nEntries: i32,
                                        values: *mut u16)
     -> *mut cmsToneCurve;
    pub fn cmsBuildTabulatedToneCurveFloat(ContextID: cmsContext,
                                           nEntries: u32,
                                           values: *mut f32)
     -> *mut cmsToneCurve;
    pub fn cmsFreeToneCurve(Curve: *mut cmsToneCurve);
    pub fn cmsFreeToneCurveTriple(Curve: *mut *mut cmsToneCurve);
    pub fn cmsDupToneCurve(Src: *const cmsToneCurve) -> *mut cmsToneCurve;
    pub fn cmsReverseToneCurve(InGamma: *const cmsToneCurve)
     -> *mut cmsToneCurve;
    pub fn cmsReverseToneCurveEx(nResultSamples: i32,
                                 InGamma: *const cmsToneCurve)
     -> *mut cmsToneCurve;
    pub fn cmsJoinToneCurve(ContextID: cmsContext, X: *const cmsToneCurve,
                            Y: *const cmsToneCurve, nPoints: u32)
     -> *mut cmsToneCurve;
    pub fn cmsSmoothToneCurve(Tab: *mut cmsToneCurve,
                              lambda: f64) -> cmsBool;
    pub fn cmsEvalToneCurveFloat(Curve: *const cmsToneCurve,
                                 v: f32) -> f32;
    pub fn cmsEvalToneCurve16(Curve: *const cmsToneCurve, v: u16)
     -> u16;
    pub fn cmsIsToneCurveMultisegment(InGamma: *const cmsToneCurve)
     -> cmsBool;
    pub fn cmsIsToneCurveLinear(Curve: *const cmsToneCurve) -> cmsBool;
    pub fn cmsIsToneCurveMonotonic(t: *const cmsToneCurve) -> cmsBool;
    pub fn cmsIsToneCurveDescending(t: *const cmsToneCurve) -> cmsBool;
    pub fn cmsGetToneCurveParametricType(t: *const cmsToneCurve)
     -> i32;
    pub fn cmsEstimateGamma(t: *const cmsToneCurve,
                            Precision: f64) -> f64;
    pub fn cmsGetToneCurveEstimatedTableEntries(t: *const cmsToneCurve)
     -> u32;
    pub fn cmsGetToneCurveEstimatedTable(t: *const cmsToneCurve)
     -> *const u16;
    pub fn cmsPipelineAlloc(ContextID: cmsContext,
                            InputChannels: u32,
                            OutputChannels: u32)
     -> *mut cmsPipeline;
    pub fn cmsPipelineFree(lut: *mut cmsPipeline);
    pub fn cmsPipelineDup(Orig: *const cmsPipeline) -> *mut cmsPipeline;
    pub fn cmsGetPipelineContextID(lut: *const cmsPipeline) -> cmsContext;
    pub fn cmsPipelineInputChannels(lut: *const cmsPipeline)
     -> u32;
    pub fn cmsPipelineOutputChannels(lut: *const cmsPipeline)
     -> u32;
    pub fn cmsPipelineStageCount(lut: *const cmsPipeline) -> u32;
    pub fn cmsPipelineGetPtrToFirstStage(lut: *const cmsPipeline)
     -> *mut cmsStage;
    pub fn cmsPipelineGetPtrToLastStage(lut: *const cmsPipeline)
     -> *mut cmsStage;
    pub fn cmsPipelineEval16(In: *mut u16,
                             Out: *mut u16,
                             lut: *const cmsPipeline);
    pub fn cmsPipelineEvalFloat(In: *mut f32,
                                Out: *mut f32,
                                lut: *const cmsPipeline);
    pub fn cmsPipelineEvalReverseFloat(Target: *mut f32,
                                       Result: *mut f32,
                                       Hint: *mut f32,
                                       lut: *const cmsPipeline) -> cmsBool;
    pub fn cmsPipelineCat(l1: *mut cmsPipeline, l2: *const cmsPipeline)
     -> cmsBool;
    pub fn cmsPipelineSetSaveAs8bitsFlag(lut: *mut cmsPipeline, On: cmsBool)
     -> cmsBool;
    pub fn cmsPipelineInsertStage(lut: *mut cmsPipeline, loc: cmsStageLoc,
                                  mpe: *mut cmsStage)
     -> c_int;
    pub fn cmsPipelineUnlinkStage(lut: *mut cmsPipeline, loc: cmsStageLoc,
                                  mpe: *mut *mut cmsStage);
    pub fn cmsPipelineCheckAndRetreiveStages(Lut: *const cmsPipeline,
                                             n: u32, ...)
     -> cmsBool;
    pub fn cmsStageAllocIdentity(ContextID: cmsContext,
                                 nChannels: u32) -> *mut cmsStage;
    pub fn cmsStageAllocToneCurves(ContextID: cmsContext,
                                   nChannels: u32,
                                   Curves: *mut *mut cmsToneCurve)
     -> *mut cmsStage;
    pub fn cmsStageAllocMatrix(ContextID: cmsContext, Rows: u32,
                               Cols: u32,
                               Matrix: *const f64,
                               Offset: *const f64)
     -> *mut cmsStage;
    pub fn cmsStageAllocCLut16bit(ContextID: cmsContext,
                                  nGridPoints: u32,
                                  inputChan: u32,
                                  outputChan: u32,
                                  Table: *const u16)
     -> *mut cmsStage;
    pub fn cmsStageAllocCLutFloat(ContextID: cmsContext,
                                  nGridPoints: u32,
                                  inputChan: u32,
                                  outputChan: u32,
                                  Table: *const f32)
     -> *mut cmsStage;
    pub fn cmsStageAllocCLut16bitGranular(ContextID: cmsContext,
                                          clutPoints: *mut u32,
                                          inputChan: u32,
                                          outputChan: u32,
                                          Table: *const u16)
     -> *mut cmsStage;
    pub fn cmsStageAllocCLutFloatGranular(ContextID: cmsContext,
                                          clutPoints: *mut u32,
                                          inputChan: u32,
                                          outputChan: u32,
                                          Table: *const f32)
     -> *mut cmsStage;
    pub fn cmsStageDup(mpe: *mut cmsStage) -> *mut cmsStage;
    pub fn cmsStageFree(mpe: *mut cmsStage);
    pub fn cmsStageNext(mpe: *const cmsStage) -> *mut cmsStage;
    pub fn cmsStageInputChannels(mpe: *const cmsStage) -> u32;
    pub fn cmsStageOutputChannels(mpe: *const cmsStage) -> u32;
    pub fn cmsStageType(mpe: *const cmsStage) -> cmsStageSignature;
    pub fn cmsStageData(mpe: *const cmsStage) -> *mut c_void;
    pub fn cmsStageSampleCLut16bit(mpe: *mut cmsStage, Sampler: cmsSAMPLER16,
                                   Cargo: *mut c_void,
                                   dwFlags: u32) -> cmsBool;
    pub fn cmsStageSampleCLutFloat(mpe: *mut cmsStage,
                                   Sampler: cmsSAMPLERFLOAT,
                                   Cargo: *mut c_void,
                                   dwFlags: u32) -> cmsBool;
    pub fn cmsSliceSpace16(nInputs: u32,
                           clutPoints: *mut u32,
                           Sampler: cmsSAMPLER16,
                           Cargo: *mut c_void) -> cmsBool;
    pub fn cmsSliceSpaceFloat(nInputs: u32,
                              clutPoints: *mut u32,
                              Sampler: cmsSAMPLERFLOAT,
                              Cargo: *mut c_void) -> cmsBool;
    pub fn cmsMLUalloc(ContextID: cmsContext, nItems: u32)
     -> *mut cmsMLU;
    pub fn cmsMLUfree(mlu: *mut cmsMLU);
    pub fn cmsMLUdup(mlu: *const cmsMLU) -> *mut cmsMLU;
    pub fn cmsMLUsetASCII(mlu: *mut cmsMLU,
                          LanguageCode: *mut c_char,
                          CountryCode: *mut c_char,
                          ASCIIString: *const c_char)
     -> cmsBool;
    pub fn cmsMLUsetWide(mlu: *mut cmsMLU,
                         LanguageCode: *mut c_char,
                         CountryCode: *mut c_char,
                         WideString: *const wchar_t) -> cmsBool;
    pub fn cmsMLUgetASCII(mlu: *const cmsMLU,
                          LanguageCode: *mut c_char,
                          CountryCode: *mut c_char,
                          Buffer: *mut c_char,
                          BufferSize: u32) -> u32;
    pub fn cmsMLUgetWide(mlu: *const cmsMLU,
                         LanguageCode: *mut c_char,
                         CountryCode: *mut c_char,
                         Buffer: *mut wchar_t, BufferSize: u32)
     -> u32;
    pub fn cmsMLUgetTranslation(mlu: *const cmsMLU,
                                LanguageCode: *mut c_char,
                                CountryCode: *mut c_char,
                                ObtainedLanguage: *mut c_char,
                                ObtainedCountry: *mut c_char)
     -> cmsBool;
    pub fn cmsMLUtranslationsCount(mlu: *const cmsMLU) -> u32;
    pub fn cmsMLUtranslationsCodes(mlu: *const cmsMLU, idx: u32,
                                   LanguageCode: *mut c_char,
                                   CountryCode: *mut c_char)
     -> cmsBool;
    pub fn cmsAllocNamedColorList(ContextID: cmsContext, n: u32,
                                  ColorantCount: u32,
                                  Prefix: *const c_char,
                                  Suffix: *const c_char)
     -> *mut cmsNAMEDCOLORLIST;
    pub fn cmsFreeNamedColorList(v: *mut cmsNAMEDCOLORLIST);
    pub fn cmsDupNamedColorList(v: *const cmsNAMEDCOLORLIST)
     -> *mut cmsNAMEDCOLORLIST;
    pub fn cmsAppendNamedColor(v: *mut cmsNAMEDCOLORLIST,
                               Name: *const c_char,
                               PCS: *mut u16,
                               Colorant: *mut u16) -> cmsBool;
    pub fn cmsNamedColorCount(v: *const cmsNAMEDCOLORLIST) -> u32;
    pub fn cmsNamedColorIndex(v: *const cmsNAMEDCOLORLIST,
                              Name: *const c_char)
     -> i32;
    pub fn cmsNamedColorInfo(NamedColorList: *const cmsNAMEDCOLORLIST,
                             nColor: u32,
                             Name: *mut c_char,
                             Prefix: *mut c_char,
                             Suffix: *mut c_char,
                             PCS: *mut u16,
                             Colorant: *mut u16) -> cmsBool;
    pub fn cmsGetNamedColorList(xform: cmsHTRANSFORM)
     -> *mut cmsNAMEDCOLORLIST;
    pub fn cmsAllocProfileSequenceDescription(ContextID: cmsContext,
                                              n: u32)
     -> *mut cmsSEQ;
    pub fn cmsDupProfileSequenceDescription(pseq: *const cmsSEQ)
     -> *mut cmsSEQ;
    pub fn cmsFreeProfileSequenceDescription(pseq: *mut cmsSEQ);
    pub fn cmsDictAlloc(ContextID: cmsContext) -> cmsHANDLE;
    pub fn cmsDictFree(hDict: cmsHANDLE);
    pub fn cmsDictDup(hDict: cmsHANDLE) -> cmsHANDLE;
    pub fn cmsDictAddEntry(hDict: cmsHANDLE, Name: *const wchar_t,
                           Value: *const wchar_t, DisplayName: *const cmsMLU,
                           DisplayValue: *const cmsMLU) -> cmsBool;
    pub fn cmsDictGetEntryList(hDict: cmsHANDLE) -> *const cmsDICTentry;
    pub fn cmsDictNextEntry(e: *const cmsDICTentry) -> *const cmsDICTentry;
    pub fn cmsCreateProfilePlaceholder(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsGetProfileContextID(hProfile: cmsHPROFILE) -> cmsContext;
    pub fn cmsGetTagCount(hProfile: cmsHPROFILE) -> i32;
    pub fn cmsGetTagSignature(hProfile: cmsHPROFILE, n: u32)
     -> cmsTagSignature;
    pub fn cmsIsTag(hProfile: cmsHPROFILE, sig: cmsTagSignature) -> cmsBool;
    pub fn cmsReadTag(hProfile: cmsHPROFILE, sig: cmsTagSignature)
     -> *mut c_void;
    pub fn cmsWriteTag(hProfile: cmsHPROFILE, sig: cmsTagSignature,
                       data: *const c_void) -> cmsBool;
    pub fn cmsLinkTag(hProfile: cmsHPROFILE, sig: cmsTagSignature,
                      dest: cmsTagSignature) -> cmsBool;
    pub fn cmsTagLinkedTo(hProfile: cmsHPROFILE, sig: cmsTagSignature)
     -> cmsTagSignature;
    pub fn cmsReadRawTag(hProfile: cmsHPROFILE, sig: cmsTagSignature,
                         Buffer: *mut c_void,
                         BufferSize: u32) -> i32;
    pub fn cmsWriteRawTag(hProfile: cmsHPROFILE, sig: cmsTagSignature,
                          data: *const c_void,
                          Size: u32) -> cmsBool;
    pub fn cmsGetHeaderFlags(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsGetHeaderAttributes(hProfile: cmsHPROFILE,
                                  Flags: *mut u64);
    pub fn cmsGetHeaderProfileID(hProfile: cmsHPROFILE,
                                 ProfileID: *mut u8);
    pub fn cmsGetHeaderCreationDateTime(hProfile: cmsHPROFILE, Dest: *mut tm)
     -> cmsBool;
    pub fn cmsGetHeaderRenderingIntent(hProfile: cmsHPROFILE)
     -> u32;
    pub fn cmsSetHeaderFlags(hProfile: cmsHPROFILE, Flags: u32);
    pub fn cmsGetHeaderManufacturer(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsSetHeaderManufacturer(hProfile: cmsHPROFILE,
                                    manufacturer: u32);
    pub fn cmsGetHeaderCreator(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsGetHeaderModel(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsSetHeaderModel(hProfile: cmsHPROFILE, model: u32);
    pub fn cmsSetHeaderAttributes(hProfile: cmsHPROFILE,
                                  Flags: u64);
    pub fn cmsSetHeaderProfileID(hProfile: cmsHPROFILE,
                                 ProfileID: *mut u8);
    pub fn cmsSetHeaderRenderingIntent(hProfile: cmsHPROFILE,
                                       RenderingIntent: u32);
    pub fn cmsGetPCS(hProfile: cmsHPROFILE) -> cmsColorSpaceSignature;
    pub fn cmsSetPCS(hProfile: cmsHPROFILE, pcs: cmsColorSpaceSignature);
    pub fn cmsGetColorSpace(hProfile: cmsHPROFILE) -> cmsColorSpaceSignature;
    pub fn cmsSetColorSpace(hProfile: cmsHPROFILE,
                            sig: cmsColorSpaceSignature);
    pub fn cmsGetDeviceClass(hProfile: cmsHPROFILE)
     -> cmsProfileClassSignature;
    pub fn cmsSetDeviceClass(hProfile: cmsHPROFILE,
                             sig: cmsProfileClassSignature);
    pub fn cmsSetProfileVersion(hProfile: cmsHPROFILE,
                                Version: f64);
    pub fn cmsGetProfileVersion(hProfile: cmsHPROFILE) -> f64;
    pub fn cmsGetEncodedICCversion(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsSetEncodedICCversion(hProfile: cmsHPROFILE,
                                   Version: u32);
    pub fn cmsIsIntentSupported(hProfile: cmsHPROFILE,
                                Intent: u32,
                                UsedDirection: u32) -> cmsBool;
    pub fn cmsIsMatrixShaper(hProfile: cmsHPROFILE) -> cmsBool;
    pub fn cmsIsCLUT(hProfile: cmsHPROFILE, Intent: u32,
                     UsedDirection: u32) -> cmsBool;
    pub fn _cmsICCcolorSpace(OurNotation: c_int)
     -> cmsColorSpaceSignature;
    pub fn _cmsLCMScolorSpace(ProfileSpace: cmsColorSpaceSignature)
     -> c_int;
    pub fn cmsChannelsOf(ColorSpace: cmsColorSpaceSignature)
     -> u32;
    pub fn cmsFormatterForColorspaceOfProfile(hProfile: cmsHPROFILE,
                                              nBytes: u32,
                                              lIsFloat: cmsBool)
     -> u32;
    pub fn cmsFormatterForPCSOfProfile(hProfile: cmsHPROFILE,
                                       nBytes: u32,
                                       lIsFloat: cmsBool) -> u32;
    pub fn cmsGetProfileInfo(hProfile: cmsHPROFILE, Info: cmsInfoType,
                             LanguageCode: *mut c_char,
                             CountryCode: *mut c_char,
                             Buffer: *mut wchar_t,
                             BufferSize: u32) -> u32;
    pub fn cmsGetProfileInfoASCII(hProfile: cmsHPROFILE, Info: cmsInfoType,
                                  LanguageCode: *mut c_char,
                                  CountryCode: *mut c_char,
                                  Buffer: *mut c_char,
                                  BufferSize: u32)
     -> u32;
    pub fn cmsOpenIOhandlerFromFile(ContextID: cmsContext,
                                    FileName: *const c_char,
                                    AccessMode: *const c_char)
     -> *mut cmsIOHANDLER;
    pub fn cmsOpenIOhandlerFromStream(ContextID: cmsContext,
                                      Stream: *mut FILE) -> *mut cmsIOHANDLER;
    pub fn cmsOpenIOhandlerFromMem(ContextID: cmsContext,
                                   Buffer: *mut c_void,
                                   size: u32,
                                   AccessMode: *const c_char)
     -> *mut cmsIOHANDLER;
    pub fn cmsOpenIOhandlerFromNULL(ContextID: cmsContext)
     -> *mut cmsIOHANDLER;
    pub fn cmsGetProfileIOhandler(hProfile: cmsHPROFILE) -> *mut cmsIOHANDLER;
    pub fn cmsCloseIOhandler(io: *mut cmsIOHANDLER) -> cmsBool;
    pub fn cmsMD5computeID(hProfile: cmsHPROFILE) -> cmsBool;
    pub fn cmsOpenProfileFromFile(ICCProfile: *const c_char,
                                  sAccess: *const c_char)
     -> cmsHPROFILE;
    pub fn cmsOpenProfileFromFileTHR(ContextID: cmsContext,
                                     ICCProfile:
                                         *const c_char,
                                     sAccess: *const c_char)
     -> cmsHPROFILE;
    pub fn cmsOpenProfileFromStream(ICCProfile: *mut FILE,
                                    sAccess: *const c_char)
     -> cmsHPROFILE;
    pub fn cmsOpenProfileFromStreamTHR(ContextID: cmsContext,
                                       ICCProfile: *mut FILE,
                                       sAccess: *const c_char)
     -> cmsHPROFILE;
    pub fn cmsOpenProfileFromMem(MemPtr: *const c_void,
                                 dwSize: u32) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromMemTHR(ContextID: cmsContext,
                                    MemPtr: *const c_void,
                                    dwSize: u32) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromIOhandlerTHR(ContextID: cmsContext,
                                          io: *mut cmsIOHANDLER)
     -> cmsHPROFILE;
    pub fn cmsOpenProfileFromIOhandler2THR(ContextID: cmsContext,
                                           io: *mut cmsIOHANDLER,
                                           write: cmsBool) -> cmsHPROFILE;
    pub fn cmsCloseProfile(hProfile: cmsHPROFILE) -> cmsBool;
    pub fn cmsSaveProfileToFile(hProfile: cmsHPROFILE,
                                FileName: *const c_char)
     -> cmsBool;
    pub fn cmsSaveProfileToStream(hProfile: cmsHPROFILE, Stream: *mut FILE)
     -> cmsBool;
    pub fn cmsSaveProfileToMem(hProfile: cmsHPROFILE,
                               MemPtr: *mut c_void,
                               BytesNeeded: *mut u32) -> cmsBool;
    pub fn cmsSaveProfileToIOhandler(hProfile: cmsHPROFILE,
                                     io: *mut cmsIOHANDLER)
     -> u32;
    pub fn cmsCreateRGBProfileTHR(ContextID: cmsContext,
                                  WhitePoint: *const cmsCIExyY,
                                  Primaries: *const cmsCIExyYTRIPLE,
                                  TransferFunction: *mut *mut cmsToneCurve)
     -> cmsHPROFILE;
    pub fn cmsCreateRGBProfile(WhitePoint: *const cmsCIExyY,
                               Primaries: *const cmsCIExyYTRIPLE,
                               TransferFunction: *mut *mut cmsToneCurve)
     -> cmsHPROFILE;
    pub fn cmsCreateGrayProfileTHR(ContextID: cmsContext,
                                   WhitePoint: *const cmsCIExyY,
                                   TransferFunction: *const cmsToneCurve)
     -> cmsHPROFILE;
    pub fn cmsCreateGrayProfile(WhitePoint: *const cmsCIExyY,
                                TransferFunction: *const cmsToneCurve)
     -> cmsHPROFILE;
    pub fn cmsCreateLinearizationDeviceLinkTHR(ContextID: cmsContext,
                                               ColorSpace:
                                                   cmsColorSpaceSignature,
                                               TransferFunctions:
                                                   *mut *mut cmsToneCurve)
     -> cmsHPROFILE;
    pub fn cmsCreateLinearizationDeviceLink(ColorSpace:
                                                cmsColorSpaceSignature,
                                            TransferFunctions:
                                                *mut *mut cmsToneCurve)
     -> cmsHPROFILE;
    pub fn cmsCreateInkLimitingDeviceLinkTHR(ContextID: cmsContext,
                                             ColorSpace:
                                                 cmsColorSpaceSignature,
                                             Limit: f64)
     -> cmsHPROFILE;
    pub fn cmsCreateInkLimitingDeviceLink(ColorSpace: cmsColorSpaceSignature,
                                          Limit: f64)
     -> cmsHPROFILE;
    pub fn cmsCreateLab2ProfileTHR(ContextID: cmsContext,
                                   WhitePoint: *const cmsCIExyY)
     -> cmsHPROFILE;
    pub fn cmsCreateLab2Profile(WhitePoint: *const cmsCIExyY) -> cmsHPROFILE;
    pub fn cmsCreateLab4ProfileTHR(ContextID: cmsContext,
                                   WhitePoint: *const cmsCIExyY)
     -> cmsHPROFILE;
    pub fn cmsCreateLab4Profile(WhitePoint: *const cmsCIExyY) -> cmsHPROFILE;
    pub fn cmsCreateXYZProfileTHR(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsCreateXYZProfile() -> cmsHPROFILE;
    pub fn cmsCreate_sRGBProfileTHR(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsCreate_sRGBProfile() -> cmsHPROFILE;
    pub fn cmsCreateBCHSWabstractProfileTHR(ContextID: cmsContext,
                                            nLUTPoints: c_int,
                                            Bright: f64,
                                            Contrast: f64,
                                            Hue: f64,
                                            Saturation: f64,
                                            TempSrc: c_int,
                                            TempDest: c_int)
     -> cmsHPROFILE;
    pub fn cmsCreateBCHSWabstractProfile(nLUTPoints: c_int,
                                         Bright: f64,
                                         Contrast: f64,
                                         Hue: f64,
                                         Saturation: f64,
                                         TempSrc: c_int,
                                         TempDest: c_int)
     -> cmsHPROFILE;
    pub fn cmsCreateNULLProfileTHR(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsCreateNULLProfile() -> cmsHPROFILE;
    pub fn cmsTransform2DeviceLink(hTransform: cmsHTRANSFORM,
                                   Version: f64,
                                   dwFlags: u32) -> cmsHPROFILE;
    pub fn cmsGetSupportedIntents(nMax: u32,
                                  Codes: *mut u32,
                                  Descriptions:
                                      *mut *mut c_char)
     -> u32;
    pub fn cmsGetSupportedIntentsTHR(ContextID: cmsContext,
                                     nMax: u32,
                                     Codes: *mut u32,
                                     Descriptions:
                                         *mut *mut c_char)
     -> u32;
    pub fn cmsCreateTransformTHR(ContextID: cmsContext, Input: cmsHPROFILE,
                                 InputFormat: u32,
                                 Output: cmsHPROFILE,
                                 OutputFormat: u32,
                                 Intent: u32,
                                 dwFlags: u32) -> cmsHTRANSFORM;
    pub fn cmsCreateTransform(Input: cmsHPROFILE,
                              InputFormat: u32,
                              Output: cmsHPROFILE,
                              OutputFormat: u32,
                              Intent: u32,
                              dwFlags: u32) -> cmsHTRANSFORM;
    pub fn cmsCreateProofingTransformTHR(ContextID: cmsContext,
                                         Input: cmsHPROFILE,
                                         InputFormat: u32,
                                         Output: cmsHPROFILE,
                                         OutputFormat: u32,
                                         Proofing: cmsHPROFILE,
                                         Intent: u32,
                                         ProofingIntent: u32,
                                         dwFlags: u32)
     -> cmsHTRANSFORM;
    pub fn cmsCreateProofingTransform(Input: cmsHPROFILE,
                                      InputFormat: u32,
                                      Output: cmsHPROFILE,
                                      OutputFormat: u32,
                                      Proofing: cmsHPROFILE,
                                      Intent: u32,
                                      ProofingIntent: u32,
                                      dwFlags: u32)
     -> cmsHTRANSFORM;
    pub fn cmsCreateMultiprofileTransformTHR(ContextID: cmsContext,
                                             hProfiles: *mut cmsHPROFILE,
                                             nProfiles: u32,
                                             InputFormat: u32,
                                             OutputFormat: u32,
                                             Intent: u32,
                                             dwFlags: u32)
     -> cmsHTRANSFORM;
    pub fn cmsCreateMultiprofileTransform(hProfiles: *mut cmsHPROFILE,
                                          nProfiles: u32,
                                          InputFormat: u32,
                                          OutputFormat: u32,
                                          Intent: u32,
                                          dwFlags: u32)
     -> cmsHTRANSFORM;
    pub fn cmsCreateExtendedTransform(ContextID: cmsContext,
                                      nProfiles: u32,
                                      hProfiles: *mut cmsHPROFILE,
                                      BPC: *mut cmsBool,
                                      Intents: *mut u32,
                                      AdaptationStates: *mut f64,
                                      hGamutProfile: cmsHPROFILE,
                                      nGamutPCSposition: u32,
                                      InputFormat: u32,
                                      OutputFormat: u32,
                                      dwFlags: u32)
     -> cmsHTRANSFORM;
    pub fn cmsDeleteTransform(hTransform: cmsHTRANSFORM);
    pub fn cmsDoTransform(Transform: cmsHTRANSFORM,
                          InputBuffer: *const c_void,
                          OutputBuffer: *mut c_void,
                          Size: u32);
    pub fn cmsDoTransformStride(Transform: cmsHTRANSFORM,
                                InputBuffer: *const c_void,
                                OutputBuffer: *mut c_void,
                                Size: u32,
                                Stride: u32);
    pub fn cmsSetAlarmCodes(NewAlarm: *mut u16);
    pub fn cmsGetAlarmCodes(NewAlarm: *mut u16);
    pub fn cmsSetAlarmCodesTHR(ContextID: cmsContext,
                               AlarmCodes: *mut u16);
    pub fn cmsGetAlarmCodesTHR(ContextID: cmsContext,
                               AlarmCodes: *mut u16);
    pub fn cmsSetAdaptationState(d: f64) -> f64;
    pub fn cmsSetAdaptationStateTHR(ContextID: cmsContext,
                                    d: f64) -> f64;
    pub fn cmsGetTransformContextID(hTransform: cmsHTRANSFORM) -> cmsContext;
    pub fn cmsGetTransformInputFormat(hTransform: cmsHTRANSFORM)
     -> u32;
    pub fn cmsGetTransformOutputFormat(hTransform: cmsHTRANSFORM)
     -> u32;
    pub fn cmsChangeBuffersFormat(hTransform: cmsHTRANSFORM,
                                  InputFormat: u32,
                                  OutputFormat: u32) -> cmsBool;
    pub fn cmsGetPostScriptColorResource(ContextID: cmsContext,
                                         Type: cmsPSResourceType,
                                         hProfile: cmsHPROFILE,
                                         Intent: u32,
                                         dwFlags: u32,
                                         io: *mut cmsIOHANDLER)
     -> u32;
    pub fn cmsGetPostScriptCSA(ContextID: cmsContext, hProfile: cmsHPROFILE,
                               Intent: u32,
                               dwFlags: u32,
                               Buffer: *mut c_void,
                               dwBufferLen: u32)
     -> u32;
    pub fn cmsGetPostScriptCRD(ContextID: cmsContext, hProfile: cmsHPROFILE,
                               Intent: u32,
                               dwFlags: u32,
                               Buffer: *mut c_void,
                               dwBufferLen: u32)
     -> u32;
    pub fn cmsIT8Alloc(ContextID: cmsContext) -> cmsHANDLE;
    pub fn cmsIT8Free(hIT8: cmsHANDLE);
    pub fn cmsIT8TableCount(hIT8: cmsHANDLE) -> u32;
    pub fn cmsIT8SetTable(hIT8: cmsHANDLE, nTable: u32)
     -> i32;
    pub fn cmsIT8LoadFromFile(ContextID: cmsContext,
                              cFileName: *const c_char)
     -> cmsHANDLE;
    pub fn cmsIT8LoadFromMem(ContextID: cmsContext,
                             Ptr: *mut c_void,
                             len: u32) -> cmsHANDLE;
    pub fn cmsIT8SaveToFile(hIT8: cmsHANDLE,
                            cFileName: *const c_char)
     -> cmsBool;
    pub fn cmsIT8SaveToMem(hIT8: cmsHANDLE,
                           MemPtr: *mut c_void,
                           BytesNeeded: *mut u32) -> cmsBool;
    pub fn cmsIT8GetSheetType(hIT8: cmsHANDLE)
     -> *const c_char;
    pub fn cmsIT8SetSheetType(hIT8: cmsHANDLE,
                              Type: *const c_char) -> cmsBool;
    pub fn cmsIT8SetComment(hIT8: cmsHANDLE,
                            cComment: *const c_char)
     -> cmsBool;
    pub fn cmsIT8SetPropertyStr(hIT8: cmsHANDLE,
                                cProp: *const c_char,
                                Str: *const c_char)
     -> cmsBool;
    pub fn cmsIT8SetPropertyDbl(hIT8: cmsHANDLE,
                                cProp: *const c_char,
                                Val: f64) -> cmsBool;
    pub fn cmsIT8SetPropertyHex(hIT8: cmsHANDLE,
                                cProp: *const c_char,
                                Val: u32) -> cmsBool;
    pub fn cmsIT8SetPropertyMulti(hIT8: cmsHANDLE,
                                  Key: *const c_char,
                                  SubKey: *const c_char,
                                  Buffer: *const c_char)
     -> cmsBool;
    pub fn cmsIT8SetPropertyUncooked(hIT8: cmsHANDLE,
                                     Key: *const c_char,
                                     Buffer: *const c_char)
     -> cmsBool;
    pub fn cmsIT8GetProperty(hIT8: cmsHANDLE,
                             cProp: *const c_char)
     -> *const c_char;
    pub fn cmsIT8GetPropertyDbl(hIT8: cmsHANDLE,
                                cProp: *const c_char)
     -> f64;
    pub fn cmsIT8GetPropertyMulti(hIT8: cmsHANDLE,
                                  Key: *const c_char,
                                  SubKey: *const c_char)
     -> *const c_char;
    pub fn cmsIT8EnumProperties(hIT8: cmsHANDLE,
                                PropertyNames:
                                    *mut *mut *mut c_char)
     -> u32;
    pub fn cmsIT8EnumPropertyMulti(hIT8: cmsHANDLE,
                                   cProp: *const c_char,
                                   SubpropertyNames:
                                       *mut *mut *const c_char)
     -> u32;
    pub fn cmsIT8GetDataRowCol(hIT8: cmsHANDLE, row: c_int,
                               col: c_int)
     -> *const c_char;
    pub fn cmsIT8GetDataRowColDbl(hIT8: cmsHANDLE, row: c_int,
                                  col: c_int)
     -> f64;
    pub fn cmsIT8SetDataRowCol(hIT8: cmsHANDLE, row: c_int,
                               col: c_int,
                               Val: *const c_char) -> cmsBool;
    pub fn cmsIT8SetDataRowColDbl(hIT8: cmsHANDLE, row: c_int,
                                  col: c_int,
                                  Val: f64) -> cmsBool;
    pub fn cmsIT8GetData(hIT8: cmsHANDLE,
                         cPatch: *const c_char,
                         cSample: *const c_char)
     -> *const c_char;
    pub fn cmsIT8GetDataDbl(hIT8: cmsHANDLE,
                            cPatch: *const c_char,
                            cSample: *const c_char)
     -> f64;
    pub fn cmsIT8SetData(hIT8: cmsHANDLE,
                         cPatch: *const c_char,
                         cSample: *const c_char,
                         Val: *const c_char) -> cmsBool;
    pub fn cmsIT8SetDataDbl(hIT8: cmsHANDLE,
                            cPatch: *const c_char,
                            cSample: *const c_char,
                            Val: f64) -> cmsBool;
    pub fn cmsIT8FindDataFormat(hIT8: cmsHANDLE,
                                cSample: *const c_char)
     -> c_int;
    pub fn cmsIT8SetDataFormat(hIT8: cmsHANDLE, n: c_int,
                               Sample: *const c_char)
     -> cmsBool;
    pub fn cmsIT8EnumDataFormat(hIT8: cmsHANDLE,
                                SampleNames:
                                    *mut *mut *mut c_char)
     -> c_int;
    pub fn cmsIT8GetPatchName(hIT8: cmsHANDLE, nPatch: c_int,
                              buffer: *mut c_char)
     -> *const c_char;
    pub fn cmsIT8GetPatchByName(hIT8: cmsHANDLE,
                                cPatch: *const c_char)
     -> c_int;
    pub fn cmsIT8SetTableByLabel(hIT8: cmsHANDLE,
                                 cSet: *const c_char,
                                 cField: *const c_char,
                                 ExpectedType: *const c_char)
     -> c_int;
    pub fn cmsIT8SetIndexColumn(hIT8: cmsHANDLE,
                                cSample: *const c_char)
     -> cmsBool;
    pub fn cmsIT8DefineDblFormat(hIT8: cmsHANDLE,
                                 Formatter: *const c_char);
    pub fn cmsGBDAlloc(ContextID: cmsContext) -> cmsHANDLE;
    pub fn cmsGBDFree(hGBD: cmsHANDLE);
    pub fn cmsGDBAddPoint(hGBD: cmsHANDLE, Lab: *const cmsCIELab) -> cmsBool;
    pub fn cmsGDBCompute(hGDB: cmsHANDLE, dwFlags: u32)
     -> cmsBool;
    pub fn cmsGDBCheckPoint(hGBD: cmsHANDLE, Lab: *const cmsCIELab)
     -> cmsBool;
    pub fn cmsDetectBlackPoint(BlackPoint: *mut cmsCIEXYZ,
                               hProfile: cmsHPROFILE, Intent: u32,
                               dwFlags: u32) -> cmsBool;
    pub fn cmsDetectDestinationBlackPoint(BlackPoint: *mut cmsCIEXYZ,
                                          hProfile: cmsHPROFILE,
                                          Intent: u32,
                                          dwFlags: u32)
     -> cmsBool;
    pub fn cmsDetectTAC(hProfile: cmsHPROFILE) -> f64;
    pub fn cmsDesaturateLab(Lab: *mut cmsCIELab, amax: f64, amin: f64,
                            bmax: f64, bmin: f64) -> cmsBool;
}
