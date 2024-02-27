#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

//!  Little Color Management System
//!  Copyright (c) 1998-2014 Marti Maria Saguer
//!
//! Permission is hereby granted, free of charge, to any person obtaining
//! a copy of this software and associated documentation files (the "Software"),
//! to deal in the Software without restriction, including without limitation
//! the rights to use, copy, modify, merge, publish, distribute, sublicense,
//! and/or sell copies of the Software, and to permit persons to whom the Software
//! is furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in
//! all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
//! EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
//! THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
//! NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
//! LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//! OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
//! WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//!
//!---------------------------------------------------------------------------------
//!
//! Version 2.16

use std::os::raw::{c_char, c_int, c_long, c_void};
#[doc(hidden)]
use libc;
use std::mem::MaybeUninit;
use libc::FILE;
use std::default::Default;

// That one is missing in Rust's libc
#[cfg(not(windows))]
#[doc(hidden)]
pub type tm = libc::tm;
#[cfg(windows)]
#[doc(hidden)]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
}

#[doc(hidden)]
pub type wchar_t = libc::wchar_t;
pub type Signature = u32;
pub type S15Fixed16Number = i32;
pub type Bool = c_int;


/// D50 XYZ normalized to Y=1.0
pub const D50X: f64 = 0.9642;
pub const D50Y: f64 = 1.0;
pub const D50Z: f64 = 0.8249;

/// V4 perceptual black
pub const PERCEPTUAL_BLACK_X: f64 = 0.00336;
pub const PERCEPTUAL_BLACK_Y: f64 = 0.0034731;
pub const PERCEPTUAL_BLACK_Z: f64 = 0.00287;

/// Definitions in ICC spec
/// 'acsp'
pub const MagicNumber: Signature = 0x61637370;
/// 'lcms'
pub const lcmsSignature: Signature = 0x6c636d73;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum TagTypeSignature {
    /// 'chrm'
    ChromaticityType                  = 0x6368726D,
    /// 'clro'
    ColorantOrderType                 = 0x636C726F,
    /// 'clrt'
    ColorantTableType                 = 0x636C7274,
    /// 'crdi'
    CrdInfoType                       = 0x63726469,
    /// 'curv'
    CurveType                         = 0x63757276,
    /// 'data'
    DataType                          = 0x64617461,
    /// 'dict'
    DictType                          = 0x64696374,
    /// 'dtim'
    DateTimeType                      = 0x6474696D,
    /// 'devs'
    DeviceSettingsType                = 0x64657673,
    /// 'mft2'
    Lut16Type                         = 0x6d667432,
    /// 'mft1'
    Lut8Type                          = 0x6d667431,
    /// 'mAB '
    LutAtoBType                       = 0x6d414220,
    /// 'mBA '
    LutBtoAType                       = 0x6d424120,
    /// 'meas'
    MeasurementType                   = 0x6D656173,
    /// 'mluc'
    MultiLocalizedUnicodeType         = 0x6D6C7563,
    /// 'mpet'
    MultiProcessElementType           = 0x6D706574,
    /// 'ncol' -- DEPRECATED!
    NamedColorType                    = 0x6E636F6C,
    /// 'ncl2'
    NamedColor2Type                   = 0x6E636C32,
    /// 'para'
    ParametricCurveType               = 0x70617261,
    /// 'pseq'
    ProfileSequenceDescType           = 0x70736571,
    /// 'psid'
    ProfileSequenceIdType             = 0x70736964,
    /// 'rcs2'
    ResponseCurveSet16Type            = 0x72637332,
    /// 'sf32'
    S15Fixed16ArrayType               = 0x73663332,
    /// 'scrn'
    ScreeningType                     = 0x7363726E,
    /// 'sig '
    SignatureType                     = 0x73696720,
    /// 'text'
    TextType                          = 0x74657874,
    /// 'desc'
    TextDescriptionType               = 0x64657363,
    /// 'uf32'
    U16Fixed16ArrayType               = 0x75663332,
    /// 'bfd '
    UcrBgType                         = 0x62666420,
    /// 'ui16'
    UInt16ArrayType                   = 0x75693136,
    /// 'ui32'
    UInt32ArrayType                   = 0x75693332,
    /// 'ui64'
    UInt64ArrayType                   = 0x75693634,
    /// 'ui08'
    UInt8ArrayType                    = 0x75693038,
    /// 'vcgt'
    VcgtType                          = 0x76636774,
    /// 'view'
    ViewingConditionsType             = 0x76696577,
    /// 'XYZ '
    XYZType                           = 0x58595A20,
    /// `cicp`
    CicpType                          = 0x63696370,
    MHC2Type                          = 1296581426,
}

