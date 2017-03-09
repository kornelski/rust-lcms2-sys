#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

// Based on:
//
//  Little Color Management System
//  Copyright (c) 1998-2014 Marti Maria Saguer
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
//---------------------------------------------------------------------------------
//
// Version 2.7

use std::os::raw::{c_char, c_int, c_long, c_void};
use libc;
use std::mem;
use libc::FILE;
use std::default::Default;

pub type tm = libc::tm;
pub type wchar_t = libc::wchar_t;
pub type Signature = u32;
pub type S15Fixed16Number = i32;
pub type Bool = c_int;


// D50 XYZ normalized to Y=1.0
pub const D50X: f64 = 0.9642;
pub const D50Y: f64 = 1.0;
pub const D50Z: f64 = 0.8249;

// V4 perceptual black
pub const PERCEPTUAL_BLACK_X: f64 = 0.00336;
pub const PERCEPTUAL_BLACK_Y: f64 = 0.0034731;
pub const PERCEPTUAL_BLACK_Z: f64 = 0.00287;

// Definitions in ICC spec
pub const MagicNumber: Signature = 0x61637370; // 'acsp'
pub const lcmsSignature: Signature = 0x6c636d73; // 'lcms'

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum TagTypeSignature {
    SigChromaticityType                  = 0x6368726D,  // 'chrm'
    SigColorantOrderType                 = 0x636C726F,  // 'clro'
    SigColorantTableType                 = 0x636C7274,  // 'clrt'
    SigCrdInfoType                       = 0x63726469,  // 'crdi'
    SigCurveType                         = 0x63757276,  // 'curv'
    SigDataType                          = 0x64617461,  // 'data'
    SigDictType                          = 0x64696374,  // 'dict'
    SigDateTimeType                      = 0x6474696D,  // 'dtim'
    SigDeviceSettingsType                = 0x64657673,  // 'devs'
    SigLut16Type                         = 0x6d667432,  // 'mft2'
    SigLut8Type                          = 0x6d667431,  // 'mft1'
    SigLutAtoBType                       = 0x6d414220,  // 'mAB '
    SigLutBtoAType                       = 0x6d424120,  // 'mBA '
    SigMeasurementType                   = 0x6D656173,  // 'meas'
    SigMultiLocalizedUnicodeType         = 0x6D6C7563,  // 'mluc'
    SigMultiProcessElementType           = 0x6D706574,  // 'mpet'
    SigNamedColorType                    = 0x6E636F6C,  // 'ncol' -- DEPRECATED!
    SigNamedColor2Type                   = 0x6E636C32,  // 'ncl2'
    SigParametricCurveType               = 0x70617261,  // 'para'
    SigProfileSequenceDescType           = 0x70736571,  // 'pseq'
    SigProfileSequenceIdType             = 0x70736964,  // 'psid'
    SigResponseCurveSet16Type            = 0x72637332,  // 'rcs2'
    SigS15Fixed16ArrayType               = 0x73663332,  // 'sf32'
    SigScreeningType                     = 0x7363726E,  // 'scrn'
    SigSignatureType                     = 0x73696720,  // 'sig '
    SigTextType                          = 0x74657874,  // 'text'
    SigTextDescriptionType               = 0x64657363,  // 'desc'
    SigU16Fixed16ArrayType               = 0x75663332,  // 'uf32'
    SigUcrBgType                         = 0x62666420,  // 'bfd '
    SigUInt16ArrayType                   = 0x75693136,  // 'ui16'
    SigUInt32ArrayType                   = 0x75693332,  // 'ui32'
    SigUInt64ArrayType                   = 0x75693634,  // 'ui64'
    SigUInt8ArrayType                    = 0x75693038,  // 'ui08'
    SigVcgtType                          = 0x76636774,  // 'vcgt'
    SigViewingConditionsType             = 0x76696577,  // 'view'
    SigXYZType                           = 0x58595A20   // 'XYZ '
}

