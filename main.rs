const IA_MPEGH_DEC_NO_ERROR: i32 = 0x00000000;

struct IaErrorInfoStruct {
    pb_module_name: &'static str,
    ppb_class_names: [&'static str; 16],
    ppppb_error_msg_pointers: [[[Option<&'static [&'static str]>; 16]; 7]; 2],
}

static IMPEGHD_PPB_API_FATAL: [&'static str; 1] = ["NULL Pointer: Memory Allocation Error"];
static IMPEGHD_PPB_CONFIG_NON_FATAL: [&'static str; 5] = [
    "Invalid pcm word size value",
    "Invalid target loudness value",
    "Invalid effect type",
    "Invalid cicp index",
    "Invalid preset id",
];
static IMPEGHD_PPB_CONFIG_FATAL: [&'static str; 1] = [""];
static IMPEGHD_PPB_INIT_NON_FATAL: [&'static str; 6] = [
    "Insufficient input bytes",
    "Insufficient element interaction bytes",
    "Insufficient local setup information bytes",
    "Insufficient binaural renderer impulse response bytes",
    "Invalid resample ratio",
    "Insufficient mae bytes",
];

static mut IMPEGHD_ERROR_INFO: IaErrorInfoStruct = IaErrorInfoStruct {
    pb_module_name: "Ittiam mpegh_dec",
    ppb_class_names: [
        "API", "Configuration", "Initialization", "Execution", "", "", "", "", "", "", "", "", "",
        "", "", "MPEGHD",
    ],
    ppppb_error_msg_pointers: [
        [
            [None, None, None, None, None, None, None, None, None, None, None, None, None, None,
             None, None],
            [None, None, None, None, None, None, None, None, None, None, None, None, None, None,
             None, None],
            [None, None, None, None, None, None,None,None,None,None,None,None,None,None,None,None],
            [None; 16], [None; 16], [None; 16], [None; 16]
        ],
        [
            [None; 16], [None; 16], [None; 16], [None; 16], [None; 16], [None; 16], [None; 16]
        ]
    ],
};

fn impeghd_error_handler_init() {
    unsafe {
        IMPEGHD_ERROR_INFO.ppppb_error_msg_pointers[1][0][0] = Some(&IMPEGHD_PPB_API_FATAL);
        IMPEGHD_ERROR_INFO.ppppb_error_msg_pointers[0][1][0] = Some(&IMPEGHD_PPB_CONFIG_NON_FATAL);
        IMPEGHD_ERROR_INFO.ppppb_error_msg_pointers[1][1][0] = Some(&IMPEGHD_PPB_CONFIG_FATAL);
        IMPEGHD_ERROR_INFO.ppppb_error_msg_pointers[0][2][0] = Some(&IMPEGHD_PPB_INIT_NON_FATAL);
    }
}