pub const BlueMatrixColumnTag: TagSignature = TagSignature::BlueColorantTag;
pub const GreenMatrixColumnTag: TagSignature = TagSignature::GreenColorantTag;
pub const RedMatrixColumnTag: TagSignature = TagSignature::RedColorantTag;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum TagSignature {
    /// 'A2B0'
    AToB0Tag                          = 0x41324230,
    /// 'A2B1'
    AToB1Tag                          = 0x41324231,
    /// 'A2B2'
    AToB2Tag                          = 0x41324232,
    /// 'bXYZ'
    BlueColorantTag                   = 0x6258595A,
    /// 'bTRC'
    BlueTRCTag                        = 0x62545243,
    /// 'B2A0'
    BToA0Tag                          = 0x42324130,
    /// 'B2A1'
    BToA1Tag                          = 0x42324131,
    /// 'B2A2'
    BToA2Tag                          = 0x42324132,
    /// 'calt'
    CalibrationDateTimeTag            = 0x63616C74,
    /// 'targ'
    CharTargetTag                     = 0x74617267,
    /// 'chad'
    ChromaticAdaptationTag            = 0x63686164,
    /// 'chrm'
    ChromaticityTag                   = 0x6368726D,
    /// 'clro'
    ColorantOrderTag                  = 0x636C726F,
    /// 'clrt'
    ColorantTableTag                  = 0x636C7274,
    /// 'clot'
    ColorantTableOutTag               = 0x636C6F74,
    /// 'ciis'
    ColorimetricIntentImageStateTag   = 0x63696973,
    /// 'cprt'
    CopyrightTag                      = 0x63707274,
    /// 'crdi'
    CrdInfoTag                        = 0x63726469,
    /// 'data'
    DataTag                           = 0x64617461,
    /// 'dtim'
    DateTimeTag                       = 0x6474696D,
    /// 'dmnd'
    DeviceMfgDescTag                  = 0x646D6E64,
    /// 'dmdd'
    DeviceModelDescTag                = 0x646D6464,
    /// 'devs'
    DeviceSettingsTag                 = 0x64657673,
    /// 'D2B0'
    DToB0Tag                          = 0x44324230,
    /// 'D2B1'
    DToB1Tag                          = 0x44324231,
    /// 'D2B2'
    DToB2Tag                          = 0x44324232,
    /// 'D2B3'
    DToB3Tag                          = 0x44324233,
    /// 'B2D0'
    BToD0Tag                          = 0x42324430,
    /// 'B2D1'
    BToD1Tag                          = 0x42324431,
    /// 'B2D2'
    BToD2Tag                          = 0x42324432,
    /// 'B2D3'
    BToD3Tag                          = 0x42324433,
    /// 'gamt'
    GamutTag                          = 0x67616D74,
    /// 'kTRC'
    GrayTRCTag                        = 0x6b545243,
    /// 'gXYZ'
    GreenColorantTag                  = 0x6758595A,
    /// 'gTRC'
    GreenTRCTag                       = 0x67545243,
    /// 'lumi'
    LuminanceTag                      = 0x6C756D69,
    /// 'meas'
    MeasurementTag                    = 0x6D656173,
    /// 'bkpt'
    MediaBlackPointTag                = 0x626B7074,
    /// 'wtpt'
    MediaWhitePointTag                = 0x77747074,
    /// 'ncol' // Deprecated by the ICC
    NamedColorTag                     = 0x6E636F6C,
    /// 'ncl2'
    NamedColor2Tag                    = 0x6E636C32,
    /// 'resp'
    OutputResponseTag                 = 0x72657370,
    /// 'rig0'
    PerceptualRenderingIntentGamutTag = 0x72696730,
    /// 'pre0'
    Preview0Tag                       = 0x70726530,
    /// 'pre1'
    Preview1Tag                       = 0x70726531,
    /// 'pre2'
    Preview2Tag                       = 0x70726532,
    /// 'desc'
    ProfileDescriptionTag             = 0x64657363,
    /// 'dscm'
    ProfileDescriptionMLTag           = 0x6473636D,
    /// 'pseq'
    ProfileSequenceDescTag            = 0x70736571,
    /// 'psid'
    ProfileSequenceIdTag              = 0x70736964,
    /// 'psd0'
    Ps2CRD0Tag                        = 0x70736430,
    /// 'psd1'
    Ps2CRD1Tag                        = 0x70736431,
    /// 'psd2'
    Ps2CRD2Tag                        = 0x70736432,
    /// 'psd3'
    Ps2CRD3Tag                        = 0x70736433,
    /// 'ps2s'
    Ps2CSATag                         = 0x70733273,
    /// 'ps2i'
    Ps2RenderingIntentTag             = 0x70733269,
    /// 'rXYZ'
    RedColorantTag                    = 0x7258595A,
    /// 'rTRC'
    RedTRCTag                         = 0x72545243,
    /// 'rig2'
    SaturationRenderingIntentGamutTag = 0x72696732,
    /// 'scrd'
    ScreeningDescTag                  = 0x73637264,
    /// 'scrn'
    ScreeningTag                      = 0x7363726E,
    /// 'tech'
    TechnologyTag                     = 0x74656368,
    /// 'bfd '
    UcrBgTag                          = 0x62666420,
    /// 'vued'
    ViewingCondDescTag                = 0x76756564,
    /// 'view'
    ViewingConditionsTag              = 0x76696577,
    /// 'vcgt'
    VcgtTag                           = 0x76636774,
    /// 'meta'
    MetaTag                           = 0x6D657461,
    /// 'arts'
    ArgyllArtsTag                     = 0x61727473,
    /// `cicp`
    CicpTag                           = 0x63696370,
    MHC2Tag                           = 1296581426,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum TechnologySignature {
    /// 'dcam'
    DigitalCamera                     = 0x6463616D,
    /// 'fscn'
    FilmScanner                       = 0x6673636E,
    /// 'rscn'
    ReflectiveScanner                 = 0x7273636E,
    /// 'ijet'
    InkJetPrinter                     = 0x696A6574,
    /// 'twax'
    ThermalWaxPrinter                 = 0x74776178,
    /// 'epho'
    ElectrophotographicPrinter        = 0x6570686F,
    /// 'esta'
    ElectrostaticPrinter              = 0x65737461,
    /// 'dsub'
    DyeSublimationPrinter             = 0x64737562,
    /// 'rpho'
    PhotographicPaperPrinter          = 0x7270686F,
    /// 'fprn'
    FilmWriter                        = 0x6670726E,
    /// 'vidm'
    VideoMonitor                      = 0x7669646D,
    /// 'vidc'
    VideoCamera                       = 0x76696463,
    /// 'pjtv'
    ProjectionTelevision              = 0x706A7476,
    /// 'CRT '
    CRTDisplay                        = 0x43525420,
    /// 'PMD '
    PMDisplay                         = 0x504D4420,
    /// 'AMD '
    AMDisplay                         = 0x414D4420,
    /// 'KPCD'
    PhotoCD                           = 0x4B504344,
    /// 'imgs'
    PhotoImageSetter                  = 0x696D6773,
    /// 'grav'
    Gravure                           = 0x67726176,
    /// 'offs'
    OffsetLithography                 = 0x6F666673,
    /// 'silk'
    Silkscreen                        = 0x73696C6B,
    /// 'flex'
    Flexography                       = 0x666C6578,
    /// 'mpfs'
    MotionPictureFilmScanner          = 0x6D706673,
    /// 'mpfr'
    MotionPictureFilmRecorder         = 0x6D706672,
    /// 'dmpc'
    DigitalMotionPictureCamera        = 0x646D7063,
    /// 'dcpj'
    DigitalCinemaProjector            = 0x64636A70
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum ColorSpaceSignature {
    /// 'XYZ '
    XYZData                           = 0x58595A20,
    /// 'Lab '
    LabData                           = 0x4C616220,
    /// 'Luv '
    LuvData                           = 0x4C757620,
    /// 'YCbr'
    YCbCrData                         = 0x59436272,
    /// 'Yxy '
    YxyData                           = 0x59787920,
    /// 'RGB '
    RgbData                           = 0x52474220,
    /// 'GRAY'
    GrayData                          = 0x47524159,
    /// 'HSV '
    HsvData                           = 0x48535620,
    /// 'HLS '
    HlsData                           = 0x484C5320,
    /// 'CMYK'
    CmykData                          = 0x434D594B,
    /// 'CMY '
    CmyData                           = 0x434D5920,
    /// 'MCH1'
    MCH1Data                          = 0x4D434831,
    /// 'MCH2'
    MCH2Data                          = 0x4D434832,
    /// 'MCH3'
    MCH3Data                          = 0x4D434833,
    /// 'MCH4'
    MCH4Data                          = 0x4D434834,
    /// 'MCH5'
    MCH5Data                          = 0x4D434835,
    /// 'MCH6'
    MCH6Data                          = 0x4D434836,
    /// 'MCH7'
    MCH7Data                          = 0x4D434837,
    /// 'MCH8'
    MCH8Data                          = 0x4D434838,
    /// 'MCH9'
    MCH9Data                          = 0x4D434839,
    /// 'MCHA'
    MCHAData                          = 0x4D434841,
    /// 'MCHB'
    MCHBData                          = 0x4D434842,
    /// 'MCHC'
    MCHCData                          = 0x4D434843,
    /// 'MCHD'
    MCHDData                          = 0x4D434844,
    /// 'MCHE'
    MCHEData                          = 0x4D434845,
    /// 'MCHF'
    MCHFData                          = 0x4D434846,
    /// 'nmcl'
    NamedData                         = 0x6e6d636c,
    /// '1CLR'
    Sig1colorData                     = 0x31434C52,
    /// '2CLR'
    Sig2colorData                     = 0x32434C52,
    /// '3CLR'
    Sig3colorData                     = 0x33434C52,
    /// '4CLR'
    Sig4colorData                     = 0x34434C52,
    /// '5CLR'
    Sig5colorData                     = 0x35434C52,
    /// '6CLR'
    Sig6colorData                     = 0x36434C52,
    /// '7CLR'
    Sig7colorData                     = 0x37434C52,
    /// '8CLR'
    Sig8colorData                     = 0x38434C52,
    /// '9CLR'
    Sig9colorData                     = 0x39434C52,
    /// 'ACLR'
    Sig10colorData                    = 0x41434C52,
    /// 'BCLR'
    Sig11colorData                    = 0x42434C52,
    /// 'CCLR'
    Sig12colorData                    = 0x43434C52,
    /// 'DCLR'
    Sig13colorData                    = 0x44434C52,
    /// 'ECLR'
    Sig14colorData                    = 0x45434C52,
    /// 'FCLR'
    Sig15colorData                    = 0x46434C52,
    /// 'LuvK'
    LuvKData                          = 0x4C75764B
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum ProfileClassSignature {
    /// 'scnr'
    InputClass                        = 0x73636E72,
    /// 'mntr'
    DisplayClass                      = 0x6D6E7472,
    /// 'prtr'
    OutputClass                       = 0x70727472,
    /// 'link'
    LinkClass                         = 0x6C696E6B,
    /// 'abst'
    AbstractClass                     = 0x61627374,
    /// 'spac'
    ColorSpaceClass                   = 0x73706163,
    /// 'nmcl'
    NamedColorClass                   = 0x6e6d636c
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum PlatformSignature {
    /// 'APPL'
    Macintosh                         = 0x4150504C,
    /// 'MSFT'
    Microsoft                         = 0x4D534654,
    /// 'SUNW'
    Solaris                           = 0x53554E57,
    /// 'SGI '
    SGI                               = 0x53474920,
    /// 'TGNT'
    Taligent                          = 0x54474E54,
    /// '*nix'   // From argyll -- Not official
    Unices                            = 0x2A6E6978
}

///'prmg'
pub const PerceptualReferenceMediumGamut:u32 =         0x70726d67;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum ColorimetricIntentImageState {
    ///'scoe'
    SceneColorimetryEstimates =              0x73636F65,
    ///'sape'
    SceneAppearanceEstimates =               0x73617065,
    ///'fpce'
    FocalPlaneColorimetryEstimates =         0x66706365,
    ///'rhoc'
    ReflectionHardcopyOriginalColorimetry =  0x72686F63,
    ///'rpoc'
    ReflectionPrintOutputColorimetry =       0x72706F63,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum StageSignature {
    ///'cvst'
    CurveSetElemType              = 0x63767374,
    ///'matf'
    MatrixElemType                = 0x6D617466,
    ///'clut'
    CLutElemType                  = 0x636C7574,

    /// 'bACS'
    BAcsElemType                  = 0x62414353,
    /// 'eACS'
    EAcsElemType                  = 0x65414353,

    /// Custom from here, not in the ICC Spec
    /// 'l2x '
    XYZ2LabElemType               = 0x6C327820,
    /// 'x2l '
    Lab2XYZElemType               = 0x78326C20,
    /// 'ncl '
    NamedColorElemType            = 0x6E636C20,
    /// '2 4 '
    LabV2toV4                     = 0x32203420,
    /// '4 2 '
    LabV4toV2                     = 0x34203220,

    /// Identities
    /// 'idn '
    IdentityElemType              = 0x69646E20,

    /// Float to floatPCS
    /// 'd2l '
    Lab2FloatPCS                  = 0x64326C20,
    /// 'l2d '
    FloatPCS2Lab                  = 0x6C326420,
    /// 'd2x '
    XYZ2FloatPCS                  = 0x64327820,
    /// 'x2d '
    FloatPCS2XYZ                  = 0x78326420,
    /// 'clp '
    ClipNegativesElemType         = 0x636c7020
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum CurveSegSignature {
    /// 'parf'
    FormulaCurveSeg               = 0x70617266,
    /// 'samf'
    SampledCurveSeg               = 0x73616D66,
    /// 'curf'
    SegmentedCurve                = 0x63757266
}

///'StaA'
pub const StatusA:u32 =                    0x53746141;
///'StaE'
pub const StatusE:u32 =                    0x53746145;
///'StaI'
pub const StatusI:u32 =                    0x53746149;
///'StaT'
pub const StatusT:u32 =                    0x53746154;
///'StaM'
pub const StatusM:u32 =                    0x5374614D;
///'DN  '
pub const DN:u32 =                         0x444E2020;
///'DN P'
pub const DNP:u32 =                        0x444E2050;
///'DNN '
pub const DNN:u32 =                        0x444E4E20;
///'DNNP'
pub const DNNP:u32 =                       0x444E4E50;

/// Device attributes, currently defined values correspond to the low 4 bytes
/// of the 8 byte attribute quantity
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
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
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
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Debug)]
pub struct EncodedXYZNumber {
    pub X: S15Fixed16Number,
    pub Y: S15Fixed16Number,
    pub Z: S15Fixed16Number,
}

impl Default for EncodedXYZNumber {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Debug)]
/// Profile ID as computed by MD5 algorithm
pub struct ProfileID {
    pub ID32: [u32; 4],
}

impl Default for ProfileID {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ICCHeader {
    /// Profile size in bytes
    pub size: u32,
    /// CMM for this profile
    pub cmmId: Signature,
    /// Format version number
    pub version: u32,
    /// Type of profile
    pub deviceClass: ProfileClassSignature,
    /// Color space of data
    pub colorSpace: ColorSpaceSignature,
    /// PCS, XYZ or Lab only
    pub pcs: ColorSpaceSignature,
    /// Date profile was created
    pub date: DateTimeNumber,
    /// Magic Number to identify an ICC profile
    pub magic: Signature,
    /// Primary Platform
    pub platform: PlatformSignature,
    /// Various bit settings
    pub flags: u32,
    /// Device manufacturer
    pub manufacturer: Signature,
    /// Device model number
    pub model: u32,
    /// Device attributes
    pub attributes: u64,
    /// Rendering intent
    pub renderingIntent: Intent,
    /// Profile illuminant
    pub illuminant: EncodedXYZNumber,
    /// Profile creator
    pub creator: Signature,
    /// Profile ID using MD5
    pub profileID: ProfileID,
    /// Reserved for future use
    pub reserved: [i8; 28],
}
impl Default for ICCHeader {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct TagBase {
    pub sig: TagTypeSignature,
    pub reserved: [i8; 4],
}
impl Default for TagBase {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct TagEntry {
    pub sig: TagSignature,
    pub offset: u32,
    pub size: u32,
}
impl Default for TagEntry {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

pub type HANDLE = *mut c_void;
#[repr(C)]
pub struct _HPROFILE { _opaque_type: [u8; 0] }
pub type HPROFILE = *mut _HPROFILE;
#[repr(C)]
pub struct _HTRANSFORM { _opaque_type: [u8; 0] }
pub type HTRANSFORM = *mut _HTRANSFORM;

/// Maximum number of channels in ICC profiles
pub const MAXCHANNELS: usize =  16;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PixelType(pub u32);

// Pixel types
/// Don't check colorspace
pub const PT_ANY: PixelType =   PixelType(0);
/// 1 & 2 are reserved
pub const PT_GRAY: PixelType =  PixelType(3);
pub const PT_RGB: PixelType =   PixelType(4);
pub const PT_CMY: PixelType =   PixelType(5);
pub const PT_CMYK: PixelType =  PixelType(6);
pub const PT_YCbCr: PixelType = PixelType(7);
/// Lu'v'
pub const PT_YUV: PixelType =   PixelType(8);
pub const PT_XYZ: PixelType =   PixelType(9);
pub const PT_Lab: PixelType =   PixelType(10);
/// Lu'v'K
pub const PT_YUVK: PixelType =  PixelType(11);
pub const PT_HSV: PixelType =   PixelType(12);
pub const PT_HLS: PixelType =   PixelType(13);
pub const PT_Yxy: PixelType =   PixelType(14);

pub const PT_MCH1: PixelType =  PixelType(15);
pub const PT_MCH2: PixelType =  PixelType(16);
pub const PT_MCH3: PixelType =  PixelType(17);
pub const PT_MCH4: PixelType =  PixelType(18);
pub const PT_MCH5: PixelType =  PixelType(19);
pub const PT_MCH6: PixelType =  PixelType(20);
pub const PT_MCH7: PixelType =  PixelType(21);
pub const PT_MCH8: PixelType =  PixelType(22);
pub const PT_MCH9: PixelType =  PixelType(23);
pub const PT_MCH10: PixelType = PixelType(24);
pub const PT_MCH11: PixelType = PixelType(25);
pub const PT_MCH12: PixelType = PixelType(26);
pub const PT_MCH13: PixelType = PixelType(27);
pub const PT_MCH14: PixelType = PixelType(28);
pub const PT_MCH15: PixelType = PixelType(29);

/// Identical to PT_Lab, but using the V2 old encoding
pub const PT_LabV2: PixelType = PixelType(30);

/// Format of pixel is defined by one u32, using bit fields as follows
///
///                               2                1          0
///                        4 3 2 10987 6 5 4 3 2 1 098 7654 321
///                        M A O TTTTT U Y F P X S EEE CCCC BBB
///
///            M: Premultiplied alpha (only works when extra samples is 1)
///            A: Floating point -- With this flag we can differentiate 16 bits as float and as int
///            O: Optimized -- previous optimization already returns the final 8-bit value
///            T: Pixeltype
///            F: Flavor  0=MinIsBlack(Chocolate) 1=MinIsWhite(Vanilla)
///            P: Planar? 0=Chunky, 1=Planar
///            X: swap 16 bps endianness?
///            S: Do swap? ie, BGR, KYMC
///            E: Extra samples
///            C: Channels (Samples per pixel)
///            B: bytes per sample
///            Y: Swap first - changes ABGR to BGRA and KCMY to CMYK
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PixelFormat(pub u32);

impl PixelFormat {
    pub const GRAY_8: PixelFormat = PixelFormat(196617);
    pub const GRAY_8_REV: PixelFormat = PixelFormat(204809);
    pub const GRAY_16: PixelFormat = PixelFormat(196618);
    pub const GRAY_16_REV: PixelFormat = PixelFormat(204810);
    pub const GRAY_16_SE: PixelFormat = PixelFormat(198666);
    pub const GRAYA_8: PixelFormat = PixelFormat(196745);
    pub const GRAYA_16: PixelFormat = PixelFormat(196746);
    pub const GRAYA_16_SE: PixelFormat = PixelFormat(198794);
    pub const GRAYA_8_PLANAR: PixelFormat = PixelFormat(200841);
    pub const GRAYA_16_PLANAR: PixelFormat = PixelFormat(200842);
    pub const RGB_8: PixelFormat = PixelFormat(262169);
    pub const RGB_8_PLANAR: PixelFormat = PixelFormat(266265);
    pub const BGR_8: PixelFormat = PixelFormat(263193);
    pub const BGR_8_PLANAR: PixelFormat = PixelFormat(267289);
    pub const RGB_16: PixelFormat = PixelFormat(262170);
    pub const RGB_16_PLANAR: PixelFormat = PixelFormat(266266);
    pub const RGB_16_SE: PixelFormat = PixelFormat(264218);
    pub const BGR_16: PixelFormat = PixelFormat(263194);
    pub const BGR_16_PLANAR: PixelFormat = PixelFormat(267290);
    pub const BGR_16_SE: PixelFormat = PixelFormat(265242);
    pub const RGBA_8: PixelFormat = PixelFormat(262297);
    pub const RGBA_8_PLANAR: PixelFormat = PixelFormat(266393);
    pub const RGBA_16: PixelFormat = PixelFormat(262298);
    pub const RGBA_16_PLANAR: PixelFormat = PixelFormat(266394);
    pub const RGBA_16_SE: PixelFormat = PixelFormat(264346);
    pub const ARGB_8: PixelFormat = PixelFormat(278681);
    pub const ARGB_8_PLANAR: PixelFormat = PixelFormat(282777);
    pub const ARGB_16: PixelFormat = PixelFormat(278682);
    pub const ABGR_8: PixelFormat = PixelFormat(263321);
    pub const ABGR_8_PLANAR: PixelFormat = PixelFormat(267417);
    pub const ABGR_16: PixelFormat = PixelFormat(263322);
    pub const ABGR_16_PLANAR: PixelFormat = PixelFormat(267418);
    pub const ABGR_16_SE: PixelFormat = PixelFormat(265370);
    pub const BGRA_8: PixelFormat = PixelFormat(279705);
    pub const BGRA_8_PLANAR: PixelFormat = PixelFormat(283801);
    pub const BGRA_16: PixelFormat = PixelFormat(279706);
    pub const BGRA_16_SE: PixelFormat = PixelFormat(281754);
    pub const CMY_8: PixelFormat = PixelFormat(327705);
    pub const CMY_8_PLANAR: PixelFormat = PixelFormat(331801);
    pub const CMY_16: PixelFormat = PixelFormat(327706);
    pub const CMY_16_PLANAR: PixelFormat = PixelFormat(331802);
    pub const CMY_16_SE: PixelFormat = PixelFormat(329754);
    pub const CMYK_8: PixelFormat = PixelFormat(393249);
    pub const CMYKA_8: PixelFormat = PixelFormat(393377);
    pub const CMYK_8_REV: PixelFormat = PixelFormat(401441);
    pub const CMYK_8_PLANAR: PixelFormat = PixelFormat(397345);
    pub const CMYK_16: PixelFormat = PixelFormat(393250);
    pub const CMYK_16_REV: PixelFormat = PixelFormat(401442);
    pub const CMYK_16_PLANAR: PixelFormat = PixelFormat(397346);
    pub const CMYK_16_SE: PixelFormat = PixelFormat(395298);
    pub const KYMC_8: PixelFormat = PixelFormat(394273);
    pub const KYMC_16: PixelFormat = PixelFormat(394274);
    pub const KYMC_16_SE: PixelFormat = PixelFormat(396322);
    pub const KCMY_8: PixelFormat = PixelFormat(409633);
    pub const KCMY_8_REV: PixelFormat = PixelFormat(417825);
    pub const KCMY_16: PixelFormat = PixelFormat(409634);
    pub const KCMY_16_REV: PixelFormat = PixelFormat(417826);
    pub const KCMY_16_SE: PixelFormat = PixelFormat(411682);
    pub const CMYK5_8: PixelFormat = PixelFormat(1245225);
    pub const CMYK5_16: PixelFormat = PixelFormat(1245226);
    pub const CMYK5_16_SE: PixelFormat = PixelFormat(1247274);
    pub const KYMC5_8: PixelFormat = PixelFormat(1246249);
    pub const KYMC5_16: PixelFormat = PixelFormat(1246250);
    pub const KYMC5_16_SE: PixelFormat = PixelFormat(1248298);
    pub const CMYK6_8: PixelFormat = PixelFormat(1310769);
    pub const CMYK6_8_PLANAR: PixelFormat = PixelFormat(1314865);
    pub const CMYK6_16: PixelFormat = PixelFormat(1310770);
    pub const CMYK6_16_PLANAR: PixelFormat = PixelFormat(1314866);
    pub const CMYK6_16_SE: PixelFormat = PixelFormat(1312818);
    pub const CMYK7_8: PixelFormat = PixelFormat(1376313);
    pub const CMYK7_16: PixelFormat = PixelFormat(1376314);
    pub const CMYK7_16_SE: PixelFormat = PixelFormat(1378362);
    pub const KYMC7_8: PixelFormat = PixelFormat(1377337);
    pub const KYMC7_16: PixelFormat = PixelFormat(1377338);
    pub const KYMC7_16_SE: PixelFormat = PixelFormat(1379386);
    pub const CMYK8_8: PixelFormat = PixelFormat(1441857);
    pub const CMYK8_16: PixelFormat = PixelFormat(1441858);
    pub const CMYK8_16_SE: PixelFormat = PixelFormat(1443906);
    pub const KYMC8_8: PixelFormat = PixelFormat(1442881);
    pub const KYMC8_16: PixelFormat = PixelFormat(1442882);
    pub const KYMC8_16_SE: PixelFormat = PixelFormat(1444930);
    pub const CMYK9_8: PixelFormat = PixelFormat(1507401);
    pub const CMYK9_16: PixelFormat = PixelFormat(1507402);
    pub const CMYK9_16_SE: PixelFormat = PixelFormat(1509450);
    pub const KYMC9_8: PixelFormat = PixelFormat(1508425);
    pub const KYMC9_16: PixelFormat = PixelFormat(1508426);
    pub const KYMC9_16_SE: PixelFormat = PixelFormat(1510474);
    pub const CMYK10_8: PixelFormat = PixelFormat(1572945);
    pub const CMYK10_16: PixelFormat = PixelFormat(1572946);
    pub const CMYK10_16_SE: PixelFormat = PixelFormat(1574994);
    pub const KYMC10_8: PixelFormat = PixelFormat(1573969);
    pub const KYMC10_16: PixelFormat = PixelFormat(1573970);
    pub const KYMC10_16_SE: PixelFormat = PixelFormat(1576018);
    pub const CMYK11_8: PixelFormat = PixelFormat(1638489);
    pub const CMYK11_16: PixelFormat = PixelFormat(1638490);
    pub const CMYK11_16_SE: PixelFormat = PixelFormat(1640538);
    pub const KYMC11_8: PixelFormat = PixelFormat(1639513);
    pub const KYMC11_16: PixelFormat = PixelFormat(1639514);
    pub const KYMC11_16_SE: PixelFormat = PixelFormat(1641562);
    pub const CMYK12_8: PixelFormat = PixelFormat(1704033);
    pub const CMYK12_16: PixelFormat = PixelFormat(1704034);
    pub const CMYK12_16_SE: PixelFormat = PixelFormat(1706082);
    pub const KYMC12_8: PixelFormat = PixelFormat(1705057);
    pub const KYMC12_16: PixelFormat = PixelFormat(1705058);
    pub const KYMC12_16_SE: PixelFormat = PixelFormat(1707106);
    pub const XYZ_16: PixelFormat = PixelFormat(589850);
    pub const Lab_8: PixelFormat = PixelFormat(655385);
    pub const LabV2_8: PixelFormat = PixelFormat(1966105);
    pub const ALab_8: PixelFormat = PixelFormat(671897);
    pub const ALabV2_8: PixelFormat = PixelFormat(1982617);
    pub const Lab_16: PixelFormat = PixelFormat(655386);
    pub const LabV2_16: PixelFormat = PixelFormat(1966106);
    pub const Yxy_16: PixelFormat = PixelFormat(917530);
    pub const YCbCr_8: PixelFormat = PixelFormat(458777);
    pub const YCbCr_8_PLANAR: PixelFormat = PixelFormat(462873);
    pub const YCbCr_16: PixelFormat = PixelFormat(458778);
    pub const YCbCr_16_PLANAR: PixelFormat = PixelFormat(462874);
    pub const YCbCr_16_SE: PixelFormat = PixelFormat(460826);
    pub const YUV_8: PixelFormat = PixelFormat(524313);
    pub const YUV_8_PLANAR: PixelFormat = PixelFormat(528409);
    pub const YUV_16: PixelFormat = PixelFormat(524314);
    pub const YUV_16_PLANAR: PixelFormat = PixelFormat(528410);
    pub const YUV_16_SE: PixelFormat = PixelFormat(526362);
    pub const HLS_8: PixelFormat = PixelFormat(851993);
    pub const HLS_8_PLANAR: PixelFormat = PixelFormat(856089);
    pub const HLS_16: PixelFormat = PixelFormat(851994);
    pub const HLS_16_PLANAR: PixelFormat = PixelFormat(856090);
    pub const HLS_16_SE: PixelFormat = PixelFormat(854042);
    pub const HSV_8: PixelFormat = PixelFormat(786457);
    pub const HSV_8_PLANAR: PixelFormat = PixelFormat(790553);
    pub const HSV_16: PixelFormat = PixelFormat(786458);
    pub const HSV_16_PLANAR: PixelFormat = PixelFormat(790554);
    pub const HSV_16_SE: PixelFormat = PixelFormat(788506);
    // Named color index. Only 16 bits allowed (don't check colorspace)
    pub const NAMED_COLOR_INDEX: PixelFormat = PixelFormat(10);
    pub const XYZ_FLT: PixelFormat = PixelFormat(4784156);
    pub const Lab_FLT: PixelFormat = PixelFormat(4849692);
    pub const LabA_FLT: PixelFormat = PixelFormat(4849820);
    pub const GRAY_FLT: PixelFormat = PixelFormat(4390924);
    pub const RGB_FLT: PixelFormat = PixelFormat(4456476);
    pub const RGBA_FLT: PixelFormat = PixelFormat(4456604);
    pub const ARGB_FLT: PixelFormat = PixelFormat(4472988);
    pub const BGR_FLT: PixelFormat = PixelFormat(4457500);
    pub const BGRA_FLT: PixelFormat = PixelFormat(4474012);
    pub const CMYK_FLT: PixelFormat = PixelFormat(4587556);
    pub const XYZ_DBL: PixelFormat = PixelFormat(4784152);
    pub const Lab_DBL: PixelFormat = PixelFormat(4849688);
    pub const GRAY_DBL: PixelFormat = PixelFormat(4390920);
    pub const RGB_DBL: PixelFormat = PixelFormat(4456472);
    pub const BGR_DBL: PixelFormat = PixelFormat(4457496);
    pub const CMYK_DBL: PixelFormat = PixelFormat(4587552);
    pub const GRAY_HALF_FLT: PixelFormat = PixelFormat(4390922);
    pub const RGB_HALF_FLT: PixelFormat = PixelFormat(4456474);
    pub const RGBA_HALF_FLT: PixelFormat = PixelFormat(4456602);
    pub const CMYK_HALF_FLT: PixelFormat = PixelFormat(4587554);
    pub const ARGB_HALF_FLT: PixelFormat = PixelFormat(4472986);
    pub const BGR_HALF_FLT: PixelFormat = PixelFormat(4457498);
    pub const BGRA_HALF_FLT: PixelFormat = PixelFormat(4474010);

    ///   M: Premultiplied alpha (only works when extra samples is 1)
    #[must_use]
    #[inline]
    pub fn premultiplied(&self) -> bool {
        ((self.0 >> 23) & 1) != 0
    }

    ///   A: Floating point -- With this flag we can differentiate 16 bits as float and as int
    #[must_use]
    #[inline]
    pub fn float(&self) -> bool {
        ((self.0 >> 22) & 1) != 0
    }

    ///   O: Optimized -- previous optimization already returns the final 8-bit value
    #[must_use]
    #[inline]
    pub fn optimized(&self) -> bool {
        ((self.0 >> 21) & 1) != 0
    }

    ///   T: Color space (`PT_*`)
    #[must_use]
    #[inline]
    pub fn pixel_type(&self) -> PixelType {
        PixelType((self.0 >> 16) & 31)
    }

    ///   Y: Swap first - changes ABGR to BGRA and KCMY to CMYK
    #[must_use]
    #[inline]
    pub fn swapfirst(&self) -> bool {
        ((self.0 >> 14) & 1) != 0
    }

    ///   F: Flavor  0=MinIsBlack(Chocolate) 1=MinIsWhite(Vanilla)
    #[must_use]
    #[inline]
    pub fn min_is_white(&self) -> bool {
        ((self.0 >> 13) & 1) != 0
    }

    ///   P: Planar? 0=Chunky, 1=Planar
    #[must_use]
    #[inline]
    pub fn planar(&self) -> bool {
        ((self.0 >> 12) & 1) != 0
    }

    ///   X: swap 16 bps endianness?
    #[must_use]
    #[inline]
    pub fn endian16(&self) -> bool {
        ((self.0 >> 11) & 1) != 0
    }

    ///   S: Do swap? ie, BGR, KYMC
    #[must_use]
    #[inline]
    pub fn doswap(&self) -> bool {
        ((self.0 >> 10) & 1) != 0
    }

    ///   E: Extra samples
    #[must_use]
    #[inline]
    pub fn extra(&self) -> usize {
        ((self.0 >> 7) & 7) as usize
    }

    ///   C: Channels (Samples per pixel)
    #[must_use]
    #[inline]
    pub fn channels(&self) -> usize {
        ((self.0 >> 3) & 15) as usize
    }

    ///   B: bytes per sample
    #[must_use]
    pub fn bytes_per_channel(&self) -> usize {
        let res = (self.0 & 7) as usize;
        // 8 overflows the field
        if res != 0 {res} else {8}
    }

    /// size of pixel
    #[must_use]
    pub fn bytes_per_pixel(&self) -> usize {
        self.bytes_per_channel() * (self.extra() + self.channels())
    }
}

#[test]
fn test_bpc() {
    assert_eq!(8, PixelFormat::XYZ_DBL.bytes_per_channel());
    assert_eq!(8, PixelFormat::Lab_DBL.bytes_per_channel());
    assert_eq!(8, PixelFormat::GRAY_DBL.bytes_per_channel());
    assert_eq!(8, PixelFormat::RGB_DBL.bytes_per_channel());
    assert_eq!(8, PixelFormat::BGR_DBL.bytes_per_channel());
    assert_eq!(8, PixelFormat::CMYK_DBL.bytes_per_channel());
}

#[test]
fn test_pixelformat() {
    assert_eq!(4, PixelFormat::CMYKA_8.channels());
    assert_eq!(1, PixelFormat::CMYKA_8.extra());

    assert!(!PixelFormat::CMYKA_8.doswap());
    assert_eq!(1, PixelFormat::CMYKA_8.bytes_per_channel());
    assert_eq!(5, PixelFormat::CMYKA_8.bytes_per_pixel());

    assert_eq!(2, PixelFormat::CMYK_HALF_FLT.bytes_per_channel());
    assert_eq!(PT_CMYK, PixelFormat::CMYK_HALF_FLT.pixel_type());
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct CIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
impl Default for CIEXYZ {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct CIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}
impl Default for CIExyY {
    #[inline]
    fn default() -> Self { CIExyY{x:0., y:0., Y:1.} }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct CIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}
impl Default for CIELab {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct CIELCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}
impl Default for CIELCh {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct JCh {
    pub J: f64,
    pub C: f64,
    pub h: f64,
}
impl Default for JCh {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct CIEXYZTRIPLE {
    pub Red: CIEXYZ,
    pub Green: CIEXYZ,
    pub Blue: CIEXYZ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub struct CIExyYTRIPLE {
    pub Red: CIExyY,
    pub Green: CIExyY,
    pub Blue: CIExyY,
}

/// Illuminant types for structs below
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
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
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
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
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}


#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(Debug)]
pub enum Surround {
    Avg       = 1,
    Dim       = 2,
    Dark      = 3,
    Cutsheet  = 4,
}

pub const D_CALCULATE: f64 = -1.;

#[repr(C)]
pub struct _cmsContext_struct { _opaque_type: [u8; 0] }
// Each context holds its owns globals and its own plug-ins. There is a global context with the id = 0 for lecacy compatibility
// though using the global context is not recommended. Proper context handling makes lcms more thread-safe.
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
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
/// This describes a curve segment.
///
/// For a table of supported types, see the manual. User can increase the number of
/// available types by using a proper plug-in. Parametric segments allow 10 parameters at most
pub struct CurveSegment {
    pub x0: f32,
    pub x1: f32,
    pub Type: i32,
    pub Params: [f64; 10],
    pub nGridPoints: u32,
    pub SampledPoints: *mut f32,
}
impl Default for CurveSegment {
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}

pub enum ToneCurve { }
pub enum Pipeline { }
pub enum Stage { }

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
/// Where to place/locate the stages in the pipeline chain
pub enum StageLoc {
    AT_BEGIN = 0,
    AT_END = 1,
}
pub type SAMPLER16 = unsafe extern "C" fn(input: *const u16, output: *mut u16, user_data: *mut c_void) -> i32;
pub type SAMPLERFLOAT = unsafe extern "C" fn(input: *const f32, output: *mut f32, user_data: *mut c_void) -> i32;

#[repr(C)]
pub struct MLU { _opaque_type: [u8; 0] }

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
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
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
    #[inline]
    fn default() -> Self { unsafe { MaybeUninit::zeroed().assume_init() } }
}
#[repr(C)]
pub struct NAMEDCOLORLIST { _opaque_type: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
/// Profile sequence descriptor.
///
/// Some fields come from profile sequence descriptor tag, others
/// come from Profile Sequence Identifier Tag
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
pub enum InfoType {
    Description = 0,
    Manufacturer = 1,
    Model = 2,
    Copyright = 3,
}
#[repr(C)]
pub struct IOHANDLER { _opaque_type: [u8; 0] }


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Intent {
    /// ICC Intents
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,

    /// non-icc intents
    PreserveKOnlyPerceptual = 10,
    PreserveKOnlyRelativeColorimetric = 11,
    PreserveKOnlySaturation = 12,
    PreserveKPlanePerceptual = 13,
    PreserveKPlaneRelativeColorimetric = 14,
    PreserveKPlaneSaturation = 15,
}

// Flags

/// Inhibit 1-pixel cache
pub const FLAGS_NOCACHE: u32 =                  0x0040;
/// Inhibit optimizations
pub const FLAGS_NOOPTIMIZE: u32 =               0x0100;
/// Don't transform anyway
pub const FLAGS_NULLTRANSFORM: u32 =            0x0200;

/// Proofing flags
/// Out of Gamut alarm
pub const FLAGS_GAMUTCHECK: u32 =               0x1000;
/// Do softproofing
pub const FLAGS_SOFTPROOFING: u32 =             0x4000;

// Misc
pub const FLAGS_BLACKPOINTCOMPENSATION: u32 =   0x2000;
/// Don't fix scum dot
pub const FLAGS_NOWHITEONWHITEFIXUP: u32 =      0x0004;
/// Use more memory to give better accurancy
pub const FLAGS_HIGHRESPRECALC: u32 =           0x0400;
/// Use less memory to minimize resources
pub const FLAGS_LOWRESPRECALC: u32 =            0x0800;

/// For devicelink creation
/// Create 8 bits devicelinks
pub const FLAGS_8BITS_DEVICELINK: u32 =         0x0008;
/// Guess device class (for transform2devicelink)
pub const FLAGS_GUESSDEVICECLASS: u32 =         0x0020;
/// Keep profile sequence for devicelink creation
pub const FLAGS_KEEP_SEQUENCE: u32 =            0x0080;

/// Specific to a particular optimizations
/// Force CLUT optimization
pub const FLAGS_FORCE_CLUT: u32 =               0x0002;
/// create postlinearization tables if possible
pub const FLAGS_CLUT_POST_LINEARIZATION: u32 =  0x0001;
/// create prelinearization tables if possible
pub const FLAGS_CLUT_PRE_LINEARIZATION: u32 =   0x0010;

/// Specific to unbounded mode
/// Prevent negative numbers in floating point transforms
pub const FLAGS_NONEGATIVES: u32 =              0x8000;

/// Alpha channels are copied on `cmsDoTransform()`
pub const FLAGS_COPY_ALPHA: u32 =           0x04000000;

// Fine-tune control over number of gridpoints
#[must_use]
#[inline]
pub fn FLAGS_GRIDPOINTS(n: u32) -> u32 { ((n) & 0xFF) << 16 }

/// CRD special
pub const FLAGS_NODEFAULTRESOURCEDEF: u32 =     0x01000000;

/// Use this flag to prevent changes being written to destination.
pub const SAMPLER_INSPECT: u32 = 0x01000000;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[derive(Debug)]
pub enum PSResourceType {
    PS_RESOURCE_CSA = 0,
    PS_RESOURCE_CRD = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VideoSignalType {
    pub ColourPrimaries: u8,
    pub TransferCharacteristics: u8,
    pub MatrixCoefficients: u8,
    pub VideoFullRangeFlag: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MHC2Type {
    pub CurveEntries: u32,
    pub RedCurve: *mut f64,
    pub GreenCurve: *mut f64,
    pub BlueCurve: *mut f64,
    /// ST.2086 min luminance in nits
    pub MinLuminance: f64,
    /// ST.2086 peak luminance in nits
    pub PeakLuminance: f64,
    pub XYZ2XYZmatrix: [[f64; 4]; 3],
}

/// back compat only
#[doc(hidden)]
#[deprecated(note = "use MHC2Type")]
pub type cmsMHC2Type = MHC2Type;

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
    pub fn cmsGetToneCurveSegment(n: u32, t: &ToneCurve) -> &CurveSegment;
    pub fn cmsGetStageContextID(mpe: *const Stage) -> Context;
    pub fn cmsBuildSegmentedToneCurve(ContextID: Context, nSegments: u32, Segments: *const CurveSegment) -> *mut ToneCurve;
    pub fn cmsBuildParametricToneCurve(ContextID: Context, Type: i32, Params: *const f64) -> *mut ToneCurve;
    pub fn cmsBuildGamma(ContextID: Context, Gamma: f64) -> *mut ToneCurve;
    pub fn cmsBuildTabulatedToneCurve16(ContextID: Context, nEntries: u32, values: *const u16) -> *mut ToneCurve;
    pub fn cmsBuildTabulatedToneCurveFloat(ContextID: Context, nEntries: u32, values: *const f32) -> *mut ToneCurve;
    pub fn cmsFreeToneCurve(Curve: *mut ToneCurve);
    pub fn cmsFreeToneCurveTriple(Curve: *mut *mut ToneCurve);
    pub fn cmsDupToneCurve(Src: *const ToneCurve) -> *mut ToneCurve;
    pub fn cmsReverseToneCurve(InGamma: *const ToneCurve) -> *mut ToneCurve;
    pub fn cmsReverseToneCurveEx(nResultSamples: u32, InGamma: *const ToneCurve) -> *mut ToneCurve;
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
    pub fn cmsPipelineInsertStage(lut: *mut Pipeline, loc: StageLoc, mpe: *mut Stage) -> Bool;
    pub fn cmsPipelineUnlinkStage(lut: *mut Pipeline, loc: StageLoc, mpe: *mut *mut Stage);
    /// This function is quite useful to analyze the structure of a Pipeline and retrieve the Stage elements
    /// that conform the Pipeline.
    ///
    /// It should be called with the Pipeline, the number of expected elements and
    /// then a list of expected types followed with a list of double pointers to Stage elements. If
    /// the function founds a match with current pipeline, it fills the pointers and returns TRUE
    /// if not, returns FALSE without touching anything.
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
    pub fn cmsMLUsetUTF8(mlu: *mut MLU, LanguageCode: *const c_char, CountryCode: *const c_char, UTF8String: *const c_char) -> Bool;
    pub fn cmsMLUsetWide(mlu: *mut MLU, LanguageCode: *const c_char, CountryCode: *const c_char, WideString: *const wchar_t) -> Bool;
    pub fn cmsMLUgetASCII(mlu: *const MLU, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
    pub fn cmsMLUgetUTF8(mlu: *const MLU, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
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
    pub fn cmsReadRawTag(hProfile: HPROFILE, sig: TagSignature, Buffer: *mut c_void, BufferSize: u32) -> u32;
    pub fn cmsWriteRawTag(hProfile: HPROFILE, sig: TagSignature, data: *const c_void, Size: u32) -> Bool;
    pub fn cmsGetHeaderFlags(hProfile: HPROFILE) -> u32;
    pub fn cmsGetHeaderAttributes(hProfile: HPROFILE, Flags: *mut u64);
    pub fn cmsGetHeaderProfileID(hProfile: HPROFILE, ProfileID: *mut u8);
    pub fn cmsGetHeaderCreationDateTime(hProfile: HPROFILE, Dest: *mut tm) -> Bool;
    pub fn cmsGetHeaderRenderingIntent(hProfile: HPROFILE) -> Intent;
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
    pub fn cmsCreateDeviceLinkFromCubeFile(cFileName: *const c_char) -> HPROFILE;
    pub fn cmsCreateDeviceLinkFromCubeFileTHR(ContextID: Context, cFileName: *const c_char) -> HPROFILE;
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
    pub fn cmsChannelsOfColorSpace(ColorSpace: ColorSpaceSignature) -> i32;
    pub fn cmsFormatterForColorspaceOfProfile(hProfile: HPROFILE, nBytes: u32, lIsFloat: Bool) -> u32;
    pub fn cmsFormatterForPCSOfProfile(hProfile: HPROFILE, nBytes: u32, lIsFloat: Bool) -> u32;
    pub fn cmsGetProfileInfo(hProfile: HPROFILE, Info: InfoType, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut wchar_t, BufferSize: u32) -> u32;
    pub fn cmsGetProfileInfoASCII(hProfile: HPROFILE, Info: InfoType, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
    pub fn cmsGetProfileInfoUTF8(hProfile: HPROFILE, Info: InfoType, LanguageCode: *const c_char, CountryCode: *const c_char, Buffer: *mut c_char, BufferSize: u32) -> u32;
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
    pub fn cmsCreate_OkLabProfile(ContextID: Context) -> HPROFILE;
    pub fn cmsCreateBCHSWabstractProfileTHR(ContextID: Context, nLUTPoints: u32, Bright: f64, Contrast: f64, Hue: f64, Saturation: f64, TempSrc: u32, TempDest: u32) -> HPROFILE;
    pub fn cmsCreateBCHSWabstractProfile(nLUTPoints: u32, Bright: f64, Contrast: f64, Hue: f64, Saturation: f64, TempSrc: u32, TempDest: u32) -> HPROFILE;
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
    /// Deprecated
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
    // Estimate total area coverage
    pub fn cmsDetectTAC(hProfile: HPROFILE) -> f64;
    pub fn cmsDesaturateLab(Lab: *mut CIELab, amax: f64, amin: f64, bmax: f64, bmin: f64) -> Bool;
    pub fn cmsDetectRGBProfileGamma(hProfile: HPROFILE, threshold: f64) -> f64;
}
