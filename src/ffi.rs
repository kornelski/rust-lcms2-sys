#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::os::raw::{c_char, c_int, c_long, c_void};
use libc::{wchar_t, tm, FILE};

pub type cmsSignature = u32;
pub type cmsS15Fixed16Number = i32;
pub type cmsBool = c_int;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsTagTypeSignature {
    cmsSigChromaticityType = 1667789421,
    cmsSigColorantOrderType = 1668051567,
    cmsSigColorantTableType = 1668051572,
    cmsSigCrdInfoType = 1668441193,
    cmsSigCurveType = 1668641398,
    cmsSigDataType = 1684108385,
    cmsSigDictType = 1684628340,
    cmsSigDateTimeType = 1685350765,
    cmsSigDeviceSettingsType = 1684371059,
    cmsSigLut16Type = 1835430962,
    cmsSigLut8Type = 1835430961,
    cmsSigLutAtoBType = 1832993312,
    cmsSigLutBtoAType = 1833058592,
    cmsSigMeasurementType = 1835360627,
    cmsSigMultiLocalizedUnicodeType = 1835824483,
    cmsSigMultiProcessElementType = 1836082548,
    cmsSigNamedColorType = 1852010348,
    cmsSigNamedColor2Type = 1852009522,
    cmsSigParametricCurveType = 1885434465,
    cmsSigProfileSequenceDescType = 1886610801,
    cmsSigProfileSequenceIdType = 1886611812,
    cmsSigResponseCurveSet16Type = 1919120178,
    cmsSigS15Fixed16ArrayType = 1936077618,
    cmsSigScreeningType = 1935897198,
    cmsSigSignatureType = 1936287520,
    cmsSigTextType = 1952807028,
    cmsSigTextDescriptionType = 1684370275,
    cmsSigU16Fixed16ArrayType = 1969632050,
    cmsSigUcrBgType = 1650877472,
    cmsSigUInt16ArrayType = 1969828150,
    cmsSigUInt32ArrayType = 1969828658,
    cmsSigUInt64ArrayType = 1969829428,
    cmsSigUInt8ArrayType = 1969827896,
    cmsSigVcgtType = 1986226036,
    cmsSigViewingConditionsType = 1986618743,
    cmsSigXYZType = 1482250784,
}
pub const cmsSigBlueMatrixColumnTag: cmsTagSignature =
    cmsTagSignature::cmsSigBlueColorantTag;
pub const cmsSigGreenMatrixColumnTag: cmsTagSignature =
    cmsTagSignature::cmsSigGreenColorantTag;
