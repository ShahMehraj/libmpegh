const IA_MAX_ERROR_SUB_CODE: usize = 70;
const IA_MPEGH_DEC_NO_ERROR: u32 = 0x00000000;
#[allow(warnings)]
struct ia_error_info_struct<'a> {
    pb_module_name: &'a str,
    ppb_class_names: [&'a str; 16],
    ppppb_error_msg_pointers: [[[&'a str; 16]; 7]; 2],
}

static IMPEGHD_PPB_API_FATAL: &[&str] = &[
    "NULL Pointer: Memory Allocation Error",
];

static IMPEGHD_PPB_CONFIG_NON_FATAL: &[&str] = &[
    "Invalid pcm word size value",
    "Invalid target loudness value",
    "Invalid effect type",
    "Invalid cicp index",
    "Invalid preset id",
];

static IMPEGHD_PPB_CONFIG_FATAL: &[&str] = &[""];

static IMPEGHD_PPB_INIT_NON_FATAL: &[&str] = &[
    "Insufficient input bytes",
    "Insufficient element interaction bytes",
    "Insufficient local setup information bytes",
    "Insufficient binaural renderer impulse response bytes",
    "Invalid resample ratio",
    "Insufficient mae bytes",
];

static IMPEGHD_PPB_DRC_INIT_NON_FATAL: &[&str] = &[
    "Insufficient bits in bitbuffer",
    "Unexpected error",
    "Parameter error",
    "Unsupported channel groups",
    "DRC coefficients exceeded maximum value",
    "Selection process Initialization failed",
    "Unsupported ducking sequence",
    "DRC instructions exceeded maximum value",
];

static IMPEGHD_PPB_HOA_INIT_NON_FATAL: &[&str] = &[
    "HOA renderer Initialization failed",
    "Invalid HOA order",
    "Invalid quantization",
    "Invalid framesize",
    "Invalid matrix",
    "Invalid transport channel",
    "Invalid subband config",
    "Invalid bitsize",
    "Matrix mismatch",
    "Invalid space position type",
    "Spherical wave model not implemented",
    "Space position Initialization failed",
];

static IMPEGHD_PPB_INIT_FATAL: &[&str] = &[
    "Decoder initialization failed",
    "No. of channels in stream greater than max channels defined",
    "Sample rate is not supported",
    "Zero Receiver Compensation Delay not valid for multisignal groups",
    "Invalid fill bytes",
    "3D audio config data not found",
    "Invalid SBR framelength index",
    "Invalid CICP speaker index",
    "Wrong window sequence",
    "Unsupported number of ASI groups",
    "Unsupported framelength",
    "Unsupported ASI switch groups",
    "Unsupported ASI group presets",
    "MHAS syncword mismatch",
    "Number of config extensions exceeded Maximum",
    "Number of speakers exceeded Maximum",
    "Initialization fatal error",
    "Config extension length exceeded",
    "Unexpected error",
    "Number of earcons exceeded Maximum",
    "Number of ASI description blocks exceeded Maximum",
    "Number of ASI description languages exceeded Maximum",
    "Number of CICP layouts exceeded Maximum",
    "Unsupported number of pcm signals",
    "Unsupported ILD index",
    "Persist memory pointer is NULL",
    "Number of signal groups exceeded maximum",
    "Unsupported MPEG-H Profile",
    "Invalid configuration parameters",
    "Channel mask 1 not supported for any LFE channel",
    "Unsupported lpd mode",
    "Unsupported elevation angle index",
    "Unsupported azimuth angle index",
    "Unsupported igf all zero",
    "Invalid fac data present flag",
    "Invalid short fac flag",
    "Invalid mct entries",
    "Invalid channel pair index",
    "Invalid downmix config type",
    "Invalid downmix type",
    "Invalid downmix id",
    "Invalid speaker distance",
    "Unsupported cicp speaker index",
    "Unsupported extension element config",
    "Audio preroll parsing failed",
    "Invalid delta time",
    "ASI parse failed",
    "ASI preset group definition config failed",
    "ASI preset group definition extension config failed",
    "Invalid group id",
    "Complex prediction not supported",
    "Invalid preset group number of conditions",
    "Invalid configuration for LC profile",
    "Invalid number of downmix id group preset extension",
    "Invalid number of wired outputs",
    "Invalid number of shifted channel",
    "Invalid max sfb",
    "Invalid ASI parameter",
    "Invalid config for number of channels",
    "Unsupported signal group type",
];

static IMPEGHD_PPB_DRC_INIT_FATAL: &[&str] = &[
    "Exceeded channel group count",
    "Invalid gain set index",
    "Unsupported method definition",
    "Unexpected error",
    "Sample rate is not supported",
    "Framesize is not supported",
    "Channel count is not supported",
    "Invalid delta tmin value",
    "Unsupported delay mode",
    "Unsupported delay samples",
    "Unsupported subband domain mode",
    "Unsupported number of subbands",
    "Invalid drc parameter for LC profile",
];

static IMPEGHD_PPB_HOA_INIT_FATAL: &[&str] = &[
    "Invalid vector index",
    "HOA renderer matrix initialization failed",
    "HOA Render initialization failed",
    "Invalid framelength indicator",
    "Invalid matrix selected",
    "Invalid number of channels",
    "Invalid active directional signal ids",
    "Invalid minimum amb order",
    "Invalid vector length",
    "Invalid diff order",
    "Invalid no of vq element bits",
    "Invalid hoa independency flag",
    "Invalid hoa nbitsq",
    "Invalid hoa amb coef trans state",
    "Invalid hoa amb coef index",
    "Invalid vvec index",
    "Invalid sparse order",
    "NFC not allowed",
    "Invalid configuration for LC profile",
];

static IMPEGHD_PPB_BINAURAL_INIT_FATAL: &[&str] = &[
    "Binaural renderer initialization failed",
    "Binaural renderer unsupported diffuse length or direct length",
];

static IMPEGHD_PPB_FC_INIT_FATAL: &[&str] = &[
    "Invalid parameter",
    "Format converter initialization failed",
    "Invalid channel number",
    "Invalid CICP Index",
    "Invalid precision level",
    "Invalid gain table size",
    "Unsupported speaker layout",
    "Compact template not found",
    "Invalid number of compact configurations",
    "Invalid number of equalizers",
    "Invalid equalizer configuration",
];

static IMPEGHD_PPB_OAM_INIT_FATAL: &[&str] = &[
    "Unsupported number of OAM objects",
];

static IMPEGHD_PPB_EI_INIT_FATAL: &[&str] = &[
    "Unsupported interaction mode",
    "Unsupported number of groups",
    "Invalid id",
    "Invalid change position",
    "Invalid change gain",
];

static IMPEGHD_PPB_EXE_NON_FATAL: &[&str] = &[
    "Insufficient input bytes ",
    "Insufficient scene displacement bytes",
    "Insufficient HOA bytes",
    "Insufficient meta data bytes",
];

static IMPEGHD_PPB_EXE_FATAL: &[&str] = &[
    "Exceeded maximum downmix matrices per id",
    "Unsupported window length",
    "Unsupported window shape",
    "Invalid TNS sfb",
    "sfb info NULL",
    "Unsupported number of channels",
    "Invalid stereo config",
    "Unsupported element index",
    "Invalid window shape combination",
    "LPD dec handle NULL",
    "Invalid pitch lag",
    "Invalid fd factor",
    "Invalid pitch",
    "Exceeded maximum downmix matrix elements",
    "Exceeded maximum multichannel bands",
    "Coefficients Pointer NULL",
    "Invalid preset id",
    "No suitable track found",
    "MCT process failed",
    "Invalid channel configuration",
    "Unsupported MHAS packet type",
    "Unsupported number of prerolls",
    "Decode frame error",
    "MDP process failed",
    "Arthmetic decode failed",
    "Invalid Sampling frequency for LPD mode",
    "Exceeded maximum scale factor band index",
    "Invalid start band value",
    "Assigned group ids exceeded maximum",
    "Invalid group index",
    "Invalid fac length",
    "Invalid max sfb value",
    "Invalid configuration parameters",
    "TD Config handle NULL",
    "Uni DRC coefficients pointer NULL",
    "Invalid scale factor",
    "sfb exceeded maximum value",
    "Domain switcher process failed",
    "Invalid compatible profile set",
];

static IMPEGHD_PPB_DRC_EXE_FATAL: &[&str] = &[
    "Invalid drc instructions index",
    "DRC process failed",
    "DRC unsupported subband domain mode",
];

static IMPEGHD_PPB_HOA_EXE_FATAL: &[&str] = &[
    "HOA spatial process failed",
    "HOA scratch pointer NULL",
    "Invalid hoa coded gain",
];

static IMPEGHD_PPB_BINAURAL_EXE_FATAL: &[&str] = &[
    "Unsupported number of speakers",
    "Unsupported number of channels",
    "Unsupported data format",
    "Filter parameter computation failed",
    "Unsupported number of taps",
    "Unsupported filter length",
];

static IMPEGHD_PPB_OAM_EXE_FATAL: &[&str] = &[
    "Object metadata decoding is unsupported",
    "Framelength is unsupported",
];

static IMPEGHD_PPB_MAE_EXE_FATAL: &[&str] = &[
    "MAE divergence error",
    "Invalid OAM index",
    "MAE interactivity violation error",
    "No diffusion error",
    "Unsupported elevation range",
    "Unsupported azimuth angle",
    "Unsupported group preset",
    "ASI and interaction data number of groups differ",
    "Switch group off",
    "On off interaction failed",
];

static IMP_EGHD_ERROR_INFO: ia_error_info_struct<'static> = ia_error_info_struct {
    pb_module_name: "Ittiam mpegh_dec",
    ppb_class_names: [
        "API",
        "Configuration",
        "Initialization",
        "Execution",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "MPEGHD",
    ],
    ppppb_error_msg_pointers: [
        [
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
        ],
        [
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
        ],
    ],
};

fn impeghd_error_handler_init() {
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][0][0] = Some(&IMPEGHD_PPB_API_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[0][1][0] = Some(&IMPEGHD_PPB_CONFIG_NON_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][1][0] = Some(&IMPEGHD_PPB_CONFIG_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[0][2][0] = Some(&IMPEGHD_PPB_INIT_NON_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[0][2][1] = Some(&IMPEGHD_PPB_DRC_INIT_NON_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[0][2][2] = Some(&IMPEGHD_PPB_HOA_INIT_NON_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][2][0] = Some(&IMPEGHD_PPB_INIT_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][2][1] = Some(&IMPEGHD_PPB_DRC_INIT_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][2][2] = Some(&IMPEGHD_PPB_HOA_INIT_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][2][3] = Some(&IMPEGHD_PPB_BINAURAL_INIT_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][2][4] = Some(&IMPEGHD_PPB_FC_INIT_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][2][5] = Some(&IMPEGHD_PPB_OAM_INIT_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][2][6] = Some(&IMPEGHD_PPB_EI_INIT_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[0][3][0] = Some(&IMPEGHD_PPB_EXE_NON_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][3][0] = Some(&IMPEGHD_PPB_EXE_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][3][1] = Some(&IMPEGHD_PPB_DRC_EXE_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][3][2] = Some(&IMPEGHD_PPB_HOA_EXE_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][3][3] = Some(&IMPEGHD_PPB_BINAURAL_EXE_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][3][5] = Some(&IMPEGHD_PPB_OAM_EXE_FATAL);
    IMP_EGHD_ERROR_INFO.ppppb_error_msg_pointers[1][3][6] = Some(&IMPEGHD_PPB_MAE_EXE_FATAL);
}

static IMPEGHD_PPB_IA_TESTBENCH_MEM_FILE_MAN_FATAL: &[&str] = &[
    "File Open Failed",
    "Wave header writing Failed",
];

static IMPEGHD_IA_TESTBENCH_ERROR_INFO: ia_error_info_struct<'static> = ia_error_info_struct {
    pb_module_name: "ia_testbench",
    ppb_class_names: [
        "", "", "", "", "Memory & File Manager", "", "", "", "", "", "", "", "", "", "", "",
    ],
    ppppb_error_msg_pointers: [
        [
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
        ],
        [
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
            [
                "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
            ],
        ],
    ],
};

fn ia_testbench_error_handler_init(
    error_info: &mut ia_error_info_struct<'static>,
    error_messages: &[&'static str; IA_MAX_ERROR_SUB_CODE],
) {
    error_info.ppppb_error_msg_pointers[1][4][0] = Some(&error_messages);
}


fn impeghd_error_handler(
    p_mod_err_info: &ia_error_info_struct,
    pb_context: Option<&str>,
    code: i32,
) -> i32 {
    if code == IA_MPEGH_DEC_NO_ERROR {
        return IA_MPEGH_DEC_NO_ERROR;
    }

    let is_fatal = ((code & 0x8000) >> 15) != 0;
    let err_class = ((code & 0x7800) >> 11) as usize;
    let mod_name = ((code & 0x0700) >> 8) as usize;
    let err_sub_code = (code & 0x00FF) as usize;

    println!();
    if !is_fatal {
        println!("non ");
    }
    println!("fatal error: ");

    if let Some(module_name) = p_mod_err_info.pb_module_name {
        println!("{}  ", module_name);
    }

    if let Some(module_name) = p_mod_err_info.pb_module_name {
        if module_name != "ia_testbench" {
            match mod_name {
                0 => println!("core coder "),
                1 => println!("DRC "),
                2 => println!("HOA "),
                3 => println!("Binaural Renderer "),
                4 => println!("Format Converter "),
                5 => println!("OAM "),
                6 => println!("MAE Preprocessor "),
                _ => {}
            }
            println!("module :");
        }
    }

    if let Some(class_name) = p_mod_err_info.ppb_class_names.get(err_class) {
        if let Some(class_name) = class_name {
            println!("{}: ", class_name);
        }
    }
    if let Some(context) = pb_context {
        println!("{}: ", context);
    }

    if err_sub_code >= IA_MAX_ERROR_SUB_CODE
        || p_mod_err_info.ppppb_error_msg_pointers[is_fatal as usize][err_class][mod_name]
            .as_ref()
            .and_then(|arr| arr.get(err_sub_code))
            .is_none()
    {
        println!("error unlisted");
    } else {
        if let Some(error_msg) = p_mod_err_info.ppppb_error_msg_pointers[is_fatal as usize][err_class][mod_name]
            .as_ref()
            .and_then(|arr| arr.get(err_sub_code))
            .and_then(|&msg| msg)
        {
            println!("{}", error_msg);
        }
    }

    IA_MPEGH_DEC_NO_ERROR
}


fn main() {
  println!("hi\n");
}