pub const SigBlueMatrixColumnTag: TagSignature = TagSignature::SigBlueColorantTag;
pub const SigGreenMatrixColumnTag: TagSignature = TagSignature::SigGreenColorantTag;
pub const SigRedMatrixColumnTag: TagSignature = TagSignature::SigRedColorantTag;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum TagSignature {
    SigAToB0Tag                          = 0x41324230,  // 'A2B0'
    SigAToB1Tag                          = 0x41324231,  // 'A2B1'
    SigAToB2Tag                          = 0x41324232,  // 'A2B2'
    SigBlueColorantTag                   = 0x6258595A,  // 'bXYZ'
    SigBlueTRCTag                        = 0x62545243,  // 'bTRC'
    SigBToA0Tag                          = 0x42324130,  // 'B2A0'
    SigBToA1Tag                          = 0x42324131,  // 'B2A1'
    SigBToA2Tag                          = 0x42324132,  // 'B2A2'
    SigCalibrationDateTimeTag            = 0x63616C74,  // 'calt'
    SigCharTargetTag                     = 0x74617267,  // 'targ'
    SigChromaticAdaptationTag            = 0x63686164,  // 'chad'
    SigChromaticityTag                   = 0x6368726D,  // 'chrm'
    SigColorantOrderTag                  = 0x636C726F,  // 'clro'
    SigColorantTableTag                  = 0x636C7274,  // 'clrt'
    SigColorantTableOutTag               = 0x636C6F74,  // 'clot'
    SigColorimetricIntentImageStateTag   = 0x63696973,  // 'ciis'
    SigCopyrightTag                      = 0x63707274,  // 'cprt'
    SigCrdInfoTag                        = 0x63726469,  // 'crdi'
    SigDataTag                           = 0x64617461,  // 'data'
    SigDateTimeTag                       = 0x6474696D,  // 'dtim'
    SigDeviceMfgDescTag                  = 0x646D6E64,  // 'dmnd'
    SigDeviceModelDescTag                = 0x646D6464,  // 'dmdd'
    SigDeviceSettingsTag                 = 0x64657673,  // 'devs'
    SigDToB0Tag                          = 0x44324230,  // 'D2B0'
    SigDToB1Tag                          = 0x44324231,  // 'D2B1'
    SigDToB2Tag                          = 0x44324232,  // 'D2B2'
    SigDToB3Tag                          = 0x44324233,  // 'D2B3'
    SigBToD0Tag                          = 0x42324430,  // 'B2D0'
    SigBToD1Tag                          = 0x42324431,  // 'B2D1'
    SigBToD2Tag                          = 0x42324432,  // 'B2D2'
    SigBToD3Tag                          = 0x42324433,  // 'B2D3'
    SigGamutTag                          = 0x67616D74,  // 'gamt'
    SigGrayTRCTag                        = 0x6b545243,  // 'kTRC'
    SigGreenColorantTag                  = 0x6758595A,  // 'gXYZ'
    SigGreenTRCTag                       = 0x67545243,  // 'gTRC'
    SigLuminanceTag                      = 0x6C756D69,  // 'lumi'
    SigMeasurementTag                    = 0x6D656173,  // 'meas'
    SigMediaBlackPointTag                = 0x626B7074,  // 'bkpt'
    SigMediaWhitePointTag                = 0x77747074,  // 'wtpt'
    SigNamedColorTag                     = 0x6E636F6C,  // 'ncol' // Deprecated by the ICC
    SigNamedColor2Tag                    = 0x6E636C32,  // 'ncl2'
    SigOutputResponseTag                 = 0x72657370,  // 'resp'
    SigPerceptualRenderingIntentGamutTag = 0x72696730,  // 'rig0'
    SigPreview0Tag                       = 0x70726530,  // 'pre0'
    SigPreview1Tag                       = 0x70726531,  // 'pre1'
    SigPreview2Tag                       = 0x70726532,  // 'pre2'
    SigProfileDescriptionTag             = 0x64657363,  // 'desc'
    SigProfileDescriptionMLTag           = 0x6473636D,  // 'dscm'
    SigProfileSequenceDescTag            = 0x70736571,  // 'pseq'
    SigProfileSequenceIdTag              = 0x70736964,  // 'psid'
    SigPs2CRD0Tag                        = 0x70736430,  // 'psd0'
    SigPs2CRD1Tag                        = 0x70736431,  // 'psd1'
    SigPs2CRD2Tag                        = 0x70736432,  // 'psd2'
    SigPs2CRD3Tag                        = 0x70736433,  // 'psd3'
    SigPs2CSATag                         = 0x70733273,  // 'ps2s'
    SigPs2RenderingIntentTag             = 0x70733269,  // 'ps2i'
    SigRedColorantTag                    = 0x7258595A,  // 'rXYZ'
    SigRedTRCTag                         = 0x72545243,  // 'rTRC'
    SigSaturationRenderingIntentGamutTag = 0x72696732,  // 'rig2'
    SigScreeningDescTag                  = 0x73637264,  // 'scrd'
    SigScreeningTag                      = 0x7363726E,  // 'scrn'
    SigTechnologyTag                     = 0x74656368,  // 'tech'
    SigUcrBgTag                          = 0x62666420,  // 'bfd '
    SigViewingCondDescTag                = 0x76756564,  // 'vued'
    SigViewingConditionsTag              = 0x76696577,  // 'view'
    SigVcgtTag                           = 0x76636774,  // 'vcgt'
    SigMetaTag                           = 0x6D657461,  // 'meta'
    SigArgyllArtsTag                     = 0x61727473,  // 'arts'
}
pub use self::TagSignature::*;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum TechnologySignature {
    SigDigitalCamera                     = 0x6463616D,  // 'dcam'
    SigFilmScanner                       = 0x6673636E,  // 'fscn'
    SigReflectiveScanner                 = 0x7273636E,  // 'rscn'
    SigInkJetPrinter                     = 0x696A6574,  // 'ijet'
    SigThermalWaxPrinter                 = 0x74776178,  // 'twax'
    SigElectrophotographicPrinter        = 0x6570686F,  // 'epho'
    SigElectrostaticPrinter              = 0x65737461,  // 'esta'
    SigDyeSublimationPrinter             = 0x64737562,  // 'dsub'
    SigPhotographicPaperPrinter          = 0x7270686F,  // 'rpho'
    SigFilmWriter                        = 0x6670726E,  // 'fprn'
    SigVideoMonitor                      = 0x7669646D,  // 'vidm'
    SigVideoCamera                       = 0x76696463,  // 'vidc'
    SigProjectionTelevision              = 0x706A7476,  // 'pjtv'
    SigCRTDisplay                        = 0x43525420,  // 'CRT '
    SigPMDisplay                         = 0x504D4420,  // 'PMD '
    SigAMDisplay                         = 0x414D4420,  // 'AMD '
    SigPhotoCD                           = 0x4B504344,  // 'KPCD'
    SigPhotoImageSetter                  = 0x696D6773,  // 'imgs'
    SigGravure                           = 0x67726176,  // 'grav'
    SigOffsetLithography                 = 0x6F666673,  // 'offs'
    SigSilkscreen                        = 0x73696C6B,  // 'silk'
    SigFlexography                       = 0x666C6578,  // 'flex'
    SigMotionPictureFilmScanner          = 0x6D706673,  // 'mpfs'
    SigMotionPictureFilmRecorder         = 0x6D706672,  // 'mpfr'
    SigDigitalMotionPictureCamera        = 0x646D7063,  // 'dmpc'
    SigDigitalCinemaProjector            = 0x64636A70   // 'dcpj'
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum ColorSpaceSignature {
    SigXYZData                           = 0x58595A20,  // 'XYZ '
    SigLabData                           = 0x4C616220,  // 'Lab '
    SigLuvData                           = 0x4C757620,  // 'Luv '
    SigYCbCrData                         = 0x59436272,  // 'YCbr'
    SigYxyData                           = 0x59787920,  // 'Yxy '
    SigRgbData                           = 0x52474220,  // 'RGB '
    SigGrayData                          = 0x47524159,  // 'GRAY'
    SigHsvData                           = 0x48535620,  // 'HSV '
    SigHlsData                           = 0x484C5320,  // 'HLS '
    SigCmykData                          = 0x434D594B,  // 'CMYK'
    SigCmyData                           = 0x434D5920,  // 'CMY '
    SigMCH1Data                          = 0x4D434831,  // 'MCH1'
    SigMCH2Data                          = 0x4D434832,  // 'MCH2'
    SigMCH3Data                          = 0x4D434833,  // 'MCH3'
    SigMCH4Data                          = 0x4D434834,  // 'MCH4'
    SigMCH5Data                          = 0x4D434835,  // 'MCH5'
    SigMCH6Data                          = 0x4D434836,  // 'MCH6'
    SigMCH7Data                          = 0x4D434837,  // 'MCH7'
    SigMCH8Data                          = 0x4D434838,  // 'MCH8'
    SigMCH9Data                          = 0x4D434839,  // 'MCH9'
    SigMCHAData                          = 0x4D434841,  // 'MCHA'
    SigMCHBData                          = 0x4D434842,  // 'MCHB'
    SigMCHCData                          = 0x4D434843,  // 'MCHC'
    SigMCHDData                          = 0x4D434844,  // 'MCHD'
    SigMCHEData                          = 0x4D434845,  // 'MCHE'
    SigMCHFData                          = 0x4D434846,  // 'MCHF'
    SigNamedData                         = 0x6e6d636c,  // 'nmcl'
    Sig1colorData                        = 0x31434C52,  // '1CLR'
    Sig2colorData                        = 0x32434C52,  // '2CLR'
    Sig3colorData                        = 0x33434C52,  // '3CLR'
    Sig4colorData                        = 0x34434C52,  // '4CLR'
    Sig5colorData                        = 0x35434C52,  // '5CLR'
    Sig6colorData                        = 0x36434C52,  // '6CLR'
    Sig7colorData                        = 0x37434C52,  // '7CLR'
    Sig8colorData                        = 0x38434C52,  // '8CLR'
    Sig9colorData                        = 0x39434C52,  // '9CLR'
    Sig10colorData                       = 0x41434C52,  // 'ACLR'
    Sig11colorData                       = 0x42434C52,  // 'BCLR'
    Sig12colorData                       = 0x43434C52,  // 'CCLR'
    Sig13colorData                       = 0x44434C52,  // 'DCLR'
    Sig14colorData                       = 0x45434C52,  // 'ECLR'
    Sig15colorData                       = 0x46434C52,  // 'FCLR'
    SigLuvKData                          = 0x4C75764B   // 'LuvK'
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum ProfileClassSignature {
    SigInputClass                        = 0x73636E72,  // 'scnr'
    SigDisplayClass                      = 0x6D6E7472,  // 'mntr'
    SigOutputClass                       = 0x70727472,  // 'prtr'
    SigLinkClass                         = 0x6C696E6B,  // 'link'
    SigAbstractClass                     = 0x61627374,  // 'abst'
    SigColorSpaceClass                   = 0x73706163,  // 'spac'
    SigNamedColorClass                   = 0x6e6d636c   // 'nmcl'
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum PlatformSignature {
    SigMacintosh                         = 0x4150504C,  // 'APPL'
    SigMicrosoft                         = 0x4D534654,  // 'MSFT'
    SigSolaris                           = 0x53554E57,  // 'SUNW'
    SigSGI                               = 0x53474920,  // 'SGI '
    SigTaligent                          = 0x54474E54,  // 'TGNT'
    SigUnices                            = 0x2A6E6978   // '*nix'   // From argyll -- Not official
}

pub const SigPerceptualReferenceMediumGamut:u32 =         0x70726d67;  //'prmg'

// For SigColorimetricIntentImageStateTag
pub const SigSceneColorimetryEstimates:u32 =              0x73636F65;  //'scoe'
pub const SigSceneAppearanceEstimates:u32 =               0x73617065;  //'sape'
pub const SigFocalPlaneColorimetryEstimates:u32 =         0x66706365;  //'fpce'
pub const SigReflectionHardcopyOriginalColorimetry:u32 =  0x72686F63;  //'rhoc'
pub const SigReflectionPrintOutputColorimetry:u32 =       0x72706F63;  //'rpoc'

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum StageSignature {
    SigCurveSetElemType              = 0x63767374,  //'cvst'
    SigMatrixElemType                = 0x6D617466,  //'matf'
    SigCLutElemType                  = 0x636C7574,  //'clut'

    SigBAcsElemType                  = 0x62414353,  // 'bACS'
    SigEAcsElemType                  = 0x65414353,  // 'eACS'

    // Custom from here, not in the ICC Spec
    SigXYZ2LabElemType               = 0x6C327820,  // 'l2x '
    SigLab2XYZElemType               = 0x78326C20,  // 'x2l '
    SigNamedColorElemType            = 0x6E636C20,  // 'ncl '
    SigLabV2toV4                     = 0x32203420,  // '2 4 '
    SigLabV4toV2                     = 0x34203220,  // '4 2 '

    // Identities
    SigIdentityElemType              = 0x69646E20,  // 'idn '

    // Float to floatPCS
    SigLab2FloatPCS                  = 0x64326C20,  // 'd2l '
    SigFloatPCS2Lab                  = 0x6C326420,  // 'l2d '
    SigXYZ2FloatPCS                  = 0x64327820,  // 'd2x '
    SigFloatPCS2XYZ                  = 0x78326420,  // 'x2d '
    SigClipNegativesElemType         = 0x636c7020   // 'clp '
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum CurveSegSignature {
    SigFormulaCurveSeg               = 0x70617266, // 'parf'
    SigSampledCurveSeg               = 0x73616D66, // 'samf'
    SigSegmentedCurve                = 0x63757266  // 'curf'
}

pub const SigStatusA:u32 =                    0x53746141; //'StaA'
pub const SigStatusE:u32 =                    0x53746145; //'StaE'
pub const SigStatusI:u32 =                    0x53746149; //'StaI'
pub const SigStatusT:u32 =                    0x53746154; //'StaT'
pub const SigStatusM:u32 =                    0x5374614D; //'StaM'
pub const SigDN:u32 =                         0x444E2020; //'DN  '
pub const SigDNP:u32 =                        0x444E2050; //'DN P'
pub const SigDNN:u32 =                        0x444E4E20; //'DNN '
pub const SigDNNP:u32 =                       0x444E4E50; //'DNNP'

// Device attributes, currently defined values correspond to the low 4 bytes
// of the 8 byte attribute quantity
pub const Reflective:u32 =     0;
pub const Transparency:u32 =   1;
pub const Glossy:u32 =         0;
pub const Matte:u32 =          2;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ICCData {
    pub len: u32,
    pub flag: u32,
    pub data: [u8; 1],
}

impl Default for ICCData {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct DateTimeNumber {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}

impl Default for DateTimeNumber {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct EncodedXYZNumber {
    pub X: S15Fixed16Number,
    pub Y: S15Fixed16Number,
    pub Z: S15Fixed16Number,
}

impl Default for EncodedXYZNumber {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Debug)]
pub struct ProfileID {
    pub ID32: [u32; 4],
}

impl Default for ProfileID {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ICCHeader {
    pub size: u32,
    pub cmmId: Signature,
    pub version: u32,
    pub deviceClass: ProfileClassSignature,
    pub colorSpace: ColorSpaceSignature,
    pub pcs: ColorSpaceSignature,
    pub date: DateTimeNumber,
    pub magic: Signature,
    pub platform: PlatformSignature,
    pub flags: u32,
    pub manufacturer: Signature,
    pub model: u32,
    pub attributes: u64,
    pub renderingIntent: Intent,
    pub illuminant: EncodedXYZNumber,
    pub creator: Signature,
    pub profileID: ProfileID,
    pub reserved: [i8; 28],
}
impl Default for ICCHeader {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct TagBase {
    pub sig: TagTypeSignature,
    pub reserved: [i8; 4],
}
impl Default for TagBase {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct TagEntry {
    pub sig: TagSignature,
    pub offset: u32,
    pub size: u32,
}
impl Default for TagEntry {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub type HANDLE = *mut c_void;
pub enum _HPROFILE {}
pub type HPROFILE = *mut _HPROFILE;
pub enum _HTRANSFORM {}
pub type HTRANSFORM = *mut _HTRANSFORM;

pub const MAXCHANNELS: usize =  16;                // Maximum number of channels in ICC profiles


// Format of pixel is defined by one UInt32Number, using bit fields as follows
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

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum PixelFormat {
    GRAY_8 = 196617,
    GRAY_8_REV = 204809,
    GRAY_16 = 196618,
    GRAY_16_REV = 204810,
    GRAY_16_SE = 198666,
    GRAYA_8 = 196745,
    GRAYA_16 = 196746,
    GRAYA_16_SE = 198794,
    GRAYA_8_PLANAR = 200841,
    GRAYA_16_PLANAR = 200842,
    RGB_8 = 262169,
    RGB_8_PLANAR = 266265,
    BGR_8 = 263193,
    BGR_8_PLANAR = 267289,
    RGB_16 = 262170,
    RGB_16_PLANAR = 266266,
    RGB_16_SE = 264218,
    BGR_16 = 263194,
    BGR_16_PLANAR = 267290,
    BGR_16_SE = 265242,
    RGBA_8 = 262297,
    RGBA_8_PLANAR = 266393,
    RGBA_16 = 262298,
    RGBA_16_PLANAR = 266394,
    RGBA_16_SE = 264346,
    ARGB_8 = 278681,
    ARGB_8_PLANAR = 282777,
    ARGB_16 = 278682,
    ABGR_8 = 263321,
    ABGR_8_PLANAR = 267417,
    ABGR_16 = 263322,
    ABGR_16_PLANAR = 267418,
    ABGR_16_SE = 265370,
    BGRA_8 = 279705,
    BGRA_8_PLANAR = 283801,
    BGRA_16 = 279706,
    BGRA_16_SE = 281754,
    CMY_8 = 327705,
    CMY_8_PLANAR = 331801,
    CMY_16 = 327706,
    CMY_16_PLANAR = 331802,
    CMY_16_SE = 329754,
    CMYK_8 = 393249,
    CMYKA_8 = 393377,
    CMYK_8_REV = 401441,
    CMYK_8_PLANAR = 397345,
    CMYK_16 = 393250,
    CMYK_16_REV = 401442,
    CMYK_16_PLANAR = 397346,
    CMYK_16_SE = 395298,
    KYMC_8 = 394273,
    KYMC_16 = 394274,
    KYMC_16_SE = 396322,
    KCMY_8 = 409633,
    KCMY_8_REV = 417825,
    KCMY_16 = 409634,
    KCMY_16_REV = 417826,
    KCMY_16_SE = 411682,
    CMYK5_8 = 1245225,
    CMYK5_16 = 1245226,
    CMYK5_16_SE = 1247274,
    KYMC5_8 = 1246249,
    KYMC5_16 = 1246250,
    KYMC5_16_SE = 1248298,
    CMYK6_8 = 1310769,
    CMYK6_8_PLANAR = 1314865,
    CMYK6_16 = 1310770,
    CMYK6_16_PLANAR = 1314866,
    CMYK6_16_SE = 1312818,
    CMYK7_8 = 1376313,
    CMYK7_16 = 1376314,
    CMYK7_16_SE = 1378362,
    KYMC7_8 = 1377337,
    KYMC7_16 = 1377338,
    KYMC7_16_SE = 1379386,
    CMYK8_8 = 1441857,
    CMYK8_16 = 1441858,
    CMYK8_16_SE = 1443906,
    KYMC8_8 = 1442881,
    KYMC8_16 = 1442882,
    KYMC8_16_SE = 1444930,
    CMYK9_8 = 1507401,
    CMYK9_16 = 1507402,
    CMYK9_16_SE = 1509450,
    KYMC9_8 = 1508425,
    KYMC9_16 = 1508426,
    KYMC9_16_SE = 1510474,
    CMYK10_8 = 1572945,
    CMYK10_16 = 1572946,
    CMYK10_16_SE = 1574994,
    KYMC10_8 = 1573969,
    KYMC10_16 = 1573970,
    KYMC10_16_SE = 1576018,
    CMYK11_8 = 1638489,
    CMYK11_16 = 1638490,
    CMYK11_16_SE = 1640538,
    KYMC11_8 = 1639513,
    KYMC11_16 = 1639514,
    KYMC11_16_SE = 1641562,
    CMYK12_8 = 1704033,
    CMYK12_16 = 1704034,
    CMYK12_16_SE = 1706082,
    KYMC12_8 = 1705057,
    KYMC12_16 = 1705058,
    KYMC12_16_SE = 1707106,
    XYZ_16 = 589850,
    Lab_8 = 655385,
    LabV2_8 = 1966105,
    ALab_8 = 671897,
    ALabV2_8 = 1982617,
    Lab_16 = 655386,
    LabV2_16 = 1966106,
    Yxy_16 = 917530,
    YCbCr_8 = 458777,
    YCbCr_8_PLANAR = 462873,
    YCbCr_16 = 458778,
    YCbCr_16_PLANAR = 462874,
    YCbCr_16_SE = 460826,
    YUV_8 = 524313,
    YUV_8_PLANAR = 528409,
    YUV_16 = 524314,
    YUV_16_PLANAR = 528410,
    YUV_16_SE = 526362,
    HLS_8 = 851993,
    HLS_8_PLANAR = 856089,
    HLS_16 = 851994,
    HLS_16_PLANAR = 856090,
    HLS_16_SE = 854042,
    HSV_8 = 786457,
    HSV_8_PLANAR = 790553,
    HSV_16 = 786458,
    HSV_16_PLANAR = 790554,
    HSV_16_SE = 788506,
    NAMED_COLOR_INDEX = 10,
    XYZ_FLT = 4784156,
    Lab_FLT = 4849692,
    LabA_FLT = 4849820,
    GRAY_FLT = 4390924,
    RGB_FLT = 4456476,
    RGBA_FLT = 4456604,
    ARGB_FLT = 4472988,
    BGR_FLT = 4457500,
    BGRA_FLT = 4474012,
    CMYK_FLT = 4587556,
    XYZ_DBL = 4784152,
    Lab_DBL = 4849688,
    GRAY_DBL = 4390920,
    RGB_DBL = 4456472,
    BGR_DBL = 4457496,
    CMYK_DBL = 4587552,
    GRAY_HALF_FLT = 4390922,
    RGB_HALF_FLT = 4456474,
    RGBA_HALF_FLT = 4456602,
    CMYK_HALF_FLT = 4587554,
    ARGB_HALF_FLT = 4472986,
    BGR_HALF_FLT = 4457498,
    BGRA_HALF_FLT = 4474010,
}

impl PixelFormat {
    pub fn float(&self) -> usize {
        (((*self as u32) >> 22) & 1) as usize
    }
    pub fn optimized(&self) -> usize {
        (((*self as u32) >> 21) & 1) as usize
    }
    pub fn colorspace(&self) -> usize {
        (((*self as u32) >> 16) & 31) as usize
    }
    pub fn swapfirst(&self) -> usize {
        (((*self as u32) >> 14) & 1) as usize
    }
    pub fn flavor(&self) -> usize {
        (((*self as u32) >> 13) & 1) as usize
    }
    pub fn planar(&self) -> bool {
        (((*self as u32) >> 12) & 1) != 0
    }
    pub fn endian16(&self) -> usize {
        (((*self as u32) >> 11) & 1) as usize
    }
    pub fn doswap(&self) -> usize {
        (((*self as u32) >> 10) & 1) as usize
    }
    pub fn extra(&self) -> usize {
        (((*self as u32) >> 7) & 7) as usize
    }
    pub fn channels(&self) -> usize {
        (((*self as u32) >> 3) & 15) as usize
    }
    pub fn bytes_per_channel(&self) -> usize {
        ((*self as u32) & 7) as usize
    }

    pub fn bytes_per_pixel(&self) -> usize {
        self.bytes_per_channel() * (self.extra() + self.channels())
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
impl Default for CIEXYZ {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}
impl Default for CIExyY {
    fn default() -> Self { CIExyY{x:0., y:0., Y:1.} }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}
impl Default for CIELab {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CIELCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}
impl Default for CIELCh {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct JCh {
    pub J: f64,
    pub C: f64,
    pub h: f64,
}
impl Default for JCh {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CIEXYZTRIPLE {
    pub Red: CIEXYZ,
    pub Green: CIEXYZ,
    pub Blue: CIEXYZ,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CIExyYTRIPLE {
    pub Red: CIExyY,
    pub Green: CIExyY,
    pub Blue: CIExyY,
}

// Illuminant types for structs below
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum IlluminantType {
    UNKNOWN = 0x0000000,
    D50 =     0x0000001,
    D65 =     0x0000002,
    D93 =     0x0000003,
    F2 =      0x0000004,
    D55 =     0x0000005,
    A =       0x0000006,
    E =       0x0000007,
    F8 =      0x0000008,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ICCMeasurementConditions {
    pub Observer: u32,
    pub Backing: CIEXYZ,
    pub Geometry: u32,
    pub Flare: f64,
    pub IlluminantType: IlluminantType,
}
impl Default for ICCMeasurementConditions {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ICCViewingConditions {
    pub IlluminantXYZ: CIEXYZ,
    pub SurroundXYZ: CIEXYZ,
    pub IlluminantType: IlluminantType,
}
impl Default for ICCViewingConditions {
    fn default() -> Self { unsafe { mem::zeroed() } }
}


#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(Debug)]
pub enum Surround {
    Avg       = 1,
    Dim       = 2,
    Dark      = 3,
    Cutsheet  = 4,
}

pub const D_CALCULATE: f64 = -1.;

pub enum _cmsContext_struct { }
pub type Context = *mut _cmsContext_struct;
pub type LogErrorHandlerFunction = ::std::option::Option<unsafe extern "C" fn(ContextID: Context, ErrorCode: u32, Text: *const c_char)>;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ViewingConditions {
    pub whitePoint: CIEXYZ,
    pub Yb: f64,
    pub La: f64,
    pub surround: Surround,
    pub D_value: f64,
}
impl Default for ViewingConditions {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CurveSegment {
    pub x0: f32,
    pub x1: f32,
    pub Type: i32,
    pub Params: [f64; 10],
    pub nGridPoints: u32,
    pub SampledPoints: *mut f32,
}
impl Default for CurveSegment {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub enum ToneCurve { }
pub enum Pipeline { }
pub enum Stage { }

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum StageLoc {
    AT_BEGIN = 0,
    AT_END = 1,
}
pub type SAMPLER16 = ::std::option::Option<unsafe extern "C" fn(In: *mut u16, Out: *mut u16, Cargo: *mut c_void) -> i32>;
pub type SAMPLERFLOAT = ::std::option::Option<unsafe extern "C" fn(In: *mut f32, Out: *mut f32, Cargo: *mut c_void) -> i32>;
pub enum MLU { }

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct UcrBg {
    pub Ucr: *mut ToneCurve,
    pub Bg: *mut ToneCurve,
    pub Desc: *mut MLU,
}

pub const PRINTER_DEFAULT_SCREENS: u32 =     0x0001;
pub const FREQUENCE_UNITS_LINES_CM: u32 =    0x0000;
pub const FREQUENCE_UNITS_LINES_INCH: u32 =  0x0002;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum SpotShape {
    UNKNOWN         = 0,
    PRINTER_DEFAULT = 1,
    ROUND           = 2,
    DIAMOND         = 3,
    ELLIPSE         = 4,
    LINE            = 5,
    SQUARE          = 6,
    CROSS           = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ScreeningChannel {
    pub Frequency: f64,
    pub ScreenAngle: f64,
    pub SpotShape: SpotShape,
}
impl Default for ScreeningChannel {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Screening {
    pub Flag: u32,
    pub nChannels: u32,
    pub Channels: [ScreeningChannel; 16],
}
impl Default for Screening {
    fn default() -> Self { unsafe { mem::zeroed() } }
}
pub enum NAMEDCOLORLIST { }

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct PSEQDESC {
    pub deviceMfg: Signature,
    pub deviceModel: Signature,
    pub attributes: u64,
    pub technology: TechnologySignature,
    pub ProfileID: ProfileID,
    pub Manufacturer: *mut MLU,
    pub Model: *mut MLU,
    pub Description: *mut MLU,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct SEQ {
    pub n: u32,
    pub ContextID: Context,
    pub seq: *mut PSEQDESC,
}

pub const EmbeddedProfileFalse: u32 =    0x00000000;
pub const EmbeddedProfileTrue: u32 =     0x00000001;
pub const UseAnywhere: u32 =             0x00000000;
pub const UseWithEmbeddedDataOnly: u32 = 0x00000002;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct DICTentry {
    pub Next: *mut DICTentry,
    pub DisplayName: *mut MLU,
    pub DisplayValue: *mut MLU,
    pub Name: *mut wchar_t,
    pub Value: *mut wchar_t,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum InfoType {
    Description = 0,
    Manufacturer = 1,
    Model = 2,
    Copyright = 3,
}
pub enum IOHANDLER { }


#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum Intent {
    // ICC Intents
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,

    // non-icc intents
    PreserveKOnlyPerceptual = 10,
    PreserveKOnlyRelativeColorimetric = 11,
    PreserveKOnlySaturation = 12,
    PreserveKPlanePerceptual = 13,
    PreserveKPlaneRelativeColorimetric = 14,
    PreserveKPlaneSaturation = 15,
}

// Flags

pub const FLAGS_NOCACHE: u32 =                  0x0040;    // Inhibit 1-pixel cache
pub const FLAGS_NOOPTIMIZE: u32 =               0x0100;    // Inhibit optimizations
pub const FLAGS_NULLTRANSFORM: u32 =            0x0200;    // Don't transform anyway

// Proofing flags
pub const FLAGS_GAMUTCHECK: u32 =               0x1000;    // Out of Gamut alarm
pub const FLAGS_SOFTPROOFING: u32 =             0x4000;    // Do softproofing

// Misc
pub const FLAGS_BLACKPOINTCOMPENSATION: u32 =   0x2000;
pub const FLAGS_NOWHITEONWHITEFIXUP: u32 =      0x0004;    // Don't fix scum dot
pub const FLAGS_HIGHRESPRECALC: u32 =           0x0400;    // Use more memory to give better accurancy
pub const FLAGS_LOWRESPRECALC: u32 =            0x0800;    // Use less memory to minimize resouces

// For devicelink creation
pub const FLAGS_8BITS_DEVICELINK: u32 =         0x0008;   // Create 8 bits devicelinks
pub const FLAGS_GUESSDEVICECLASS: u32 =         0x0020;   // Guess device class (for transform2devicelink)
pub const FLAGS_KEEP_SEQUENCE: u32 =            0x0080;   // Keep profile sequence for devicelink creation

// Specific to a particular optimizations
pub const FLAGS_FORCE_CLUT: u32 =               0x0002;    // Force CLUT optimization
pub const FLAGS_CLUT_POST_LINEARIZATION: u32 =  0x0001;    // create postlinearization tables if possible
pub const FLAGS_CLUT_PRE_LINEARIZATION: u32 =   0x0010;    // create prelinearization tables if possible

// Specific to unbounded mode
pub const FLAGS_NONEGATIVES: u32 =              0x8000;    // Prevent negative numbers in floating point transforms

pub const FLAGS_COPY_ALPHA: u32 =           0x04000000;    // Alpha channels are copied on cmsDoTransform()

// Fine-tune control over number of gridpoints
pub fn FLAGS_GRIDPOINTS(n: u32) -> u32 { ((n) & 0xFF) << 16 }

// CRD special
pub const FLAGS_NODEFAULTRESOURCEDEF: u32 =     0x01000000;

impl CIEXYZ {
    pub fn d50() -> &'static CIEXYZ {
        unsafe { cmsD50_XYZ() }
    }
}

impl CIExyY {
    pub fn d50() -> &'static CIExyY {
        unsafe { cmsD50_xyY() }
    }
}

impl From<CIEXYZ> for CIExyY {
    fn from(f: CIEXYZ) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsXYZ2xyY(&mut new, &f);
            new
        }
    }
}

impl From<CIExyY> for CIEXYZ {
    fn from(f: CIExyY) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsxyY2XYZ(&mut new, &f);
            new
        }
    }
}

impl From<CIELab> for CIELCh {
    fn from(f: CIELab) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsLab2LCh(&mut new, &f);
            new
        }
    }
}

impl From<CIELCh> for CIELab {
    fn from(f: CIELCh) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsLCh2Lab(&mut new, &f);
            new
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum PSResourceType {
    PS_RESOURCE_CSA = 0,
    PS_RESOURCE_CRD = 1,
}

extern "C" {
    pub fn cmsGetEncodedCMMversion() -> c_int;
    pub fn cmsstrcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn cmsfilelength(f: *mut FILE) -> c_long;
    pub fn cmsCreateContext(Plugin: *mut c_void, UserData: *mut c_void) -> Context;
    pub fn cmsDeleteContext(ContexID: Context);
    pub fn cmsDupContext(ContextID: Context, NewUserData: *mut c_void) -> Context;
    pub fn cmsGetContextUserData(ContextID: Context) -> *mut c_void;
    pub fn cmsPlugin(Plugin: *mut c_void) -> Bool;
    pub fn cmsPluginTHR(ContextID: Context, Plugin: *mut c_void) -> Bool;
    pub fn cmsUnregisterPlugins();
    pub fn cmsUnregisterPluginsTHR(ContextID: Context);
    pub fn cmsSetLogErrorHandler(Fn: LogErrorHandlerFunction);
    pub fn cmsSetLogErrorHandlerTHR(ContextID: Context, Fn: LogErrorHandlerFunction);
    pub fn cmsD50_XYZ() -> &'static CIEXYZ;
    pub fn cmsD50_xyY() -> &'static CIExyY;
    pub fn cmsXYZ2xyY(Dest: *mut CIExyY, Source: *const CIEXYZ);
    pub fn cmsxyY2XYZ(Dest: *mut CIEXYZ, Source: *const CIExyY);
    pub fn cmsXYZ2Lab(WhitePoint: *const CIEXYZ, Lab: *mut CIELab, xyz: *const CIEXYZ);
    pub fn cmsLab2XYZ(WhitePoint: *const CIEXYZ, xyz: *mut CIEXYZ, Lab: *const CIELab);
    pub fn cmsLab2LCh(LCh: *mut CIELCh, Lab: *const CIELab);
    pub fn cmsLCh2Lab(Lab: *mut CIELab, LCh: *const CIELCh);
    pub fn cmsLabEncoded2Float(Lab: *mut CIELab, wLab: *const u16);
    pub fn cmsLabEncoded2FloatV2(Lab: *mut CIELab, wLab: *const u16);
    pub fn cmsFloat2LabEncoded(wLab: *mut u16, Lab: *const CIELab);
    pub fn cmsFloat2LabEncodedV2(wLab: *mut u16, Lab: *const CIELab);
    pub fn cmsXYZEncoded2Float(fxyz: *mut CIEXYZ, XYZ: *const u16);
    pub fn cmsFloat2XYZEncoded(XYZ: *mut u16, fXYZ: *const CIEXYZ);
    pub fn cmsDeltaE(Lab1: *const CIELab, Lab2: *const CIELab) -> f64;
    pub fn cmsCIE94DeltaE(Lab1: *const CIELab, Lab2: *const CIELab) -> f64;
    pub fn cmsBFDdeltaE(Lab1: *const CIELab, Lab2: *const CIELab) -> f64;
    pub fn cmsCMCdeltaE(Lab1: *const CIELab, Lab2: *const CIELab, l: f64, c: f64) -> f64;
    pub fn cmsCIE2000DeltaE(Lab1: *const CIELab, Lab2: *const CIELab, Kl: f64, Kc: f64, Kh: f64) -> f64;
    pub fn cmsWhitePointFromTemp(WhitePoint: *mut CIExyY, TempK: f64) -> Bool;
    pub fn cmsTempFromWhitePoint(TempK: *mut f64, WhitePoint: *const CIExyY) -> Bool;
    pub fn cmsAdaptToIlluminant(Result: *mut CIEXYZ, SourceWhitePt: *const CIEXYZ, Illuminant: *const CIEXYZ, Value: *const CIEXYZ) -> Bool;
    pub fn cmsCIECAM02Init(ContextID: Context, pVC: *const ViewingConditions) -> HANDLE;
    pub fn cmsCIECAM02Done(hModel: HANDLE);
    pub fn cmsCIECAM02Forward(hModel: HANDLE, pIn: *const CIEXYZ, pOut: *mut JCh);
    pub fn cmsCIECAM02Reverse(hModel: HANDLE, pIn: *const JCh, pOut: *mut CIEXYZ);
    pub fn cmsBuildSegmentedToneCurve(ContextID: Context, nSegments: i32, Segments: *const CurveSegment) -> *mut ToneCurve;
    pub fn cmsBuildParametricToneCurve(ContextID: Context, Type: i32, Params: *const f64) -> *mut ToneCurve;
    pub fn cmsBuildGamma(ContextID: Context, Gamma: f64) -> *mut ToneCurve;
    pub fn cmsBuildTabulatedToneCurve16(ContextID: Context, nEntries: i32, values: *const u16) -> *mut ToneCurve;
    pub fn cmsBuildTabulatedToneCurveFloat(ContextID: Context, nEntries: u32, values: *const f32) -> *mut ToneCurve;
    pub fn cmsFreeToneCurve(Curve: *mut ToneCurve);
    pub fn cmsFreeToneCurveTriple(Curve: *mut *mut ToneCurve);
    pub fn cmsDupToneCurve(Src: *const ToneCurve) -> *mut ToneCurve;
    pub fn cmsReverseToneCurve(InGamma: *const ToneCurve) -> *mut ToneCurve;
    pub fn cmsReverseToneCurveEx(nResultSamples: i32, InGamma: *const ToneCurve) -> *mut ToneCurve;
    pub fn cmsJoinToneCurve(ContextID: Context, X: *const ToneCurve, Y: *const ToneCurve, nPoints: u32) -> *mut ToneCurve;
    pub fn cmsSmoothToneCurve(Tab: *mut ToneCurve, lambda: f64) -> Bool;
    pub fn cmsEvalToneCurveFloat(Curve: *const ToneCurve, v: f32) -> f32;
    pub fn cmsEvalToneCurve16(Curve: *const ToneCurve, v: u16) -> u16;
    pub fn cmsIsToneCurveMultisegment(InGamma: *const ToneCurve) -> Bool;
    pub fn cmsIsToneCurveLinear(Curve: *const ToneCurve) -> Bool;
    pub fn cmsIsToneCurveMonotonic(t: *const ToneCurve) -> Bool;
    pub fn cmsIsToneCurveDescending(t: *const ToneCurve) -> Bool;
    pub fn cmsGetToneCurveParametricType(t: *const ToneCurve) -> i32;
    pub fn cmsEstimateGamma(t: *const ToneCurve, Precision: f64) -> f64;
    pub fn cmsGetToneCurveEstimatedTableEntries(t: *const ToneCurve) -> u32;
    pub fn cmsGetToneCurveEstimatedTable(t: *const ToneCurve) -> *const u16;
    pub fn cmsPipelineAlloc(ContextID: Context, InputChannels: u32, OutputChannels: u32) -> *mut Pipeline;
    pub fn cmsPipelineFree(lut: *mut Pipeline);
    pub fn cmsPipelineDup(Orig: *const Pipeline) -> *mut Pipeline;
    pub fn cmsGetPipelineContextID(lut: *const Pipeline) -> Context;
    pub fn cmsPipelineInputChannels(lut: *const Pipeline) -> u32;
    pub fn cmsPipelineOutputChannels(lut: *const Pipeline) -> u32;
    pub fn cmsPipelineStageCount(lut: *const Pipeline) -> u32;
    pub fn cmsPipelineGetPtrToFirstStage(lut: *const Pipeline) -> *mut Stage;
    pub fn cmsPipelineGetPtrToLastStage(lut: *const Pipeline) -> *mut Stage;
    pub fn cmsPipelineEval16(In: *const u16, Out: *mut u16, lut: *const Pipeline);
    pub fn cmsPipelineEvalFloat(In: *const f32, Out: *mut f32, lut: *const Pipeline);
    pub fn cmsPipelineEvalReverseFloat(Target: *mut f32, Result: *mut f32, Hint: *mut f32, lut: *const Pipeline) -> Bool;
    pub fn cmsPipelineCat(l1: *mut Pipeline, l2: *const Pipeline) -> Bool;
    pub fn cmsPipelineSetSaveAs8bitsFlag(lut: *mut Pipeline, On: Bool) -> Bool;
    pub fn cmsPipelineInsertStage(lut: *mut Pipeline, loc: StageLoc, mpe: *mut Stage) -> c_int;
    pub fn cmsPipelineUnlinkStage(lut: *mut Pipeline, loc: StageLoc, mpe: *mut *mut Stage);
    pub fn cmsPipelineCheckAndRetreiveStages(Lut: *const Pipeline, n: u32, ...) -> Bool;
    pub fn cmsStageAllocIdentity(ContextID: Context, nChannels: u32) -> *mut Stage;
    pub fn cmsStageAllocToneCurves(ContextID: Context, nChannels: u32, Curves: *const *const ToneCurve) -> *mut Stage;
    pub fn cmsStageAllocMatrix(ContextID: Context, Rows: u32, Cols: u32, Matrix: *const f64, Offset: *const f64) -> *mut Stage;
    pub fn cmsStageAllocCLut16bit(ContextID: Context, nGridPoints: u32, inputChan: u32, outputChan: u32, Table: *const u16) -> *mut Stage;
    pub fn cmsStageAllocCLutFloat(ContextID: Context, nGridPoints: u32, inputChan: u32, outputChan: u32, Table: *const f32) -> *mut Stage;
    pub fn cmsStageAllocCLut16bitGranular(ContextID: Context, clutPoints: *const u32, inputChan: u32, outputChan: u32, Table: *const u16) -> *mut Stage;
    pub fn cmsStageAllocCLutFloatGranular(ContextID: Context, clutPoints: *const u32, inputChan: u32, outputChan: u32, Table: *const f32) -> *mut Stage;
    pub fn cmsStageDup(mpe: *mut Stage) -> *mut Stage;
    pub fn cmsStageFree(mpe: *mut Stage);
    pub fn cmsStageNext(mpe: *const Stage) -> *mut Stage;
    pub fn cmsStageInputChannels(mpe: *const Stage) -> u32;
    pub fn cmsStageOutputChannels(mpe: *const Stage) -> u32;
    pub fn cmsStageType(mpe: *const Stage) -> StageSignature;
    pub fn cmsStageData(mpe: *const Stage) -> *mut c_void;
    pub fn cmsStageSampleCLut16bit(mpe: *mut Stage, Sampler: SAMPLER16, Cargo: *mut c_void, dwFlags: u32) -> Bool;
    pub fn cmsStageSampleCLutFloat(mpe: *mut Stage, Sampler: SAMPLERFLOAT, Cargo: *mut c_void, dwFlags: u32) -> Bool;
    pub fn cmsSliceSpace16(nInputs: u32, clutPoints: *const u32, Sampler: SAMPLER16, Cargo: *mut c_void) -> Bool;
    pub fn cmsSliceSpaceFloat(nInputs: u32, clutPoints: *const u32, Sampler: SAMPLERFLOAT, Cargo: *mut c_void) -> Bool;
    pub fn cmsMLUalloc(ContextID: Context, nItems: u32) -> *mut MLU;
    pub fn cmsMLUfree(mlu: *mut MLU);
    pub fn cmsMLUdup(mlu: *const MLU) -> *mut MLU;
    pub fn cmsMLUsetASCII(mlu: *mut MLU, LanguageCode: *const c_char, CountryCode: *const c_char, ASCIIString: *const c_char) -> Bool;
    pub fn cmsMLUsetWide(mlu: *mut MLU, LanguageCode: *const c_char, CountryCode: *const c_char, WideString: *const wchar_t) -> Bool;
    pub fn cmsMLUgetASCII(mlu: *const MLU, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
    pub fn cmsMLUgetWide(mlu: *const MLU, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut wchar_t, BufferSize: u32) -> u32;
    pub fn cmsMLUgetTranslation(mlu: *const MLU, LanguageCode: *const c_char, CountryCode: *const c_char, ObtainedLanguage: *mut c_char, ObtainedCountry: *mut c_char) -> Bool;
    pub fn cmsMLUtranslationsCount(mlu: *const MLU) -> u32;
    pub fn cmsMLUtranslationsCodes(mlu: *const MLU, idx: u32, LanguageCode: *mut c_char, CountryCode: *mut c_char) -> Bool;
    pub fn cmsAllocNamedColorList(ContextID: Context, n: u32, ColorantCount: u32, Prefix: *const c_char, Suffix: *const c_char) -> *mut NAMEDCOLORLIST;
    pub fn cmsFreeNamedColorList(v: *mut NAMEDCOLORLIST);
    pub fn cmsDupNamedColorList(v: *const NAMEDCOLORLIST) -> *mut NAMEDCOLORLIST;
    pub fn cmsAppendNamedColor(v: *mut NAMEDCOLORLIST, Name: *const c_char, PCS: *mut u16, Colorant: *mut u16) -> Bool;
    pub fn cmsNamedColorCount(v: *const NAMEDCOLORLIST) -> u32;
    pub fn cmsNamedColorIndex(v: *const NAMEDCOLORLIST, Name: *const c_char) -> i32;
    pub fn cmsNamedColorInfo(NamedColorList: *const NAMEDCOLORLIST, nColor: u32, Name: *mut c_char, Prefix: *mut c_char, Suffix: *mut c_char, PCS: *mut u16, Colorant: *mut u16) -> Bool;
    pub fn cmsGetNamedColorList(xform: HTRANSFORM) -> *mut NAMEDCOLORLIST;
    pub fn cmsAllocProfileSequenceDescription(ContextID: Context, n: u32) -> *mut SEQ;
    pub fn cmsDupProfileSequenceDescription(pseq: *const SEQ) -> *mut SEQ;
    pub fn cmsFreeProfileSequenceDescription(pseq: *mut SEQ);
    pub fn cmsDictAlloc(ContextID: Context) -> HANDLE;
    pub fn cmsDictFree(hDict: HANDLE);
    pub fn cmsDictDup(hDict: HANDLE) -> HANDLE;
    pub fn cmsDictAddEntry(hDict: HANDLE, Name: *const wchar_t, Value: *const wchar_t, DisplayName: *const MLU, DisplayValue: *const MLU) -> Bool;
    pub fn cmsDictGetEntryList(hDict: HANDLE) -> *const DICTentry;
    pub fn cmsDictNextEntry(e: *const DICTentry) -> *const DICTentry;
    pub fn cmsCreateProfilePlaceholder(ContextID: Context) -> HPROFILE;
    pub fn cmsGetProfileContextID(hProfile: HPROFILE) -> Context;
    pub fn cmsGetTagCount(hProfile: HPROFILE) -> i32;
    pub fn cmsGetTagSignature(hProfile: HPROFILE, n: u32) -> TagSignature;
    pub fn cmsIsTag(hProfile: HPROFILE, sig: TagSignature) -> Bool;
    pub fn cmsReadTag(hProfile: HPROFILE, sig: TagSignature) -> *mut c_void;
    pub fn cmsWriteTag(hProfile: HPROFILE, sig: TagSignature, data: *const c_void) -> Bool;
    pub fn cmsLinkTag(hProfile: HPROFILE, sig: TagSignature, dest: TagSignature) -> Bool;
    pub fn cmsTagLinkedTo(hProfile: HPROFILE, sig: TagSignature) -> TagSignature;
    pub fn cmsReadRawTag(hProfile: HPROFILE, sig: TagSignature, Buffer: *mut c_void, BufferSize: u32) -> i32;
    pub fn cmsWriteRawTag(hProfile: HPROFILE, sig: TagSignature, data: *const c_void, Size: u32) -> Bool;
    pub fn cmsGetHeaderFlags(hProfile: HPROFILE) -> u32;
    pub fn cmsGetHeaderAttributes(hProfile: HPROFILE, Flags: *mut u64);
    pub fn cmsGetHeaderProfileID(hProfile: HPROFILE, ProfileID: *mut u8);
    pub fn cmsGetHeaderCreationDateTime(hProfile: HPROFILE, Dest: *mut tm) -> Bool;
    pub fn cmsGetHeaderRenderingIntent(hProfile: HPROFILE) -> u32;
    pub fn cmsSetHeaderFlags(hProfile: HPROFILE, Flags: u32);
    pub fn cmsGetHeaderManufacturer(hProfile: HPROFILE) -> u32;
    pub fn cmsSetHeaderManufacturer(hProfile: HPROFILE, manufacturer: u32);
    pub fn cmsGetHeaderCreator(hProfile: HPROFILE) -> u32;
    pub fn cmsGetHeaderModel(hProfile: HPROFILE) -> u32;
    pub fn cmsSetHeaderModel(hProfile: HPROFILE, model: u32);
    pub fn cmsSetHeaderAttributes(hProfile: HPROFILE, Flags: u64);
    pub fn cmsSetHeaderProfileID(hProfile: HPROFILE, ProfileID: *mut u8);
    pub fn cmsSetHeaderRenderingIntent(hProfile: HPROFILE, RenderingIntent: Intent);
    pub fn cmsGetPCS(hProfile: HPROFILE) -> ColorSpaceSignature;
    pub fn cmsSetPCS(hProfile: HPROFILE, pcs: ColorSpaceSignature);
    pub fn cmsGetColorSpace(hProfile: HPROFILE) -> ColorSpaceSignature;
    pub fn cmsSetColorSpace(hProfile: HPROFILE, sig: ColorSpaceSignature);
    pub fn cmsGetDeviceClass(hProfile: HPROFILE) -> ProfileClassSignature;
    pub fn cmsSetDeviceClass(hProfile: HPROFILE, sig: ProfileClassSignature);
    pub fn cmsSetProfileVersion(hProfile: HPROFILE, Version: f64);
    pub fn cmsGetProfileVersion(hProfile: HPROFILE) -> f64;
    pub fn cmsGetEncodedICCversion(hProfile: HPROFILE) -> u32;
    pub fn cmsSetEncodedICCversion(hProfile: HPROFILE, Version: u32);
    pub fn cmsIsIntentSupported(hProfile: HPROFILE, Intent: Intent, UsedDirection: u32) -> Bool;
    pub fn cmsIsMatrixShaper(hProfile: HPROFILE) -> Bool;
    pub fn cmsIsCLUT(hProfile: HPROFILE, Intent: Intent, UsedDirection: u32) -> Bool;
    pub fn _cmsICCcolorSpace(OurNotation: c_int) -> ColorSpaceSignature;
    pub fn _cmsLCMScolorSpace(ProfileSpace: ColorSpaceSignature) -> c_int;
    pub fn cmsChannelsOf(ColorSpace: ColorSpaceSignature) -> u32;
    pub fn cmsFormatterForColorspaceOfProfile(hProfile: HPROFILE, nBytes: u32, lIsFloat: Bool) -> u32;
    pub fn cmsFormatterForPCSOfProfile(hProfile: HPROFILE, nBytes: u32, lIsFloat: Bool) -> u32;
    pub fn cmsGetProfileInfo(hProfile: HPROFILE, Info: InfoType, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut wchar_t, BufferSize: u32) -> u32;
    pub fn cmsGetProfileInfoASCII(hProfile: HPROFILE, Info: InfoType, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
    pub fn cmsOpenIOhandlerFromFile(ContextID: Context, FileName: *const c_char, AccessMode: *const c_char) -> *mut IOHANDLER;
    pub fn cmsOpenIOhandlerFromStream(ContextID: Context, Stream: *mut FILE) -> *mut IOHANDLER;
    pub fn cmsOpenIOhandlerFromMem(ContextID: Context, Buffer: *mut c_void, size: u32, AccessMode: *const c_char) -> *mut IOHANDLER;
    pub fn cmsOpenIOhandlerFromNULL(ContextID: Context) -> *mut IOHANDLER;
    pub fn cmsGetProfileIOhandler(hProfile: HPROFILE) -> *mut IOHANDLER;
    pub fn cmsCloseIOhandler(io: *mut IOHANDLER) -> Bool;
    pub fn cmsMD5computeID(hProfile: HPROFILE) -> Bool;
    pub fn cmsOpenProfileFromFile(ICCProfile: *const c_char, sAccess: *const c_char) -> HPROFILE;
    pub fn cmsOpenProfileFromFileTHR(ContextID: Context, ICCProfile: *const c_char, sAccess: *const c_char) -> HPROFILE;
    pub fn cmsOpenProfileFromStream(ICCProfile: *mut FILE, sAccess: *const c_char) -> HPROFILE;
    pub fn cmsOpenProfileFromStreamTHR(ContextID: Context, ICCProfile: *mut FILE, sAccess: *const c_char) -> HPROFILE;
    pub fn cmsOpenProfileFromMem(MemPtr: *const c_void, dwSize: u32) -> HPROFILE;
    pub fn cmsOpenProfileFromMemTHR(ContextID: Context, MemPtr: *const c_void, dwSize: u32) -> HPROFILE;
    pub fn cmsOpenProfileFromIOhandlerTHR(ContextID: Context, io: *mut IOHANDLER) -> HPROFILE;
    pub fn cmsOpenProfileFromIOhandler2THR(ContextID: Context, io: *mut IOHANDLER, write: Bool) -> HPROFILE;
    pub fn cmsCloseProfile(hProfile: HPROFILE) -> Bool;
    pub fn cmsSaveProfileToFile(hProfile: HPROFILE, FileName: *const c_char) -> Bool;
    pub fn cmsSaveProfileToStream(hProfile: HPROFILE, Stream: *mut FILE) -> Bool;
    pub fn cmsSaveProfileToMem(hProfile: HPROFILE, MemPtr: *mut c_void, BytesNeeded: *mut u32) -> Bool;
    pub fn cmsSaveProfileToIOhandler(hProfile: HPROFILE, io: *mut IOHANDLER) -> u32;
    pub fn cmsCreateRGBProfileTHR(ContextID: Context, WhitePoint: *const CIExyY, Primaries: *const CIExyYTRIPLE, TransferFunction: *const *const ToneCurve) -> HPROFILE;
    pub fn cmsCreateRGBProfile(WhitePoint: *const CIExyY, Primaries: *const CIExyYTRIPLE, TransferFunction: *const *const ToneCurve) -> HPROFILE;
    pub fn cmsCreateGrayProfileTHR(ContextID: Context, WhitePoint: *const CIExyY, TransferFunction: *const ToneCurve) -> HPROFILE;
    pub fn cmsCreateGrayProfile(WhitePoint: *const CIExyY, TransferFunction: *const ToneCurve) -> HPROFILE;
    pub fn cmsCreateLinearizationDeviceLinkTHR(ContextID: Context, ColorSpace: ColorSpaceSignature, TransferFunctions: *const *const ToneCurve) -> HPROFILE;
    pub fn cmsCreateLinearizationDeviceLink(ColorSpace: ColorSpaceSignature, TransferFunctions: *const *const ToneCurve) -> HPROFILE;
    pub fn cmsCreateInkLimitingDeviceLinkTHR(ContextID: Context, ColorSpace: ColorSpaceSignature, Limit: f64) -> HPROFILE;
    pub fn cmsCreateInkLimitingDeviceLink(ColorSpace: ColorSpaceSignature, Limit: f64) -> HPROFILE;
    pub fn cmsCreateLab2ProfileTHR(ContextID: Context, WhitePoint: *const CIExyY) -> HPROFILE;
    pub fn cmsCreateLab2Profile(WhitePoint: *const CIExyY) -> HPROFILE;
    pub fn cmsCreateLab4ProfileTHR(ContextID: Context, WhitePoint: *const CIExyY) -> HPROFILE;
    pub fn cmsCreateLab4Profile(WhitePoint: *const CIExyY) -> HPROFILE;
    pub fn cmsCreateXYZProfileTHR(ContextID: Context) -> HPROFILE;
    pub fn cmsCreateXYZProfile() -> HPROFILE;
    pub fn cmsCreate_sRGBProfileTHR(ContextID: Context) -> HPROFILE;
    pub fn cmsCreate_sRGBProfile() -> HPROFILE;
    pub fn cmsCreateBCHSWabstractProfileTHR(ContextID: Context, nLUTPoints: c_int, Bright: f64, Contrast: f64, Hue: f64, Saturation: f64, TempSrc: c_int, TempDest: c_int) -> HPROFILE;
    pub fn cmsCreateBCHSWabstractProfile(nLUTPoints: c_int, Bright: f64, Contrast: f64, Hue: f64, Saturation: f64, TempSrc: c_int, TempDest: c_int) -> HPROFILE;
    pub fn cmsCreateNULLProfileTHR(ContextID: Context) -> HPROFILE;
    pub fn cmsCreateNULLProfile() -> HPROFILE;
    pub fn cmsTransform2DeviceLink(hTransform: HTRANSFORM, Version: f64, dwFlags: u32) -> HPROFILE;
    pub fn cmsGetSupportedIntents(nMax: u32, Codes: *mut u32, Descriptions: *mut *mut c_char) -> u32;
    pub fn cmsGetSupportedIntentsTHR(ContextID: Context, nMax: u32, Codes: *mut u32, Descriptions: *mut *mut c_char) -> u32;
    pub fn cmsCreateTransformTHR(ContextID: Context, Input: HPROFILE, InputFormat: PixelFormat, Output: HPROFILE, OutputFormat: PixelFormat, Intent: Intent, dwFlags: u32) -> HTRANSFORM;
    pub fn cmsCreateTransform(Input: HPROFILE, InputFormat: PixelFormat, Output: HPROFILE, OutputFormat: PixelFormat, Intent: Intent, dwFlags: u32) -> HTRANSFORM;
    pub fn cmsCreateProofingTransformTHR(ContextID: Context,
                                         Input: HPROFILE,
                                         InputFormat: PixelFormat,
                                         Output: HPROFILE,
                                         OutputFormat: PixelFormat,
                                         Proofing: HPROFILE,
                                         Intent: Intent,
                                         ProofingIntent: Intent,
                                         dwFlags: u32)
                                         -> HTRANSFORM;
    pub fn cmsCreateProofingTransform(Input: HPROFILE,
                                      InputFormat: PixelFormat,
                                      Output: HPROFILE,
                                      OutputFormat: PixelFormat,
                                      Proofing: HPROFILE,
                                      Intent: Intent,
                                      ProofingIntent: Intent,
                                      dwFlags: u32)
                                      -> HTRANSFORM;
    pub fn cmsCreateMultiprofileTransformTHR(ContextID: Context,
                                             hProfiles: *mut HPROFILE,
                                             nProfiles: u32,
                                             InputFormat: PixelFormat,
                                             OutputFormat: PixelFormat,
                                             Intent: Intent,
                                             dwFlags: u32)
                                             -> HTRANSFORM;
    pub fn cmsCreateMultiprofileTransform(hProfiles: *mut HPROFILE, nProfiles: u32, InputFormat: PixelFormat, OutputFormat: PixelFormat, Intent: Intent, dwFlags: u32) -> HTRANSFORM;
    pub fn cmsCreateExtendedTransform(ContextID: Context,
                                      nProfiles: u32,
                                      hProfiles: *mut HPROFILE,
                                      BPC: *mut Bool,
                                      Intents: *mut u32,
                                      AdaptationStates: *mut f64,
                                      hGamutProfile: HPROFILE,
                                      nGamutPCSposition: u32,
                                      InputFormat: PixelFormat,
                                      OutputFormat: PixelFormat,
                                      dwFlags: u32)
                                      -> HTRANSFORM;
    pub fn cmsDeleteTransform(hTransform: HTRANSFORM);
    pub fn cmsDoTransform(Transform: HTRANSFORM, InputBuffer: *const c_void, OutputBuffer: *mut c_void, Size: u32);
    pub fn cmsDoTransformStride(Transform: HTRANSFORM, InputBuffer: *const c_void, OutputBuffer: *mut c_void, Size: u32, Stride: u32);
    pub fn cmsDoTransformLineStride(Transform: HTRANSFORM,
                                    InputBuffer: *const c_void,
                                    OutputBuffer: *mut c_void,
                                    PixelsPerLine: u32,
                                    LineCount: u32,
                                    BytesPerLineIn: u32,
                                    BytesPerLineOut: u32,
                                    BytesPerPlaneIn: u32,
                                    BytesPerPlaneOut: u32);
    pub fn cmsSetAlarmCodes(NewAlarm: *const u16);
    pub fn cmsGetAlarmCodes(NewAlarm: *mut u16);
    pub fn cmsSetAlarmCodesTHR(ContextID: Context, AlarmCodes: *const u16);
    pub fn cmsGetAlarmCodesTHR(ContextID: Context, AlarmCodes: *mut u16);
    pub fn cmsSetAdaptationState(d: f64) -> f64;
    pub fn cmsSetAdaptationStateTHR(ContextID: Context, d: f64) -> f64;
    pub fn cmsGetTransformContextID(hTransform: HTRANSFORM) -> Context;
    pub fn cmsGetTransformInputFormat(hTransform: HTRANSFORM) -> PixelFormat;
    pub fn cmsGetTransformOutputFormat(hTransform: HTRANSFORM) -> PixelFormat;
    pub fn cmsChangeBuffersFormat(hTransform: HTRANSFORM, InputFormat: PixelFormat, OutputFormat: PixelFormat) -> Bool;
    pub fn cmsGetPostScriptColorResource(ContextID: Context, Type: PSResourceType, hProfile: HPROFILE, Intent: Intent, dwFlags: u32, io: *mut IOHANDLER) -> u32;
    pub fn cmsGetPostScriptCSA(ContextID: Context, hProfile: HPROFILE, Intent: Intent, dwFlags: u32, Buffer: *mut c_void, dwBufferLen: u32) -> u32;
    pub fn cmsGetPostScriptCRD(ContextID: Context, hProfile: HPROFILE, Intent: Intent, dwFlags: u32, Buffer: *mut c_void, dwBufferLen: u32) -> u32;
    pub fn cmsIT8Alloc(ContextID: Context) -> HANDLE;
    pub fn cmsIT8Free(hIT8: HANDLE);
    pub fn cmsIT8TableCount(hIT8: HANDLE) -> u32;
    pub fn cmsIT8SetTable(hIT8: HANDLE, nTable: u32) -> i32;
    pub fn cmsIT8LoadFromFile(ContextID: Context, cFileName: *const c_char) -> HANDLE;
    pub fn cmsIT8LoadFromMem(ContextID: Context, Ptr: *const c_void, len: u32) -> HANDLE;
    pub fn cmsIT8SaveToFile(hIT8: HANDLE, cFileName: *const c_char) -> Bool;
    pub fn cmsIT8SaveToMem(hIT8: HANDLE, MemPtr: *mut c_void, BytesNeeded: *mut u32) -> Bool;
    pub fn cmsIT8GetSheetType(hIT8: HANDLE) -> *const c_char;
    pub fn cmsIT8SetSheetType(hIT8: HANDLE, Type: *const c_char) -> Bool;
    pub fn cmsIT8SetComment(hIT8: HANDLE, cComment: *const c_char) -> Bool;
    pub fn cmsIT8SetPropertyStr(hIT8: HANDLE, cProp: *const c_char, Str: *const c_char) -> Bool;
    pub fn cmsIT8SetPropertyDbl(hIT8: HANDLE, cProp: *const c_char, Val: f64) -> Bool;
    pub fn cmsIT8SetPropertyHex(hIT8: HANDLE, cProp: *const c_char, Val: u32) -> Bool;
    pub fn cmsIT8SetPropertyMulti(hIT8: HANDLE, Key: *const c_char, SubKey: *const c_char, Buffer: *const c_char) -> Bool;
    pub fn cmsIT8SetPropertyUncooked(hIT8: HANDLE, Key: *const c_char, Buffer: *const c_char) -> Bool;
    pub fn cmsIT8GetProperty(hIT8: HANDLE, cProp: *const c_char) -> *const c_char;
    pub fn cmsIT8GetPropertyDbl(hIT8: HANDLE, cProp: *const c_char) -> f64;
    pub fn cmsIT8GetPropertyMulti(hIT8: HANDLE, Key: *const c_char, SubKey: *const c_char) -> *const c_char;
    pub fn cmsIT8EnumProperties(hIT8: HANDLE, PropertyNames: *mut *mut *mut c_char) -> u32;
    pub fn cmsIT8EnumPropertyMulti(hIT8: HANDLE, cProp: *const c_char, SubpropertyNames: *mut *mut *const c_char) -> u32;
    pub fn cmsIT8GetDataRowCol(hIT8: HANDLE, row: c_int, col: c_int) -> *const c_char;
    pub fn cmsIT8GetDataRowColDbl(hIT8: HANDLE, row: c_int, col: c_int) -> f64;
    pub fn cmsIT8SetDataRowCol(hIT8: HANDLE, row: c_int, col: c_int, Val: *const c_char) -> Bool;
    pub fn cmsIT8SetDataRowColDbl(hIT8: HANDLE, row: c_int, col: c_int, Val: f64) -> Bool;
    pub fn cmsIT8GetData(hIT8: HANDLE, cPatch: *const c_char, cSample: *const c_char) -> *const c_char;
    pub fn cmsIT8GetDataDbl(hIT8: HANDLE, cPatch: *const c_char, cSample: *const c_char) -> f64;
    pub fn cmsIT8SetData(hIT8: HANDLE, cPatch: *const c_char, cSample: *const c_char, Val: *const c_char) -> Bool;
    pub fn cmsIT8SetDataDbl(hIT8: HANDLE, cPatch: *const c_char, cSample: *const c_char, Val: f64) -> Bool;
    pub fn cmsIT8FindDataFormat(hIT8: HANDLE, cSample: *const c_char) -> c_int;
    pub fn cmsIT8SetDataFormat(hIT8: HANDLE, n: c_int, Sample: *const c_char) -> Bool;
    pub fn cmsIT8EnumDataFormat(hIT8: HANDLE, SampleNames: *mut *mut *mut c_char) -> c_int;
    pub fn cmsIT8GetPatchName(hIT8: HANDLE, nPatch: c_int, buffer: *mut c_char) -> *const c_char;
    pub fn cmsIT8GetPatchByName(hIT8: HANDLE, cPatch: *const c_char) -> c_int;
    pub fn cmsIT8SetTableByLabel(hIT8: HANDLE, cSet: *const c_char, cField: *const c_char, ExpectedType: *const c_char) -> c_int;
    pub fn cmsIT8SetIndexColumn(hIT8: HANDLE, cSample: *const c_char) -> Bool;
    pub fn cmsIT8DefineDblFormat(hIT8: HANDLE, Formatter: *const c_char);
    pub fn cmsGBDAlloc(ContextID: Context) -> HANDLE;
    pub fn cmsGBDFree(hGBD: HANDLE);
    pub fn cmsGDBAddPoint(hGBD: HANDLE, Lab: *const CIELab) -> Bool;
    pub fn cmsGDBCompute(hGDB: HANDLE, dwFlags: u32) -> Bool;
    pub fn cmsGDBCheckPoint(hGBD: HANDLE, Lab: *const CIELab) -> Bool;
    pub fn cmsDetectBlackPoint(BlackPoint: *mut CIEXYZ, hProfile: HPROFILE, Intent: Intent, dwFlags: u32) -> Bool;
    pub fn cmsDetectDestinationBlackPoint(BlackPoint: *mut CIEXYZ, hProfile: HPROFILE, Intent: Intent, dwFlags: u32) -> Bool;
    pub fn cmsDetectTAC(hProfile: HPROFILE) -> f64;
    pub fn cmsDesaturateLab(Lab: *mut CIELab, amax: f64, amin: f64, bmax: f64, bmin: f64) -> Bool;
}