pub const cmsSigRedMatrixColumnTag: cmsTagSignature =
    cmsTagSignature::cmsSigRedColorantTag;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsTagSignature {
    cmsSigAToB0Tag = 1093812784,
    cmsSigAToB1Tag = 1093812785,
    cmsSigAToB2Tag = 1093812786,
    cmsSigBlueColorantTag = 1649957210,
    cmsSigBlueTRCTag = 1649693251,
    cmsSigBToA0Tag = 1110589744,
    cmsSigBToA1Tag = 1110589745,
    cmsSigBToA2Tag = 1110589746,
    cmsSigCalibrationDateTimeTag = 1667329140,
    cmsSigCharTargetTag = 1952543335,
    cmsSigChromaticAdaptationTag = 1667785060,
    cmsSigChromaticityTag = 1667789421,
    cmsSigColorantOrderTag = 1668051567,
    cmsSigColorantTableTag = 1668051572,
    cmsSigColorantTableOutTag = 1668050804,
    cmsSigColorimetricIntentImageStateTag = 1667852659,
    cmsSigCopyrightTag = 1668313716,
    cmsSigCrdInfoTag = 1668441193,
    cmsSigDataTag = 1684108385,
    cmsSigDateTimeTag = 1685350765,
    cmsSigDeviceMfgDescTag = 1684893284,
    cmsSigDeviceModelDescTag = 1684890724,
    cmsSigDeviceSettingsTag = 1684371059,
    cmsSigDToB0Tag = 1144144432,
    cmsSigDToB1Tag = 1144144433,
    cmsSigDToB2Tag = 1144144434,
    cmsSigDToB3Tag = 1144144435,
    cmsSigBToD0Tag = 1110590512,
    cmsSigBToD1Tag = 1110590513,
    cmsSigBToD2Tag = 1110590514,
    cmsSigBToD3Tag = 1110590515,
    cmsSigGamutTag = 1734438260,
    cmsSigGrayTRCTag = 1800688195,
    cmsSigGreenColorantTag = 1733843290,
    cmsSigGreenTRCTag = 1733579331,
    cmsSigLuminanceTag = 1819635049,
    cmsSigMeasurementTag = 1835360627,
    cmsSigMediaBlackPointTag = 1651208308,
    cmsSigMediaWhitePointTag = 2004119668,
    cmsSigNamedColorTag = 1852010348,
    cmsSigNamedColor2Tag = 1852009522,
    cmsSigOutputResponseTag = 1919251312,
    cmsSigPerceptualRenderingIntentGamutTag = 1919510320,
    cmsSigPreview0Tag = 1886545200,
    cmsSigPreview1Tag = 1886545201,
    cmsSigPreview2Tag = 1886545202,
    cmsSigProfileDescriptionTag = 1684370275,
    cmsSigProfileDescriptionMLTag = 1685283693,
    cmsSigProfileSequenceDescTag = 1886610801,
    cmsSigProfileSequenceIdTag = 1886611812,
    cmsSigPs2CRD0Tag = 1886610480,
    cmsSigPs2CRD1Tag = 1886610481,
    cmsSigPs2CRD2Tag = 1886610482,
    cmsSigPs2CRD3Tag = 1886610483,
    cmsSigPs2CSATag = 1886597747,
    cmsSigPs2RenderingIntentTag = 1886597737,
    cmsSigRedColorantTag = 1918392666,
    cmsSigRedTRCTag = 1918128707,
    cmsSigSaturationRenderingIntentGamutTag = 1919510322,
    cmsSigScreeningDescTag = 1935897188,
    cmsSigScreeningTag = 1935897198,
    cmsSigTechnologyTag = 1952801640,
    cmsSigUcrBgTag = 1650877472,
    cmsSigViewingCondDescTag = 1987405156,
    cmsSigViewingConditionsTag = 1986618743,
    cmsSigVcgtTag = 1986226036,
    cmsSigMetaTag = 1835365473,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsTechnologySignature {
    cmsSigDigitalCamera = 1684234605,
    cmsSigFilmScanner = 1718838126,
    cmsSigReflectiveScanner = 1920164718,
    cmsSigInkJetPrinter = 1768580468,
    cmsSigThermalWaxPrinter = 1953980792,
    cmsSigElectrophotographicPrinter = 1701865583,
    cmsSigElectrostaticPrinter = 1702065249,
    cmsSigDyeSublimationPrinter = 1685288290,
    cmsSigPhotographicPaperPrinter = 1919969391,
    cmsSigFilmWriter = 1718645358,
    cmsSigVideoMonitor = 1986618477,
    cmsSigVideoCamera = 1986618467,
    cmsSigProjectionTelevision = 1886024822,
    cmsSigCRTDisplay = 1129468960,
    cmsSigPMDisplay = 1347240992,
    cmsSigAMDisplay = 1095582752,
    cmsSigPhotoCD = 1263551300,
    cmsSigPhotoImageSetter = 1768777587,
    cmsSigGravure = 1735549302,
    cmsSigOffsetLithography = 1868981875,
    cmsSigSilkscreen = 1936288875,
    cmsSigFlexography = 1718379896,
    cmsSigMotionPictureFilmScanner = 1836082803,
    cmsSigMotionPictureFilmRecorder = 1836082802,
    cmsSigDigitalMotionPictureCamera = 1684893795,
    cmsSigDigitalCinemaProjector = 1684236912,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsColorSpaceSignature {
    cmsSigXYZData = 1482250784,
    cmsSigLabData = 1281450528,
    cmsSigLuvData = 1282766368,
    cmsSigYCbCrData = 1497588338,
    cmsSigYxyData = 1501067552,
    cmsSigRgbData = 1380401696,
    cmsSigGrayData = 1196573017,
    cmsSigHsvData = 1213421088,
    cmsSigHlsData = 1212961568,
    cmsSigCmykData = 1129142603,
    cmsSigCmyData = 1129142560,
    cmsSigMCH1Data = 1296255025,
    cmsSigMCH2Data = 1296255026,
    cmsSigMCH3Data = 1296255027,
    cmsSigMCH4Data = 1296255028,
    cmsSigMCH5Data = 1296255029,
    cmsSigMCH6Data = 1296255030,
    cmsSigMCH7Data = 1296255031,
    cmsSigMCH8Data = 1296255032,
    cmsSigMCH9Data = 1296255033,
    cmsSigMCHAData = 1296255041,
    cmsSigMCHBData = 1296255042,
    cmsSigMCHCData = 1296255043,
    cmsSigMCHDData = 1296255044,
    cmsSigMCHEData = 1296255045,
    cmsSigMCHFData = 1296255046,
    cmsSigNamedData = 1852662636,
    cmsSig1colorData = 826494034,
    cmsSig2colorData = 843271250,
    cmsSig3colorData = 860048466,
    cmsSig4colorData = 876825682,
    cmsSig5colorData = 893602898,
    cmsSig6colorData = 910380114,
    cmsSig7colorData = 927157330,
    cmsSig8colorData = 943934546,
    cmsSig9colorData = 960711762,
    cmsSig10colorData = 1094929490,
    cmsSig11colorData = 1111706706,
    cmsSig12colorData = 1128483922,
    cmsSig13colorData = 1145261138,
    cmsSig14colorData = 1162038354,
    cmsSig15colorData = 1178815570,
    cmsSigLuvKData = 1282766411,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsProfileClassSignature {
    cmsSigInputClass = 1935896178,
    cmsSigDisplayClass = 1835955314,
    cmsSigOutputClass = 1886549106,
    cmsSigLinkClass = 1818848875,
    cmsSigAbstractClass = 1633842036,
    cmsSigColorSpaceClass = 1936744803,
    cmsSigNamedColorClass = 1852662636,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsPlatformSignature {
    cmsSigMacintosh = 1095782476,
    cmsSigMicrosoft = 1297303124,
    cmsSigSolaris = 1398099543,
    cmsSigSGI = 1397180704,
    cmsSigTaligent = 1413959252,
    cmsSigUnices = 711879032,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsStageSignature {
    cmsSigCurveSetElemType = 1668707188,
    cmsSigMatrixElemType = 1835103334,
    cmsSigCLutElemType = 1668052340,
    cmsSigBAcsElemType = 1648444243,
    cmsSigEAcsElemType = 1698775891,
    cmsSigXYZ2LabElemType = 1815246880,
    cmsSigLab2XYZElemType = 2016570400,
    cmsSigNamedColorElemType = 1852009504,
    cmsSigLabV2toV4 = 840971296,
    cmsSigLabV4toV2 = 874525216,
    cmsSigIdentityElemType = 1768189472,
    cmsSigLab2FloatPCS = 1681026080,
    cmsSigFloatPCS2Lab = 1815241760,
    cmsSigXYZ2FloatPCS = 1681029152,
    cmsSigFloatPCS2XYZ = 2016568352,
    cmsSigClipNegativesElemType = 1668050976,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cmsCurveSegSignature {
    cmsSigFormulaCurveSeg = 1885434470,
    cmsSigSampledCurveSeg = 1935764838,
    cmsSigSegmentedCurve = 1668641382,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cmsICCData {
    pub len: u32,
    pub flag: u32,
    pub data: [u8; 1usize],
    _bindgen_padding_0_: [u8; 3usize],
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
