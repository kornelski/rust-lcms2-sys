#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::os::raw::{c_char, c_int, c_long, c_void};
use libc::{wchar_t, tm, FILE};
use std::default::Default;

pub type cmsSignature = u32;
pub type cmsS15Fixed16Number = i32;
pub type cmsBool = c_int;


// D50 XYZ normalized to Y=1.0
pub const cmsD50X: f64 = 0.9642;
pub const cmsD50Y: f64 = 1.0;
pub const cmsD50Z: f64 = 0.8249;

// V4 perceptual black
pub const cmsPERCEPTUAL_BLACK_X: f64 = 0.00336;
pub const cmsPERCEPTUAL_BLACK_Y: f64 = 0.0034731;
pub const cmsPERCEPTUAL_BLACK_Z: f64 = 0.00287;

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

impl Default for cmsICCData {
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

impl Default for cmsDateTimeNumber {
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

impl Default for cmsEncodedXYZNumber {
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

impl Default for cmsProfileID {
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
impl Default for cmsICCHeader {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsTagBase {
    pub sig: cmsTagTypeSignature,
    pub reserved: [i8; 4usize],
}
impl Default for cmsTagBase {
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
impl Default for cmsTagEntry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type cmsHANDLE = *mut c_void;
pub type cmsHPROFILE = *mut c_void;
pub type cmsHTRANSFORM = *mut c_void;

pub const cmsMAXCHANNELS: usize =  16;                // Maximum number of channels in ICC profiles


// Format of pixel is defined by one cmsUInt32Number, using bit fields as follows
//
//                               2                1          0
//                          3 2 10987 6 5 4 3 2 1 098 7654 321
//                          A O TTTTT U Y F P X S EEE CCCC BBB
//
//            A: Floating point -- With this flag we can differentiate 16 bits as float and as int
//            O: Optimized -- previous optimization already returns the final 8-bit value
//            T: Pixeltype
//            F: Flavor  0=MinIsBlack(Chocolate) 1=MinIsWhite(Vanilla)
//            P: Planar? 0=Chunky, 1=Planar
//            X: swap 16 bps endianess?
//            S: Do swap? ie, BGR, KYMC
//            E: Extra samples
//            C: Channels (Samples per pixel)
//            B: bytes per sample
//            Y: Swap first - changes ABGR to BGRA and KCMY to CMYK


// Pixel types
pub const PT_ANY: u32 =       0;    // Don't check colorspace
                          // 1 & 2 are reserved
pub const PT_GRAY: u32 =      3;
pub const PT_RGB: u32 =       4;
pub const PT_CMY: u32 =       5;
pub const PT_CMYK: u32 =      6;
pub const PT_YCbCr: u32 =     7;
pub const PT_YUV: u32 =       8;      // Lu'v'
pub const PT_XYZ: u32 =       9;
pub const PT_Lab: u32 =       10;
pub const PT_YUVK: u32 =      11;     // Lu'v'K
pub const PT_HSV: u32 =       12;
pub const PT_HLS: u32 =       13;
pub const PT_Yxy: u32 =       14;

pub const PT_MCH1: u32 =      15;
pub const PT_MCH2: u32 =      16;
pub const PT_MCH3: u32 =      17;
pub const PT_MCH4: u32 =      18;
pub const PT_MCH5: u32 =      19;
pub const PT_MCH6: u32 =      20;
pub const PT_MCH7: u32 =      21;
pub const PT_MCH8: u32 =      22;
pub const PT_MCH9: u32 =      23;
pub const PT_MCH10: u32 =     24;
pub const PT_MCH11: u32 =     25;
pub const PT_MCH12: u32 =     26;
pub const PT_MCH13: u32 =     27;
pub const PT_MCH14: u32 =     28;
pub const PT_MCH15: u32 =     29;

pub const PT_LabV2: u32 =     30;     // Identical to PT_Lab, but using the V2 old encoding

pub const TYPE_YUVK_8: PixelFormat = PixelFormat::TYPE_CMYK_8_REV;
pub const TYPE_YUVK_16: PixelFormat = PixelFormat::TYPE_CMYK_16_REV;
pub const TYPE_ABGR_FLT: PixelFormat = PixelFormat::TYPE_BGR_FLT;
pub const TYPE_ABGR_HALF_FLT: PixelFormat = PixelFormat::TYPE_BGR_HALF_FLT;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum PixelFormat {
    TYPE_GRAY_8 = 196617,
    TYPE_GRAY_8_REV = 204809,
    TYPE_GRAY_16 = 196618,
    TYPE_GRAY_16_REV = 204810,
    TYPE_GRAY_16_SE = 198666,
    TYPE_GRAYA_8 = 196745,
    TYPE_GRAYA_16 = 196746,
    TYPE_GRAYA_16_SE = 198794,
    TYPE_GRAYA_8_PLANAR = 200841,
    TYPE_GRAYA_16_PLANAR = 200842,
    TYPE_RGB_8 = 262169,
    TYPE_RGB_8_PLANAR = 266265,
    TYPE_BGR_8 = 263193,
    TYPE_BGR_8_PLANAR = 267289,
    TYPE_RGB_16 = 262170,
    TYPE_RGB_16_PLANAR = 266266,
    TYPE_RGB_16_SE = 264218,
    TYPE_BGR_16 = 263194,
    TYPE_BGR_16_PLANAR = 267290,
    TYPE_BGR_16_SE = 265242,
    TYPE_RGBA_8 = 262297,
    TYPE_RGBA_8_PLANAR = 266393,
    TYPE_RGBA_16 = 262298,
    TYPE_RGBA_16_PLANAR = 266394,
    TYPE_RGBA_16_SE = 264346,
    TYPE_ARGB_8 = 278681,
    TYPE_ARGB_8_PLANAR = 282777,
    TYPE_ARGB_16 = 278682,
    TYPE_ABGR_8 = 263321,
    TYPE_ABGR_8_PLANAR = 267417,
    TYPE_ABGR_16 = 263322,
    TYPE_ABGR_16_PLANAR = 267418,
    TYPE_ABGR_16_SE = 265370,
    TYPE_BGRA_8 = 279705,
    TYPE_BGRA_8_PLANAR = 283801,
    TYPE_BGRA_16 = 279706,
    TYPE_BGRA_16_SE = 281754,
    TYPE_CMY_8 = 327705,
    TYPE_CMY_8_PLANAR = 331801,
    TYPE_CMY_16 = 327706,
    TYPE_CMY_16_PLANAR = 331802,
    TYPE_CMY_16_SE = 329754,
    TYPE_CMYK_8 = 393249,
    TYPE_CMYKA_8 = 393377,
    TYPE_CMYK_8_REV = 401441,
    TYPE_CMYK_8_PLANAR = 397345,
    TYPE_CMYK_16 = 393250,
    TYPE_CMYK_16_REV = 401442,
    TYPE_CMYK_16_PLANAR = 397346,
    TYPE_CMYK_16_SE = 395298,
    TYPE_KYMC_8 = 394273,
    TYPE_KYMC_16 = 394274,
    TYPE_KYMC_16_SE = 396322,
    TYPE_KCMY_8 = 409633,
    TYPE_KCMY_8_REV = 417825,
    TYPE_KCMY_16 = 409634,
    TYPE_KCMY_16_REV = 417826,
    TYPE_KCMY_16_SE = 411682,
    TYPE_CMYK5_8 = 1245225,
    TYPE_CMYK5_16 = 1245226,
    TYPE_CMYK5_16_SE = 1247274,
    TYPE_KYMC5_8 = 1246249,
    TYPE_KYMC5_16 = 1246250,
    TYPE_KYMC5_16_SE = 1248298,
    TYPE_CMYK6_8 = 1310769,
    TYPE_CMYK6_8_PLANAR = 1314865,
    TYPE_CMYK6_16 = 1310770,
    TYPE_CMYK6_16_PLANAR = 1314866,
    TYPE_CMYK6_16_SE = 1312818,
    TYPE_CMYK7_8 = 1376313,
    TYPE_CMYK7_16 = 1376314,
    TYPE_CMYK7_16_SE = 1378362,
    TYPE_KYMC7_8 = 1377337,
    TYPE_KYMC7_16 = 1377338,
    TYPE_KYMC7_16_SE = 1379386,
    TYPE_CMYK8_8 = 1441857,
    TYPE_CMYK8_16 = 1441858,
    TYPE_CMYK8_16_SE = 1443906,
    TYPE_KYMC8_8 = 1442881,
    TYPE_KYMC8_16 = 1442882,
    TYPE_KYMC8_16_SE = 1444930,
    TYPE_CMYK9_8 = 1507401,
    TYPE_CMYK9_16 = 1507402,
    TYPE_CMYK9_16_SE = 1509450,
    TYPE_KYMC9_8 = 1508425,
    TYPE_KYMC9_16 = 1508426,
    TYPE_KYMC9_16_SE = 1510474,
    TYPE_CMYK10_8 = 1572945,
    TYPE_CMYK10_16 = 1572946,
    TYPE_CMYK10_16_SE = 1574994,
    TYPE_KYMC10_8 = 1573969,
    TYPE_KYMC10_16 = 1573970,
    TYPE_KYMC10_16_SE = 1576018,
    TYPE_CMYK11_8 = 1638489,
    TYPE_CMYK11_16 = 1638490,
    TYPE_CMYK11_16_SE = 1640538,
    TYPE_KYMC11_8 = 1639513,
    TYPE_KYMC11_16 = 1639514,
    TYPE_KYMC11_16_SE = 1641562,
    TYPE_CMYK12_8 = 1704033,
    TYPE_CMYK12_16 = 1704034,
    TYPE_CMYK12_16_SE = 1706082,
    TYPE_KYMC12_8 = 1705057,
    TYPE_KYMC12_16 = 1705058,
    TYPE_KYMC12_16_SE = 1707106,
    TYPE_XYZ_16 = 589850,
    TYPE_Lab_8 = 655385,
    TYPE_LabV2_8 = 1966105,
    TYPE_ALab_8 = 671897,
    TYPE_ALabV2_8 = 1982617,
    TYPE_Lab_16 = 655386,
    TYPE_LabV2_16 = 1966106,
    TYPE_Yxy_16 = 917530,
    TYPE_YCbCr_8 = 458777,
    TYPE_YCbCr_8_PLANAR = 462873,
    TYPE_YCbCr_16 = 458778,
    TYPE_YCbCr_16_PLANAR = 462874,
    TYPE_YCbCr_16_SE = 460826,
    TYPE_YUV_8 = 524313,
    TYPE_YUV_8_PLANAR = 528409,
    TYPE_YUV_16 = 524314,
    TYPE_YUV_16_PLANAR = 528410,
    TYPE_YUV_16_SE = 526362,
    TYPE_HLS_8 = 851993,
    TYPE_HLS_8_PLANAR = 856089,
    TYPE_HLS_16 = 851994,
    TYPE_HLS_16_PLANAR = 856090,
    TYPE_HLS_16_SE = 854042,
    TYPE_HSV_8 = 786457,
    TYPE_HSV_8_PLANAR = 790553,
    TYPE_HSV_16 = 786458,
    TYPE_HSV_16_PLANAR = 790554,
    TYPE_HSV_16_SE = 788506,
    TYPE_NAMED_COLOR_INDEX = 10,
    TYPE_XYZ_FLT = 4784156,
    TYPE_Lab_FLT = 4849692,
    TYPE_LabA_FLT = 4849820,
    TYPE_GRAY_FLT = 4390924,
    TYPE_RGB_FLT = 4456476,
    TYPE_RGBA_FLT = 4456604,
    TYPE_ARGB_FLT = 4472988,
    TYPE_BGR_FLT = 4457500,
    TYPE_BGRA_FLT = 4474012,
    TYPE_CMYK_FLT = 4587556,
    TYPE_XYZ_DBL = 4784152,
    TYPE_Lab_DBL = 4849688,
    TYPE_GRAY_DBL = 4390920,
    TYPE_RGB_DBL = 4456472,
    TYPE_BGR_DBL = 4457496,
    TYPE_CMYK_DBL = 4587552,
    TYPE_GRAY_HALF_FLT = 4390922,
    TYPE_RGB_HALF_FLT = 4456474,
    TYPE_RGBA_HALF_FLT = 4456602,
    TYPE_CMYK_HALF_FLT = 4587554,
    TYPE_ARGB_HALF_FLT = 4472986,
    TYPE_BGR_HALF_FLT = 4457498,
    TYPE_BGRA_HALF_FLT = 4474010,
}
pub use self::PixelFormat::*;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsCIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
impl Default for cmsCIEXYZ {
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
impl Default for cmsCIExyY {
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
impl Default for cmsCIELab {
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
impl Default for cmsCIELCh {
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
impl Default for cmsJCh {
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
impl Default for cmsCIEXYZTRIPLE {
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
impl Default for cmsCIExyYTRIPLE {
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
impl Default for cmsICCMeasurementConditions {
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
impl Default for cmsICCViewingConditions {
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
impl Default for cmsViewingConditions {
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
impl Default for cmsCurveSegment {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub enum cmsToneCurve { }
pub enum cmsPipeline { }
pub enum cmsStage { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsStageLoc {
    cmsAT_BEGIN = 0,
    cmsAT_END = 1,
}
pub type cmsSAMPLER16 = ::std::option::Option<unsafe extern "C" fn(In: *mut u16, Out: *mut u16, Cargo: *mut c_void) -> i32>;
pub type cmsSAMPLERFLOAT = ::std::option::Option<unsafe extern "C" fn(In: *mut f32, Out: *mut f32, Cargo: *mut c_void) -> i32>;
pub enum cmsMLU { }

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsUcrBg {
    pub Ucr: *mut cmsToneCurve,
    pub Bg: *mut cmsToneCurve,
    pub Desc: *mut cmsMLU,
}
impl Default for cmsUcrBg {
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
impl Default for cmsScreeningChannel {
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
impl Default for cmsScreening {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum cmsNAMEDCOLORLIST { }

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
impl Default for cmsPSEQDESC {
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
impl Default for cmsSEQ {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsDICTentry {
    pub Next: *mut cmsDICTentry,
    pub DisplayName: *mut cmsMLU,
    pub DisplayValue: *mut cmsMLU,
    pub Name: *mut wchar_t,
    pub Value: *mut wchar_t,
}
impl Default for cmsDICTentry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsInfoType {
    cmsInfoDescription = 0,
    cmsInfoManufacturer = 1,
    cmsInfoModel = 2,
    cmsInfoCopyright = 3,
}
pub enum cmsIOHANDLER { }

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsPSResourceType {
    cmsPS_RESOURCE_CSA = 0,
    cmsPS_RESOURCE_CRD = 1,
}
extern "C" {
    pub fn cmsGetEncodedCMMversion() -> c_int;
    pub fn cmsstrcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn cmsfilelength(f: *mut FILE) -> c_long;
    pub fn cmsCreateContext(Plugin: *mut c_void, UserData: *mut c_void) -> cmsContext;
    pub fn cmsDeleteContext(ContexID: cmsContext);
    pub fn cmsDupContext(ContextID: cmsContext, NewUserData: *mut c_void) -> cmsContext;
    pub fn cmsGetContextUserData(ContextID: cmsContext) -> *mut c_void;
    pub fn cmsPlugin(Plugin: *mut c_void) -> cmsBool;
    pub fn cmsPluginTHR(ContextID: cmsContext, Plugin: *mut c_void) -> cmsBool;
    pub fn cmsUnregisterPlugins();
    pub fn cmsUnregisterPluginsTHR(ContextID: cmsContext);
    pub fn cmsSetLogErrorHandler(Fn: cmsLogErrorHandlerFunction);
    pub fn cmsSetLogErrorHandlerTHR(ContextID: cmsContext, Fn: cmsLogErrorHandlerFunction);
    pub fn cmsD50_XYZ() -> *const cmsCIEXYZ;
    pub fn cmsD50_xyY() -> *const cmsCIExyY;
    pub fn cmsXYZ2xyY(Dest: *mut cmsCIExyY, Source: *const cmsCIEXYZ);
    pub fn cmsxyY2XYZ(Dest: *mut cmsCIEXYZ, Source: *const cmsCIExyY);
    pub fn cmsXYZ2Lab(WhitePoint: *const cmsCIEXYZ, Lab: *mut cmsCIELab, xyz: *const cmsCIEXYZ);
    pub fn cmsLab2XYZ(WhitePoint: *const cmsCIEXYZ, xyz: *mut cmsCIEXYZ, Lab: *const cmsCIELab);
    pub fn cmsLab2LCh(LCh: *mut cmsCIELCh, Lab: *const cmsCIELab);
    pub fn cmsLCh2Lab(Lab: *mut cmsCIELab, LCh: *const cmsCIELCh);
    pub fn cmsLabEncoded2Float(Lab: *mut cmsCIELab, wLab: *mut u16);
    pub fn cmsLabEncoded2FloatV2(Lab: *mut cmsCIELab, wLab: *mut u16);
    pub fn cmsFloat2LabEncoded(wLab: *mut u16, Lab: *const cmsCIELab);
    pub fn cmsFloat2LabEncodedV2(wLab: *mut u16, Lab: *const cmsCIELab);
    pub fn cmsXYZEncoded2Float(fxyz: *mut cmsCIEXYZ, XYZ: *mut u16);
    pub fn cmsFloat2XYZEncoded(XYZ: *mut u16, fXYZ: *const cmsCIEXYZ);
    pub fn cmsDeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab) -> f64;
    pub fn cmsCIE94DeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab) -> f64;
    pub fn cmsBFDdeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab) -> f64;
    pub fn cmsCMCdeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab, l: f64, c: f64) -> f64;
    pub fn cmsCIE2000DeltaE(Lab1: *const cmsCIELab, Lab2: *const cmsCIELab, Kl: f64, Kc: f64, Kh: f64) -> f64;
    pub fn cmsWhitePointFromTemp(WhitePoint: *mut cmsCIExyY, TempK: f64) -> cmsBool;
    pub fn cmsTempFromWhitePoint(TempK: *mut f64, WhitePoint: *const cmsCIExyY) -> cmsBool;
    pub fn cmsAdaptToIlluminant(Result: *mut cmsCIEXYZ, SourceWhitePt: *const cmsCIEXYZ, Illuminant: *const cmsCIEXYZ, Value: *const cmsCIEXYZ) -> cmsBool;
    pub fn cmsCIECAM02Init(ContextID: cmsContext, pVC: *const cmsViewingConditions) -> cmsHANDLE;
    pub fn cmsCIECAM02Done(hModel: cmsHANDLE);
    pub fn cmsCIECAM02Forward(hModel: cmsHANDLE, pIn: *const cmsCIEXYZ, pOut: *mut cmsJCh);
    pub fn cmsCIECAM02Reverse(hModel: cmsHANDLE, pIn: *const cmsJCh, pOut: *mut cmsCIEXYZ);
    pub fn cmsBuildSegmentedToneCurve(ContextID: cmsContext, nSegments: i32, Segments: *mut cmsCurveSegment) -> *mut cmsToneCurve;
    pub fn cmsBuildParametricToneCurve(ContextID: cmsContext, Type: i32, Params: *mut f64) -> *mut cmsToneCurve;
    pub fn cmsBuildGamma(ContextID: cmsContext, Gamma: f64) -> *mut cmsToneCurve;
    pub fn cmsBuildTabulatedToneCurve16(ContextID: cmsContext, nEntries: i32, values: *mut u16) -> *mut cmsToneCurve;
    pub fn cmsBuildTabulatedToneCurveFloat(ContextID: cmsContext, nEntries: u32, values: *mut f32) -> *mut cmsToneCurve;
    pub fn cmsFreeToneCurve(Curve: *mut cmsToneCurve);
    pub fn cmsFreeToneCurveTriple(Curve: *mut *mut cmsToneCurve);
    pub fn cmsDupToneCurve(Src: *const cmsToneCurve) -> *mut cmsToneCurve;
    pub fn cmsReverseToneCurve(InGamma: *const cmsToneCurve) -> *mut cmsToneCurve;
    pub fn cmsReverseToneCurveEx(nResultSamples: i32, InGamma: *const cmsToneCurve) -> *mut cmsToneCurve;
    pub fn cmsJoinToneCurve(ContextID: cmsContext, X: *const cmsToneCurve, Y: *const cmsToneCurve, nPoints: u32) -> *mut cmsToneCurve;
    pub fn cmsSmoothToneCurve(Tab: *mut cmsToneCurve, lambda: f64) -> cmsBool;
    pub fn cmsEvalToneCurveFloat(Curve: *const cmsToneCurve, v: f32) -> f32;
    pub fn cmsEvalToneCurve16(Curve: *const cmsToneCurve, v: u16) -> u16;
    pub fn cmsIsToneCurveMultisegment(InGamma: *const cmsToneCurve) -> cmsBool;
    pub fn cmsIsToneCurveLinear(Curve: *const cmsToneCurve) -> cmsBool;
    pub fn cmsIsToneCurveMonotonic(t: *const cmsToneCurve) -> cmsBool;
    pub fn cmsIsToneCurveDescending(t: *const cmsToneCurve) -> cmsBool;
    pub fn cmsGetToneCurveParametricType(t: *const cmsToneCurve) -> i32;
    pub fn cmsEstimateGamma(t: *const cmsToneCurve, Precision: f64) -> f64;
    pub fn cmsGetToneCurveEstimatedTableEntries(t: *const cmsToneCurve) -> u32;
    pub fn cmsGetToneCurveEstimatedTable(t: *const cmsToneCurve) -> *const u16;
    pub fn cmsPipelineAlloc(ContextID: cmsContext, InputChannels: u32, OutputChannels: u32) -> *mut cmsPipeline;
    pub fn cmsPipelineFree(lut: *mut cmsPipeline);
    pub fn cmsPipelineDup(Orig: *const cmsPipeline) -> *mut cmsPipeline;
    pub fn cmsGetPipelineContextID(lut: *const cmsPipeline) -> cmsContext;
    pub fn cmsPipelineInputChannels(lut: *const cmsPipeline) -> u32;
    pub fn cmsPipelineOutputChannels(lut: *const cmsPipeline) -> u32;
    pub fn cmsPipelineStageCount(lut: *const cmsPipeline) -> u32;
    pub fn cmsPipelineGetPtrToFirstStage(lut: *const cmsPipeline) -> *mut cmsStage;
    pub fn cmsPipelineGetPtrToLastStage(lut: *const cmsPipeline) -> *mut cmsStage;
    pub fn cmsPipelineEval16(In: *mut u16, Out: *mut u16, lut: *const cmsPipeline);
    pub fn cmsPipelineEvalFloat(In: *mut f32, Out: *mut f32, lut: *const cmsPipeline);
    pub fn cmsPipelineEvalReverseFloat(Target: *mut f32, Result: *mut f32, Hint: *mut f32, lut: *const cmsPipeline) -> cmsBool;
    pub fn cmsPipelineCat(l1: *mut cmsPipeline, l2: *const cmsPipeline) -> cmsBool;
    pub fn cmsPipelineSetSaveAs8bitsFlag(lut: *mut cmsPipeline, On: cmsBool) -> cmsBool;
    pub fn cmsPipelineInsertStage(lut: *mut cmsPipeline, loc: cmsStageLoc, mpe: *mut cmsStage) -> c_int;
    pub fn cmsPipelineUnlinkStage(lut: *mut cmsPipeline, loc: cmsStageLoc, mpe: *mut *mut cmsStage);
    pub fn cmsPipelineCheckAndRetreiveStages(Lut: *const cmsPipeline, n: u32, ...) -> cmsBool;
    pub fn cmsStageAllocIdentity(ContextID: cmsContext, nChannels: u32) -> *mut cmsStage;
    pub fn cmsStageAllocToneCurves(ContextID: cmsContext, nChannels: u32, Curves: *mut *mut cmsToneCurve) -> *mut cmsStage;
    pub fn cmsStageAllocMatrix(ContextID: cmsContext, Rows: u32, Cols: u32, Matrix: *const f64, Offset: *const f64) -> *mut cmsStage;
    pub fn cmsStageAllocCLut16bit(ContextID: cmsContext, nGridPoints: u32, inputChan: u32, outputChan: u32, Table: *const u16) -> *mut cmsStage;
    pub fn cmsStageAllocCLutFloat(ContextID: cmsContext, nGridPoints: u32, inputChan: u32, outputChan: u32, Table: *const f32) -> *mut cmsStage;
    pub fn cmsStageAllocCLut16bitGranular(ContextID: cmsContext, clutPoints: *mut u32, inputChan: u32, outputChan: u32, Table: *const u16) -> *mut cmsStage;
    pub fn cmsStageAllocCLutFloatGranular(ContextID: cmsContext, clutPoints: *mut u32, inputChan: u32, outputChan: u32, Table: *const f32) -> *mut cmsStage;
    pub fn cmsStageDup(mpe: *mut cmsStage) -> *mut cmsStage;
    pub fn cmsStageFree(mpe: *mut cmsStage);
    pub fn cmsStageNext(mpe: *const cmsStage) -> *mut cmsStage;
    pub fn cmsStageInputChannels(mpe: *const cmsStage) -> u32;
    pub fn cmsStageOutputChannels(mpe: *const cmsStage) -> u32;
    pub fn cmsStageType(mpe: *const cmsStage) -> cmsStageSignature;
    pub fn cmsStageData(mpe: *const cmsStage) -> *mut c_void;
    pub fn cmsStageSampleCLut16bit(mpe: *mut cmsStage, Sampler: cmsSAMPLER16, Cargo: *mut c_void, dwFlags: u32) -> cmsBool;
    pub fn cmsStageSampleCLutFloat(mpe: *mut cmsStage, Sampler: cmsSAMPLERFLOAT, Cargo: *mut c_void, dwFlags: u32) -> cmsBool;
    pub fn cmsSliceSpace16(nInputs: u32, clutPoints: *mut u32, Sampler: cmsSAMPLER16, Cargo: *mut c_void) -> cmsBool;
    pub fn cmsSliceSpaceFloat(nInputs: u32, clutPoints: *mut u32, Sampler: cmsSAMPLERFLOAT, Cargo: *mut c_void) -> cmsBool;
    pub fn cmsMLUalloc(ContextID: cmsContext, nItems: u32) -> *mut cmsMLU;
    pub fn cmsMLUfree(mlu: *mut cmsMLU);
    pub fn cmsMLUdup(mlu: *const cmsMLU) -> *mut cmsMLU;
    pub fn cmsMLUsetASCII(mlu: *mut cmsMLU, LanguageCode: *mut c_char, CountryCode: *mut c_char, ASCIIString: *const c_char) -> cmsBool;
    pub fn cmsMLUsetWide(mlu: *mut cmsMLU, LanguageCode: *mut c_char, CountryCode: *mut c_char, WideString: *const wchar_t) -> cmsBool;
    pub fn cmsMLUgetASCII(mlu: *const cmsMLU, LanguageCode: *mut c_char, CountryCode: *mut c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
    pub fn cmsMLUgetWide(mlu: *const cmsMLU, LanguageCode: *mut c_char, CountryCode: *mut c_char, Buffer: *mut wchar_t, BufferSize: u32) -> u32;
    pub fn cmsMLUgetTranslation(mlu: *const cmsMLU, LanguageCode: *mut c_char, CountryCode: *mut c_char, ObtainedLanguage: *mut c_char, ObtainedCountry: *mut c_char) -> cmsBool;
    pub fn cmsMLUtranslationsCount(mlu: *const cmsMLU) -> u32;
    pub fn cmsMLUtranslationsCodes(mlu: *const cmsMLU, idx: u32, LanguageCode: *mut c_char, CountryCode: *mut c_char) -> cmsBool;
    pub fn cmsAllocNamedColorList(ContextID: cmsContext, n: u32, ColorantCount: u32, Prefix: *const c_char, Suffix: *const c_char) -> *mut cmsNAMEDCOLORLIST;
    pub fn cmsFreeNamedColorList(v: *mut cmsNAMEDCOLORLIST);
    pub fn cmsDupNamedColorList(v: *const cmsNAMEDCOLORLIST) -> *mut cmsNAMEDCOLORLIST;
    pub fn cmsAppendNamedColor(v: *mut cmsNAMEDCOLORLIST, Name: *const c_char, PCS: *mut u16, Colorant: *mut u16) -> cmsBool;
    pub fn cmsNamedColorCount(v: *const cmsNAMEDCOLORLIST) -> u32;
    pub fn cmsNamedColorIndex(v: *const cmsNAMEDCOLORLIST, Name: *const c_char) -> i32;
    pub fn cmsNamedColorInfo(NamedColorList: *const cmsNAMEDCOLORLIST, nColor: u32, Name: *mut c_char, Prefix: *mut c_char, Suffix: *mut c_char, PCS: *mut u16, Colorant: *mut u16) -> cmsBool;
    pub fn cmsGetNamedColorList(xform: cmsHTRANSFORM) -> *mut cmsNAMEDCOLORLIST;
    pub fn cmsAllocProfileSequenceDescription(ContextID: cmsContext, n: u32) -> *mut cmsSEQ;
    pub fn cmsDupProfileSequenceDescription(pseq: *const cmsSEQ) -> *mut cmsSEQ;
    pub fn cmsFreeProfileSequenceDescription(pseq: *mut cmsSEQ);
    pub fn cmsDictAlloc(ContextID: cmsContext) -> cmsHANDLE;
    pub fn cmsDictFree(hDict: cmsHANDLE);
    pub fn cmsDictDup(hDict: cmsHANDLE) -> cmsHANDLE;
    pub fn cmsDictAddEntry(hDict: cmsHANDLE, Name: *const wchar_t, Value: *const wchar_t, DisplayName: *const cmsMLU, DisplayValue: *const cmsMLU) -> cmsBool;
    pub fn cmsDictGetEntryList(hDict: cmsHANDLE) -> *const cmsDICTentry;
    pub fn cmsDictNextEntry(e: *const cmsDICTentry) -> *const cmsDICTentry;
    pub fn cmsCreateProfilePlaceholder(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsGetProfileContextID(hProfile: cmsHPROFILE) -> cmsContext;
    pub fn cmsGetTagCount(hProfile: cmsHPROFILE) -> i32;
    pub fn cmsGetTagSignature(hProfile: cmsHPROFILE, n: u32) -> cmsTagSignature;
    pub fn cmsIsTag(hProfile: cmsHPROFILE, sig: cmsTagSignature) -> cmsBool;
    pub fn cmsReadTag(hProfile: cmsHPROFILE, sig: cmsTagSignature) -> *mut c_void;
    pub fn cmsWriteTag(hProfile: cmsHPROFILE, sig: cmsTagSignature, data: *const c_void) -> cmsBool;
    pub fn cmsLinkTag(hProfile: cmsHPROFILE, sig: cmsTagSignature, dest: cmsTagSignature) -> cmsBool;
    pub fn cmsTagLinkedTo(hProfile: cmsHPROFILE, sig: cmsTagSignature) -> cmsTagSignature;
    pub fn cmsReadRawTag(hProfile: cmsHPROFILE, sig: cmsTagSignature, Buffer: *mut c_void, BufferSize: u32) -> i32;
    pub fn cmsWriteRawTag(hProfile: cmsHPROFILE, sig: cmsTagSignature, data: *const c_void, Size: u32) -> cmsBool;
    pub fn cmsGetHeaderFlags(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsGetHeaderAttributes(hProfile: cmsHPROFILE, Flags: *mut u64);
    pub fn cmsGetHeaderProfileID(hProfile: cmsHPROFILE, ProfileID: *mut u8);
    pub fn cmsGetHeaderCreationDateTime(hProfile: cmsHPROFILE, Dest: *mut tm) -> cmsBool;
    pub fn cmsGetHeaderRenderingIntent(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsSetHeaderFlags(hProfile: cmsHPROFILE, Flags: u32);
    pub fn cmsGetHeaderManufacturer(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsSetHeaderManufacturer(hProfile: cmsHPROFILE, manufacturer: u32);
    pub fn cmsGetHeaderCreator(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsGetHeaderModel(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsSetHeaderModel(hProfile: cmsHPROFILE, model: u32);
    pub fn cmsSetHeaderAttributes(hProfile: cmsHPROFILE, Flags: u64);
    pub fn cmsSetHeaderProfileID(hProfile: cmsHPROFILE, ProfileID: *mut u8);
    pub fn cmsSetHeaderRenderingIntent(hProfile: cmsHPROFILE, RenderingIntent: u32);
    pub fn cmsGetPCS(hProfile: cmsHPROFILE) -> cmsColorSpaceSignature;
    pub fn cmsSetPCS(hProfile: cmsHPROFILE, pcs: cmsColorSpaceSignature);
    pub fn cmsGetColorSpace(hProfile: cmsHPROFILE) -> cmsColorSpaceSignature;
    pub fn cmsSetColorSpace(hProfile: cmsHPROFILE, sig: cmsColorSpaceSignature);
    pub fn cmsGetDeviceClass(hProfile: cmsHPROFILE) -> cmsProfileClassSignature;
    pub fn cmsSetDeviceClass(hProfile: cmsHPROFILE, sig: cmsProfileClassSignature);
    pub fn cmsSetProfileVersion(hProfile: cmsHPROFILE, Version: f64);
    pub fn cmsGetProfileVersion(hProfile: cmsHPROFILE) -> f64;
    pub fn cmsGetEncodedICCversion(hProfile: cmsHPROFILE) -> u32;
    pub fn cmsSetEncodedICCversion(hProfile: cmsHPROFILE, Version: u32);
    pub fn cmsIsIntentSupported(hProfile: cmsHPROFILE, Intent: u32, UsedDirection: u32) -> cmsBool;
    pub fn cmsIsMatrixShaper(hProfile: cmsHPROFILE) -> cmsBool;
    pub fn cmsIsCLUT(hProfile: cmsHPROFILE, Intent: u32, UsedDirection: u32) -> cmsBool;
    pub fn _cmsICCcolorSpace(OurNotation: c_int) -> cmsColorSpaceSignature;
    pub fn _cmsLCMScolorSpace(ProfileSpace: cmsColorSpaceSignature) -> c_int;
    pub fn cmsChannelsOf(ColorSpace: cmsColorSpaceSignature) -> u32;
    pub fn cmsFormatterForColorspaceOfProfile(hProfile: cmsHPROFILE, nBytes: u32, lIsFloat: cmsBool) -> u32;
    pub fn cmsFormatterForPCSOfProfile(hProfile: cmsHPROFILE, nBytes: u32, lIsFloat: cmsBool) -> u32;
    pub fn cmsGetProfileInfo(hProfile: cmsHPROFILE, Info: cmsInfoType, LanguageCode: *mut c_char, CountryCode: *mut c_char, Buffer: *mut wchar_t, BufferSize: u32) -> u32;
    pub fn cmsGetProfileInfoASCII(hProfile: cmsHPROFILE, Info: cmsInfoType, LanguageCode: *mut c_char, CountryCode: *mut c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
    pub fn cmsOpenIOhandlerFromFile(ContextID: cmsContext, FileName: *const c_char, AccessMode: *const c_char) -> *mut cmsIOHANDLER;
    pub fn cmsOpenIOhandlerFromStream(ContextID: cmsContext, Stream: *mut FILE) -> *mut cmsIOHANDLER;
    pub fn cmsOpenIOhandlerFromMem(ContextID: cmsContext, Buffer: *mut c_void, size: u32, AccessMode: *const c_char) -> *mut cmsIOHANDLER;
    pub fn cmsOpenIOhandlerFromNULL(ContextID: cmsContext) -> *mut cmsIOHANDLER;
    pub fn cmsGetProfileIOhandler(hProfile: cmsHPROFILE) -> *mut cmsIOHANDLER;
    pub fn cmsCloseIOhandler(io: *mut cmsIOHANDLER) -> cmsBool;
    pub fn cmsMD5computeID(hProfile: cmsHPROFILE) -> cmsBool;
    pub fn cmsOpenProfileFromFile(ICCProfile: *const c_char, sAccess: *const c_char) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromFileTHR(ContextID: cmsContext, ICCProfile: *const c_char, sAccess: *const c_char) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromStream(ICCProfile: *mut FILE, sAccess: *const c_char) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromStreamTHR(ContextID: cmsContext, ICCProfile: *mut FILE, sAccess: *const c_char) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromMem(MemPtr: *const c_void, dwSize: u32) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromMemTHR(ContextID: cmsContext, MemPtr: *const c_void, dwSize: u32) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromIOhandlerTHR(ContextID: cmsContext, io: *mut cmsIOHANDLER) -> cmsHPROFILE;
    pub fn cmsOpenProfileFromIOhandler2THR(ContextID: cmsContext, io: *mut cmsIOHANDLER, write: cmsBool) -> cmsHPROFILE;
    pub fn cmsCloseProfile(hProfile: cmsHPROFILE) -> cmsBool;
    pub fn cmsSaveProfileToFile(hProfile: cmsHPROFILE, FileName: *const c_char) -> cmsBool;
    pub fn cmsSaveProfileToStream(hProfile: cmsHPROFILE, Stream: *mut FILE) -> cmsBool;
    pub fn cmsSaveProfileToMem(hProfile: cmsHPROFILE, MemPtr: *mut c_void, BytesNeeded: *mut u32) -> cmsBool;
    pub fn cmsSaveProfileToIOhandler(hProfile: cmsHPROFILE, io: *mut cmsIOHANDLER) -> u32;
    pub fn cmsCreateRGBProfileTHR(ContextID: cmsContext, WhitePoint: *const cmsCIExyY, Primaries: *const cmsCIExyYTRIPLE, TransferFunction: *mut *mut cmsToneCurve) -> cmsHPROFILE;
    pub fn cmsCreateRGBProfile(WhitePoint: *const cmsCIExyY, Primaries: *const cmsCIExyYTRIPLE, TransferFunction: *mut *mut cmsToneCurve) -> cmsHPROFILE;
    pub fn cmsCreateGrayProfileTHR(ContextID: cmsContext, WhitePoint: *const cmsCIExyY, TransferFunction: *const cmsToneCurve) -> cmsHPROFILE;
    pub fn cmsCreateGrayProfile(WhitePoint: *const cmsCIExyY, TransferFunction: *const cmsToneCurve) -> cmsHPROFILE;
    pub fn cmsCreateLinearizationDeviceLinkTHR(ContextID: cmsContext, ColorSpace: cmsColorSpaceSignature, TransferFunctions: *mut *mut cmsToneCurve) -> cmsHPROFILE;
    pub fn cmsCreateLinearizationDeviceLink(ColorSpace: cmsColorSpaceSignature, TransferFunctions: *mut *mut cmsToneCurve) -> cmsHPROFILE;
    pub fn cmsCreateInkLimitingDeviceLinkTHR(ContextID: cmsContext, ColorSpace: cmsColorSpaceSignature, Limit: f64) -> cmsHPROFILE;
    pub fn cmsCreateInkLimitingDeviceLink(ColorSpace: cmsColorSpaceSignature, Limit: f64) -> cmsHPROFILE;
    pub fn cmsCreateLab2ProfileTHR(ContextID: cmsContext, WhitePoint: *const cmsCIExyY) -> cmsHPROFILE;
    pub fn cmsCreateLab2Profile(WhitePoint: *const cmsCIExyY) -> cmsHPROFILE;
    pub fn cmsCreateLab4ProfileTHR(ContextID: cmsContext, WhitePoint: *const cmsCIExyY) -> cmsHPROFILE;
    pub fn cmsCreateLab4Profile(WhitePoint: *const cmsCIExyY) -> cmsHPROFILE;
    pub fn cmsCreateXYZProfileTHR(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsCreateXYZProfile() -> cmsHPROFILE;
    pub fn cmsCreate_sRGBProfileTHR(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsCreate_sRGBProfile() -> cmsHPROFILE;
    pub fn cmsCreateBCHSWabstractProfileTHR(ContextID: cmsContext, nLUTPoints: c_int, Bright: f64, Contrast: f64, Hue: f64, Saturation: f64, TempSrc: c_int, TempDest: c_int) -> cmsHPROFILE;
    pub fn cmsCreateBCHSWabstractProfile(nLUTPoints: c_int, Bright: f64, Contrast: f64, Hue: f64, Saturation: f64, TempSrc: c_int, TempDest: c_int) -> cmsHPROFILE;
    pub fn cmsCreateNULLProfileTHR(ContextID: cmsContext) -> cmsHPROFILE;
    pub fn cmsCreateNULLProfile() -> cmsHPROFILE;
    pub fn cmsTransform2DeviceLink(hTransform: cmsHTRANSFORM, Version: f64, dwFlags: u32) -> cmsHPROFILE;
    pub fn cmsGetSupportedIntents(nMax: u32, Codes: *mut u32, Descriptions: *mut *mut c_char) -> u32;
    pub fn cmsGetSupportedIntentsTHR(ContextID: cmsContext, nMax: u32, Codes: *mut u32, Descriptions: *mut *mut c_char) -> u32;
    pub fn cmsCreateTransformTHR(ContextID: cmsContext, Input: cmsHPROFILE, InputFormat: PixelFormat, Output: cmsHPROFILE, OutputFormat: PixelFormat, Intent: u32, dwFlags: u32) -> cmsHTRANSFORM;
    pub fn cmsCreateTransform(Input: cmsHPROFILE, InputFormat: PixelFormat, Output: cmsHPROFILE, OutputFormat: PixelFormat, Intent: u32, dwFlags: u32) -> cmsHTRANSFORM;
    pub fn cmsCreateProofingTransformTHR(ContextID: cmsContext,
                                         Input: cmsHPROFILE,
                                         InputFormat: PixelFormat,
                                         Output: cmsHPROFILE,
                                         OutputFormat: PixelFormat,
                                         Proofing: cmsHPROFILE,
                                         Intent: u32,
                                         ProofingIntent: u32,
                                         dwFlags: u32)
                                         -> cmsHTRANSFORM;
    pub fn cmsCreateProofingTransform(Input: cmsHPROFILE,
                                      InputFormat: PixelFormat,
                                      Output: cmsHPROFILE,
                                      OutputFormat: PixelFormat,
                                      Proofing: cmsHPROFILE,
                                      Intent: u32,
                                      ProofingIntent: u32,
                                      dwFlags: u32)
                                      -> cmsHTRANSFORM;
    pub fn cmsCreateMultiprofileTransformTHR(ContextID: cmsContext,
                                             hProfiles: *mut cmsHPROFILE,
                                             nProfiles: u32,
                                             InputFormat: PixelFormat,
                                             OutputFormat: PixelFormat,
                                             Intent: u32,
                                             dwFlags: u32)
                                             -> cmsHTRANSFORM;
    pub fn cmsCreateMultiprofileTransform(hProfiles: *mut cmsHPROFILE, nProfiles: u32, InputFormat: PixelFormat, OutputFormat: PixelFormat, Intent: u32, dwFlags: u32) -> cmsHTRANSFORM;
    pub fn cmsCreateExtendedTransform(ContextID: cmsContext,
                                      nProfiles: u32,
                                      hProfiles: *mut cmsHPROFILE,
                                      BPC: *mut cmsBool,
                                      Intents: *mut u32,
                                      AdaptationStates: *mut f64,
                                      hGamutProfile: cmsHPROFILE,
                                      nGamutPCSposition: u32,
                                      InputFormat: PixelFormat,
                                      OutputFormat: PixelFormat,
                                      dwFlags: u32)
                                      -> cmsHTRANSFORM;
    pub fn cmsDeleteTransform(hTransform: cmsHTRANSFORM);
    pub fn cmsDoTransform(Transform: cmsHTRANSFORM, InputBuffer: *const c_void, OutputBuffer: *mut c_void, Size: u32);
    pub fn cmsDoTransformStride(Transform: cmsHTRANSFORM, InputBuffer: *const c_void, OutputBuffer: *mut c_void, Size: u32, Stride: u32);
    pub fn cmsSetAlarmCodes(NewAlarm: *mut u16);
    pub fn cmsGetAlarmCodes(NewAlarm: *mut u16);
    pub fn cmsSetAlarmCodesTHR(ContextID: cmsContext, AlarmCodes: *mut u16);
    pub fn cmsGetAlarmCodesTHR(ContextID: cmsContext, AlarmCodes: *mut u16);
    pub fn cmsSetAdaptationState(d: f64) -> f64;
    pub fn cmsSetAdaptationStateTHR(ContextID: cmsContext, d: f64) -> f64;
    pub fn cmsGetTransformContextID(hTransform: cmsHTRANSFORM) -> cmsContext;
    pub fn cmsGetTransformInputFormat(hTransform: cmsHTRANSFORM) -> u32;
    pub fn cmsGetTransformOutputFormat(hTransform: cmsHTRANSFORM) -> u32;
    pub fn cmsChangeBuffersFormat(hTransform: cmsHTRANSFORM, InputFormat: PixelFormat, OutputFormat: PixelFormat) -> cmsBool;
    pub fn cmsGetPostScriptColorResource(ContextID: cmsContext, Type: cmsPSResourceType, hProfile: cmsHPROFILE, Intent: u32, dwFlags: u32, io: *mut cmsIOHANDLER) -> u32;
    pub fn cmsGetPostScriptCSA(ContextID: cmsContext, hProfile: cmsHPROFILE, Intent: u32, dwFlags: u32, Buffer: *mut c_void, dwBufferLen: u32) -> u32;
    pub fn cmsGetPostScriptCRD(ContextID: cmsContext, hProfile: cmsHPROFILE, Intent: u32, dwFlags: u32, Buffer: *mut c_void, dwBufferLen: u32) -> u32;
    pub fn cmsIT8Alloc(ContextID: cmsContext) -> cmsHANDLE;
    pub fn cmsIT8Free(hIT8: cmsHANDLE);
    pub fn cmsIT8TableCount(hIT8: cmsHANDLE) -> u32;
    pub fn cmsIT8SetTable(hIT8: cmsHANDLE, nTable: u32) -> i32;
    pub fn cmsIT8LoadFromFile(ContextID: cmsContext, cFileName: *const c_char) -> cmsHANDLE;
    pub fn cmsIT8LoadFromMem(ContextID: cmsContext, Ptr: *mut c_void, len: u32) -> cmsHANDLE;
    pub fn cmsIT8SaveToFile(hIT8: cmsHANDLE, cFileName: *const c_char) -> cmsBool;
    pub fn cmsIT8SaveToMem(hIT8: cmsHANDLE, MemPtr: *mut c_void, BytesNeeded: *mut u32) -> cmsBool;
    pub fn cmsIT8GetSheetType(hIT8: cmsHANDLE) -> *const c_char;
    pub fn cmsIT8SetSheetType(hIT8: cmsHANDLE, Type: *const c_char) -> cmsBool;
    pub fn cmsIT8SetComment(hIT8: cmsHANDLE, cComment: *const c_char) -> cmsBool;
    pub fn cmsIT8SetPropertyStr(hIT8: cmsHANDLE, cProp: *const c_char, Str: *const c_char) -> cmsBool;
    pub fn cmsIT8SetPropertyDbl(hIT8: cmsHANDLE, cProp: *const c_char, Val: f64) -> cmsBool;
    pub fn cmsIT8SetPropertyHex(hIT8: cmsHANDLE, cProp: *const c_char, Val: u32) -> cmsBool;
    pub fn cmsIT8SetPropertyMulti(hIT8: cmsHANDLE, Key: *const c_char, SubKey: *const c_char, Buffer: *const c_char) -> cmsBool;
    pub fn cmsIT8SetPropertyUncooked(hIT8: cmsHANDLE, Key: *const c_char, Buffer: *const c_char) -> cmsBool;
    pub fn cmsIT8GetProperty(hIT8: cmsHANDLE, cProp: *const c_char) -> *const c_char;
    pub fn cmsIT8GetPropertyDbl(hIT8: cmsHANDLE, cProp: *const c_char) -> f64;
    pub fn cmsIT8GetPropertyMulti(hIT8: cmsHANDLE, Key: *const c_char, SubKey: *const c_char) -> *const c_char;
    pub fn cmsIT8EnumProperties(hIT8: cmsHANDLE, PropertyNames: *mut *mut *mut c_char) -> u32;
    pub fn cmsIT8EnumPropertyMulti(hIT8: cmsHANDLE, cProp: *const c_char, SubpropertyNames: *mut *mut *const c_char) -> u32;
    pub fn cmsIT8GetDataRowCol(hIT8: cmsHANDLE, row: c_int, col: c_int) -> *const c_char;
    pub fn cmsIT8GetDataRowColDbl(hIT8: cmsHANDLE, row: c_int, col: c_int) -> f64;
    pub fn cmsIT8SetDataRowCol(hIT8: cmsHANDLE, row: c_int, col: c_int, Val: *const c_char) -> cmsBool;
    pub fn cmsIT8SetDataRowColDbl(hIT8: cmsHANDLE, row: c_int, col: c_int, Val: f64) -> cmsBool;
    pub fn cmsIT8GetData(hIT8: cmsHANDLE, cPatch: *const c_char, cSample: *const c_char) -> *const c_char;
    pub fn cmsIT8GetDataDbl(hIT8: cmsHANDLE, cPatch: *const c_char, cSample: *const c_char) -> f64;
    pub fn cmsIT8SetData(hIT8: cmsHANDLE, cPatch: *const c_char, cSample: *const c_char, Val: *const c_char) -> cmsBool;
    pub fn cmsIT8SetDataDbl(hIT8: cmsHANDLE, cPatch: *const c_char, cSample: *const c_char, Val: f64) -> cmsBool;
    pub fn cmsIT8FindDataFormat(hIT8: cmsHANDLE, cSample: *const c_char) -> c_int;
    pub fn cmsIT8SetDataFormat(hIT8: cmsHANDLE, n: c_int, Sample: *const c_char) -> cmsBool;
    pub fn cmsIT8EnumDataFormat(hIT8: cmsHANDLE, SampleNames: *mut *mut *mut c_char) -> c_int;
    pub fn cmsIT8GetPatchName(hIT8: cmsHANDLE, nPatch: c_int, buffer: *mut c_char) -> *const c_char;
    pub fn cmsIT8GetPatchByName(hIT8: cmsHANDLE, cPatch: *const c_char) -> c_int;
    pub fn cmsIT8SetTableByLabel(hIT8: cmsHANDLE, cSet: *const c_char, cField: *const c_char, ExpectedType: *const c_char) -> c_int;
    pub fn cmsIT8SetIndexColumn(hIT8: cmsHANDLE, cSample: *const c_char) -> cmsBool;
    pub fn cmsIT8DefineDblFormat(hIT8: cmsHANDLE, Formatter: *const c_char);
    pub fn cmsGBDAlloc(ContextID: cmsContext) -> cmsHANDLE;
    pub fn cmsGBDFree(hGBD: cmsHANDLE);
    pub fn cmsGDBAddPoint(hGBD: cmsHANDLE, Lab: *const cmsCIELab) -> cmsBool;
    pub fn cmsGDBCompute(hGDB: cmsHANDLE, dwFlags: u32) -> cmsBool;
    pub fn cmsGDBCheckPoint(hGBD: cmsHANDLE, Lab: *const cmsCIELab) -> cmsBool;
    pub fn cmsDetectBlackPoint(BlackPoint: *mut cmsCIEXYZ, hProfile: cmsHPROFILE, Intent: u32, dwFlags: u32) -> cmsBool;
    pub fn cmsDetectDestinationBlackPoint(BlackPoint: *mut cmsCIEXYZ, hProfile: cmsHPROFILE, Intent: u32, dwFlags: u32) -> cmsBool;
    pub fn cmsDetectTAC(hProfile: cmsHPROFILE) -> f64;
    pub fn cmsDesaturateLab(Lab: *mut cmsCIELab, amax: f64, amin: f64, bmax: f64, bmin: f64) -> cmsBool;
}
