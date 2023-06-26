mod impeghd_error;
mod impeghd_memory_standards;
use impeghd_error::IaErrorInfoStruct;
use impeghd_memory_standards::IaMpeghdApiStruct;
use impeghd_error::IA_MPEGH_DEC_NO_ERROR;
use impeghd_error::impeghd_error_handler_init;
use impeghd_error::ia_testbench_error_handler_init;

use std::fs::File;
use std::ffi::c_int;
use std::ffi::c_char;
use std::io::Write;

const IA_MAX_CMD_LINE_LENGTH:i32 = 30;
const IA_MAX_ARGS: i32 = 20;
const IA_SCREEN_WIDTH: i32 = 80;
const PARAMFILE: &str = "paramfilesimple.txt";

const IA_TESTBENCH_MFMAN_FATAL_FILE_OPEN_FAILED: u32 = 0xFFFFA000;
const IA_TESTBENCH_FATAL_WAV_HDR_WRITE_FAIL: u32 = 0xFFFFA001;


static mut G_PF_OUT: Option<File> = None;
static mut G_PF_BRIR: Option<File> = None;
static mut G_BINAURAL_FLAG: c_int = 0;
static mut G_PF_EI_BS: Option<File> = None;
static mut G_PF_LSI_BS: Option<File> = None;
static mut G_PF_SD_BS: Option<File> = None;
static mut G_PF_EXT_REN_OAM_MD: Option<File> = None;
static mut G_PF_EXT_REN_CH_MD: Option<File> = None;
static mut G_PF_EXT_REN_HOA_MD: Option<File> = None;
static mut G_PF_EXT_REN_PCM: Option<File> = None;
static mut OUT_FILENAME: [c_char; IA_MAX_CMD_LINE_LENGTH as usize] = [0; IA_MAX_CMD_LINE_LENGTH as usize];
static mut RAW_TESTING: c_int = 0;

#[cfg(WAV_HEADER)]
fn impeghd_write16_bits_lh(fp: &mut File, i: c_int) {
    fp.write_all(&[(i & 0xff) as u8]).unwrap();
    fp.write_all(&[((i >> 8) & 0xff) as u8]).unwrap();
}

#[cfg(WAV_HEADER)]
fn impeghd_write32_bits_lh(fp: &mut File, i: c_int) {
    impeghd_write16_bits_lh(fp, i & 0xffff);
    impeghd_write16_bits_lh(fp, (i >> 16) & 0xffff);
}

fn write_wav_header(fp: &mut File, pcmbytes: c_int, freq: c_int, channels: c_int, bits: c_int) -> c_int {
    #[cfg(WAV_HEADER)]
    {
        #[cfg(not(ARM_PROFILE_BOARD))]
        {
            let bytes = (bits + 7) / 8;
            fp.write_all(b"RIFF").unwrap();
            impeghd_write32_bits_lh(fp, pcmbytes + 44 - 8);
            fp.write_all(b"WAVEfmt ").unwrap();
            impeghd_write32_bits_lh(fp, 2 + 2 + 4 + 4 + 2 + 2);
            impeghd_write16_bits_lh(fp, 1);
            impeghd_write16_bits_lh(fp, channels);
            impeghd_write32_bits_lh(fp, freq);
            impeghd_write32_bits_lh(fp, freq * channels * bytes);
            impeghd_write16_bits_lh(fp, channels * bytes);
            impeghd_write16_bits_lh(fp, bits);
            fp.write_all(b"data").unwrap();
            impeghd_write32_bits_lh(fp, pcmbytes);
        }
    }

    match fp.flush() {
        Ok(_) => IA_MPEGH_DEC_NO_ERROR,
        Err(_) => IA_TESTBENCH_FATAL_WAV_HDR_WRITE_FAIL as i32,
    }
}

#[cfg(DISPLAY_MESSAGE)]
fn ia_display_id_message(lib_name: &str, lib_version: &str) {
    let mut str_arr: [String; 4] = [
        String::from("ITTIAM SYSTEMS PVT LTD, BANGALORE\n"),
        String::from("http:\\\\www.ittiam.com\n"),
        String::new(),
        String::new(),
    ];

    let mut spaces: String = " ".repeat(IA_SCREEN_WIDTH / 2);

    str_arr[2].push_str(lib_name);
    str_arr[2].push_str(lib_version);
    str_arr[2].push('\n');
    str_arr[3].push('\n');

    let spaces_len = IA_SCREEN_WIDTH / 2 - (str_arr[0].len() / 2) as usize;
    spaces.truncate(spaces_len);
  
    for str_item in &str_arr {
        print!("{}", spaces);
        print!("{}", str_item);
    }
}

fn impeghd_parse_config_param(argc: i32, argv: Vec<&str>, ptr_dec_api: IaMpeghdApiStruct) -> i32 {
    let mut i: i32;
    let pstr_dec_api: IaMpeghdApiStruct = ptr_dec_api;
    for i in 0..argc {
        if let Some(arg) = argv.get(i) {
            if arg.starts_with("-pcmsz:") {
                let pb_arg_val = &arg[7..];
                pstr_dec_api.input_config.ui_pcm_wd_sz = pb_arg_val.parse::<u32>().unwrap();
            }
        }

        if !argv[i as usize].starts_with("-effect:") {
            let pb_arg_val = &argv[i as usize][8..];
            pstr_dec_api.input_config.ui_effect = pb_arg_val.parse().unwrap();
        }

        if !argv[i as usize].starts_with("-target_loudness:") {
            let pb_arg_val = &argv[i as usize][17..];
            pstr_dec_api.input_config.ui_target_loudness[0] = 1;
            pstr_dec_api.input_config.ui_target_loudness[1] = pb_arg_val.parse().unwrap();

            if pstr_dec_api.input_config.ui_target_loudness[1] > 0
                || pstr_dec_api.input_config.ui_target_loudness[1] < -63
            {
                pstr_dec_api.input_config.ui_target_loudness[1] = 0;
            }

            pstr_dec_api.input_config.ui_target_loudness[1] =
                -(pstr_dec_api.input_config.ui_target_loudness[1] << 2);
        }

        if !argv[i as usize].starts_with("-cicp:") {
            let pb_arg_val = &argv[i as usize][6..];
            pstr_dec_api.input_config.ui_cicp_layout_idx = pb_arg_val.parse().unwrap();
        }

        if !argv[i as usize].starts_with("-preset:") {
            let pb_arg_val = &argv[i as usize][8..];
            pstr_dec_api.input_config.i_preset_id = pb_arg_val.parse().unwrap() as i8;
        }

        if !argv[i as usize].starts_with("-iei:") {
            let mut ei_sz = 0;
            // Perform the equivalent file operations in Rust to get ei_sz
            pstr_dec_api.input_config.ptr_ei_buf = vec![0u8; ei_sz + 1];
            if pstr_dec_api.input_config.ptr_ei_buf.len() > 0 {
                pstr_dec_api.input_config.ei_info_size = fread(
                    &mut pstr_dec_api.input_config.ptr_ei_buf[..ei_sz],
                    1,
                    ei_sz,
                    G_PF_EI_BS,
                ) as i32;
            }
            fclose(G_PF_EI_BS);
            G_PF_EI_BS = std::ptr::null_mut();
            pstr_dec_api.input_config.ei_info_flag = 1;
        }

        if !argv[i as usize].starts_with("-ilsi:") {
            let mut lsi_sz = 0;
            // Perform the equivalent file operations in Rust to get lsi_sz
            pstr_dec_api.input_config.ptr_ls_buf = vec![0u8; lsi_sz + 1];
            if pstr_dec_api.input_config.ptr_ls_buf.len() > 0 {
                pstr_dec_api.input_config.lsi_info_size = fread(
                    &mut pstr_dec_api.input_config.ptr_ls_buf[..lsi_sz],
                    1,
                    lsi_sz,
                    G_PF_LSI_BS,
                ) as i32;
            }
            fclose(G_PF_LSI_BS);
            G_PF_LSI_BS = std::ptr::null_mut();
            pstr_dec_api.input_config.lsi_info_flag = 1;
        }

        if !argv[i as usize].starts_with("-isdi:") {
            let mut sdi_sz = 0;
            // Perform the equivalent file operations in Rust to get sdi_sz
            pstr_dec_api.input_config.ptr_sd_buf = vec![0u8; sdi_sz + 1];
            if pstr_dec_api.input_config.ptr_sd_buf.len() > 0 {
                pstr_dec_api.input_config.sd_info_size = fread(
                    &mut pstr_dec_api.input_config.ptr_sd_buf[..sdi_sz],
                    1,
                    sdi_sz,
                    G_PF_SD_BS,
                ) as i32;
            }
            fclose(G_PF_SD_BS);
            G_PF_SD_BS = std::ptr::null_mut();
            pstr_dec_api.input_config.sd_info_flag = 1;
        }

        if !argv[i as usize].starts_with("-ext_ren:") {
            let pb_arg_val = &argv[i as usize][9..];
            pstr_dec_api.input_config.extrn_rend_flag = pb_arg_val.parse().unwrap();
            if pstr_dec_api.input_config.extrn_rend_flag != 0 {
                pstr_dec_api.input_config.ptr_ext_ren_ch_data_buf = vec![0u8; 768];
                pstr_dec_api.input_config.ptr_ext_ren_oam_data_buf = vec![0u8; 768];
                pstr_dec_api.input_config.ptr_ext_ren_hoa_data_buf = vec![0u8; 768];
                pstr_dec_api.input_config.ptr_ext_ren_pcm_buf = vec![0u8; 1024 * 32 * 4];
            }
        }

        if !argv[i as usize].starts_with("-ibrir:") {
            let mut brir_sz = 0;
            // Perform the equivalent file operations in Rust to get brir_sz
            pstr_dec_api.input_config.ptr_brir_buf = vec![0u8; brir_sz + 1];
            if pstr_dec_api.input_config.ptr_brir_buf.len() > 0 {
                pstr_dec_api.input_config.binaural_data_len = fread(
                    &mut pstr_dec_api.input_config.ptr_brir_buf[..brir_sz],
                    1,
                    brir_sz,
                    G_PF_BRIR,
                ) as i32;
            }
            fclose(G_PF_BRIR);
            G_PF_BRIR = std::ptr::null_mut();
            pstr_dec_api.input_config.binaural_flag = 1;
        }

        if !argv[i as usize].starts_with("-out_fs:") {
            let pb_arg_val = &argv[i as usize][8..];
            pstr_dec_api.input_config.out_samp_freq = pb_arg_val.parse().unwrap();
            pstr_dec_api.input_config.enable_resamp = 1;
        }
    }
    return IA_MPEGH_DEC_NO_ERROR;
}

fn main() {
    print!("Compiled Successfully!");
}
