#include <stdio.h>
#include <string.h>

#define IA_MAX_ERROR_SUB_CODE 70
#define IA_MPEGH_DEC_NO_ERROR 0x00000000

typedef struct
{
  signed char *pb_module_name;
  signed char *ppb_class_names[16];
  signed char **ppppb_error_msg_pointers[2][7][16];
} ia_error_info_struct;

void *impeghd_ppb_api_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "NULL Pointer: Memory Allocation Error"};

void *impeghd_ppb_config_non_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Invalid pcm word size value", (signed char *) "Invalid target loudness value",
    (signed char *) "Invalid effect type", (signed char *) "Invalid cicp index", (signed char *) "Invalid preset id"};

void *impeghd_ppb_config_fatal[IA_MAX_ERROR_SUB_CODE] = {(signed char *) ""};

void *impeghd_ppb_init_non_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Insufficient input bytes",
    (signed char *) "Insufficient element interaction bytes",
    (signed char *) "Insufficient local setup information bytes",
    (signed char *) "Insufficient binaural renderer impulse response bytes",
    (signed char *) "Invalid resample ratio",
    (signed char *) "Insufficient mae bytes",
};

void *impeghd_ppb_drc_init_non_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Insufficient bits in bitbuffer",
    (signed char *) "Unexpected error",
    (signed char *) "Parameter error",
    (signed char *) "Unsupported channel groups",
    (signed char *) "DRC coefficients exceeded maximum value",
    (signed char *) "Selection process Initialization failed",
    (signed char *) "Unsupported ducking sequence",
    (signed char *) "DRC instructions exceeded maximum value",
};

void *impeghd_ppb_hoa_init_non_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "HOA renderer Initialization failed",
    (signed char *) "Invalid HOA order",
    (signed char *) "Invalid quantization",
    (signed char *) "Invalid framesize",
    (signed char *) "Invalid matrix",
    (signed char *) "Invalid transport channel",
    (signed char *) "Invalid subband config",
    (signed char *) "Invalid bitsize",
    (signed char *) "Matrix mismatch",
    (signed char *) "Invalid space position type",
    (signed char *) "Spherical wave model not implemented",
    (signed char *) "Space position Initialization failed",
};

void *impeghd_ppb_init_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Decoder initialization failed",
    (signed char *) "No. of channels in stream greater than max channels defined",
    (signed char *) "Sample rate is not supported",
    (signed char *) "Zero Receiver Compensation Delay not valid for multisignal groups",
    (signed char *) "Invalid fill bytes",
    (signed char *) "3D audio config data not found",
    (signed char *) "Invalid SBR framelength index",
    (signed char *) "Invalid CICP speaker index",
    (signed char *) "Wrong window sequence",
    (signed char *) "Unsupported number of ASI groups",
    (signed char *) "Unsupported framelength",
    (signed char *) "Unsupported ASI switch groups",
    (signed char *) "Unsupported ASI group presets",
    (signed char *) "MHAS syncword mismatch",
    (signed char *) "Number of config extensions exceeded Maximum",
    (signed char *) "Number of speakers exceeded Maximum",
    (signed char *) "Initialization fatal error",
    (signed char *) "Config extension length exceeded",
    (signed char *) "Unexpected error",
    (signed char *) "Number of earcons exceeded Maximum",
    (signed char *) "Number of ASI description blocks exceeded Maximum",
    (signed char *) "Number of ASI description languages exceeded Maximum",
    (signed char *) "Number of CICP layouts exceeded Maximum",
    (signed char *) "Unsupported number of pcm signals",
    (signed char *) "Unsupported ILD index",
    (signed char *) "Persist memory pointer is NULL",
    (signed char *) "Number of signal groups exceeded maximum",
    (signed char *) "Unsupported MPEG-H Profile",
    (signed char *) "Invalid configuration parameters",
    (signed char *) "Channel mask 1 not supported for any LFE channel",
    (signed char *) "Unsupported lpd mode",
    (signed char *) "Unsupported elevation angle index",
    (signed char *) "Unsupported azimuth angle index",
    (signed char *) "Unsupported igf all zero",
    (signed char *) "Invalid fac data present flag",
    (signed char *) "Invalid short fac flag",
    (signed char *) "Invalid mct entries",
    (signed char *) "Invalid channel pair index",
    (signed char *) "Invalid downmix config type",
    (signed char *) "Invalid downmix type",
    (signed char *) "Invalid downmix id",
    (signed char *) "Invalid speaker distance",
    (signed char *) "Unsupported cicp speaker index",
    (signed char *) "Unsupported extension element config",
    (signed char *) "Audio preroll parsing failed",
    (signed char *) "Invalid delta time",
    (signed char *) "ASI parse failed",
    (signed char *) "ASI preset group definition config failed",
    (signed char *) "ASI preset group definition extension config failed",
    (signed char *) "Invalid group id",
    (signed char *) "Complex prediction not supported",
    (signed char *) "Invalid preset group number of conditions",
    (signed char *) "Invalid configuration for LC profile",
    (signed char *) "Invalid number of downmix id group preset extension",
    (signed char *) "Invalid number of wired outputs",
    (signed char *) "Invalid number of shifted channel",
    (signed char *) "Invalid max sfb",
    (signed char *) "Invalid ASI parameter",
    (signed char *) "Invalid config for number of channels",
    (signed char *) "Unsupported signal group type",
};

void *impeghd_ppb_drc_init_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Exceeded channel group count",
    (signed char *) "Invalid gain set index",
    (signed char *) "Unsupported method definition",
    (signed char *) "Unexpected error",
    (signed char *) "Sample rate is not supported",
    (signed char *) "Framesize is not supported",
    (signed char *) "Channel count is not supported",
    (signed char *) "Invalid delta tmin value",
    (signed char *) "Unsupported delay mode",
    (signed char *) "Unsupported delay samples",
    (signed char *) "Unsupported subband domain mode",
    (signed char *) "Unsupported number of subbands",
    (signed char *) "Invalid drc parameter for LC profile",
};

void *impeghd_ppb_hoa_init_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Invalid vector index",
    (signed char *) "HOA renderer matrix initialization failed",
    (signed char *) "HOA Render initialization failed",
    (signed char *) "Invalid framelength indicator",
    (signed char *) "Invalid matrix selected",
    (signed char *) "Invalid number of channels",
    (signed char *) "Invalid active directional signal ids",
    (signed char *) "Invalid minimum amb order",
    (signed char *) "Invalid vector length",
    (signed char *) "Invalid diff order",
    (signed char *) "Invalid no of vq element bits",
    (signed char *) "Invalid hoa independency flag",
    (signed char *) "Invalid hoa nbitsq",
    (signed char *) "Invalid hoa amb coef trans state",
    (signed char *) "Invalid hoa amb coef index",
    (signed char *) "Invalid vvec index",
    (signed char *) "Invalid sparse order",
    (signed char *) "NFC not allowed",
    (signed char *) "Invalid configuration for LC profile",
};

void *impeghd_ppb_binaural_init_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Binaural renderer initialization failed",
    (signed char *) "Binaural renderer unsupported diffuse length or direct length",
};

void *impeghd_ppb_fc_init_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Invalid parameter",
    (signed char *) "Format converter initialization failed",
    (signed char *) "Invalid channel number",
    (signed char *) "Invalid CICP Index",
    (signed char *) "Invalid precision level",
    (signed char *) "Invalid gain table size",
    (signed char *) "Unsupported speaker layout",
    (signed char *) "Compact template not found",
    (signed char *) "Invalid number of compact configurations",
    (signed char *) "Invalid number of equalizers",
    (signed char *) "Invalid equalizer configuration",
};

void *impeghd_ppb_oam_init_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Unsupported number of OAM objects",
};

void *impeghd_ppb_ei_init_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Unsupported interaction mode",
    (signed char *) "Unsupported number of groups",
    (signed char *) "Invalid id",
    (signed char *) "Invalid change position",
    (signed char *) "Invalid change gain",
};

void *impeghd_ppb_exe_non_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Insufficient input bytes ", (signed char *) "Insufficient scene displacement bytes",
    (signed char *) "Insufficient HOA bytes", (signed char *) "Insufficient meta data bytes",
};

void *impeghd_ppb_exe_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Exceeded maximum downmix matrices per id",
    (signed char *) "Unsupported window length",
    (signed char *) "Unsupported window shape",
    (signed char *) "Invalid TNS sfb",
    (signed char *) "sfb info NULL",
    (signed char *) "Unsupported number of channels",
    (signed char *) "Invalid stereo config",
    (signed char *) "Unsupported element index",
    (signed char *) "Invalid window shape combination",
    (signed char *) "LPD dec handle NULL",
    (signed char *) "Invalid pitch lag",
    (signed char *) "Invalid fd factor",
    (signed char *) "Invalid pitch",
    (signed char *) "Exceeded maximum downmix matrix elements",
    (signed char *) "Exceeded maximum multichannel bands",
    (signed char *) "Coefficients Pointer NULL",
    (signed char *) "Invalid preset id",
    (signed char *) "No suitable track found",
    (signed char *) "MCT process failed",
    (signed char *) "Invalid channel configuration",
    (signed char *) "Unsupported MHAS packet type",
    (signed char *) "Unsupported number of prerolls",
    (signed char *) "Decode frame error",
    (signed char *) "MDP process failed",
    (signed char *) "Arthmetic decode failed",
    (signed char *) "Invalid Sampling frequency for LPD mode",
    (signed char *) "Exceeded maximum scale factor band index",
    (signed char *) "Invalid start band value",
    (signed char *) "Assigned group ids exceeded maximum",
    (signed char *) "Invalid group index",
    (signed char *) "Invalid fac length",
    (signed char *) "Invalid max sfb value",
    (signed char *) "Invalid configuration parameters",
    (signed char *) "TD Config handle NULL",
    (signed char *) "Uni DRC coefficients pointer NULL",
    (signed char *) "Invalid scale factor",
    (signed char *) "sfb exceeded maximum value",
    (signed char *) "Domain switcher process failed",
    (signed char *) "Invalid compatible profile set",
};


void *impeghd_ppb_drc_exe_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Invalid drc instructions index", (signed char *) "DRC process failed",
    (signed char *) "DRC unsupported subband domain mode",
};


void *impeghd_ppb_hoa_exe_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "HOA spatial process failed", (signed char *) "HOA scratch pointer NULL",
    (signed char *) "Invalid hoa coded gain",
};


void *impeghd_ppb_binaural_exe_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Unsupported number of speakers", (signed char *) "Unsupported number of channels",
    (signed char *) "Unsupported data format", (signed char *) "Filter parameter computation failed",
    (signed char *) "Unsupported number of taps", (signed char *) "Unsupported filter length",
};


void *impeghd_ppb_oam_exe_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "Object metadata decoding is unsupported", (signed char *) "Framelength is unsupported",
};


void *impeghd_ppb_mae_exe_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "MAE divergence error",
    (signed char *) "Invalid OAM index",
    (signed char *) "MAE interactivity violation error",
    (signed char *) "No diffusion error",
    (signed char *) "Unsupported elevation range",
    (signed char *) "Unsupported azimuth angle",
    (signed char *) "Unsupported group preset",
    (signed char *) "ASI and interaction data number of groups differ",
    (signed char *) "Switch group off",
    (signed char *) "On off interaction failed",
};

ia_error_info_struct impeghd_error_info = {

    (signed char *) "Ittiam mpegh_dec",
    {
     (signed char *) "API", (signed char *) "Configuration", (signed char *) "Initialization", (signed char *) "Execution",
     (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "",
     (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "MPEGHD"},
    {
     {{NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL}},
     {{NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL}}}};

void impeghd_error_handler_init()
{

  impeghd_error_info.ppppb_error_msg_pointers[1][0][0] = impeghd_ppb_api_fatal;
  printf("%s", impeghd_error_info.ppppb_error_msg_pointers[1][0][0]);
  impeghd_error_info.ppppb_error_msg_pointers[0][1][0] = impeghd_ppb_config_non_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][1][0] = impeghd_ppb_config_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[0][2][0] = impeghd_ppb_init_non_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[0][2][1] = impeghd_ppb_drc_init_non_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[0][2][2] = impeghd_ppb_hoa_init_non_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][2][0] = impeghd_ppb_init_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][2][1] = impeghd_ppb_drc_init_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][2][2] = impeghd_ppb_hoa_init_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][2][3] = impeghd_ppb_binaural_init_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][2][4] = impeghd_ppb_fc_init_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][2][5] = impeghd_ppb_oam_init_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][2][6] = impeghd_ppb_ei_init_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[0][3][0] = impeghd_ppb_exe_non_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][3][0] = impeghd_ppb_exe_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][3][1] = impeghd_ppb_drc_exe_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][3][2] = impeghd_ppb_hoa_exe_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][3][3] = impeghd_ppb_binaural_exe_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][3][5] = impeghd_ppb_oam_exe_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][3][6] = impeghd_ppb_mae_exe_fatal;
}

void *impeghd_ppb_ia_testbench_mem_file_man_fatal[IA_MAX_ERROR_SUB_CODE] = {
    (signed char *) "File Open Failed", (signed char *) "Wave header writing Failed"};

ia_error_info_struct impeghd_ia_testbench_error_info = {

    (signed char *) "ia_testbench",
    {
     (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "Memory & File Manager",
     (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) "",
     (signed char *) "", (signed char *) "", (signed char *) "", (signed char *) ""},
    {
     {{NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL}},
     {{NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL},
      {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
       NULL}}}};

void ia_testbench_error_handler_init()
{

  impeghd_ia_testbench_error_info.ppppb_error_msg_pointers[1][4][0] =
      impeghd_ppb_ia_testbench_mem_file_man_fatal;
}

int impeghd_error_handler(ia_error_info_struct *p_mod_err_info, signed char *pb_context,
                                   int code)
{
  if (code == IA_MPEGH_DEC_NO_ERROR)
  {
    return IA_MPEGH_DEC_NO_ERROR;
  }

  {
    signed int is_fatal = (((unsigned int)code & 0x8000) >> 15);
    signed int err_class = (((unsigned int)code & 0x7800) >> 11);
    signed int mod_name = (((unsigned int)code & 0x0700) >> 8);
    signed int err_sub_code = (((unsigned int)code & 0x00FF));

    printf("\n");
    if (!is_fatal)
    {
      printf("non ");
    }
    printf("fatal error: ");

    if (p_mod_err_info->pb_module_name != NULL)
    {
      printf("%s  ", p_mod_err_info->pb_module_name);
    }

    if (p_mod_err_info->pb_module_name != NULL &&
        strcmp((const char *)p_mod_err_info->pb_module_name, "ia_testbench"))
    {
      switch (mod_name)
      {
      case 0:
        printf("core coder ");
        break;
      case 1:
        printf("DRC ");
        break;
      case 2:
        printf("HOA ");
        break;
      case 3:
        printf("Binaural Renderer ");
        break;
      case 4:
        printf("Format Converter ");
        break;
      case 5:
        printf("OAM ");
        break;
      case 6:
        printf("MAE Preprocessor ");
        break;
      default:
        break;
      }
      printf("module :");
    }

    if (p_mod_err_info->ppb_class_names[err_class] != NULL)
    {
      printf("%s: ", p_mod_err_info->ppb_class_names[err_class]);
    }
    if (pb_context != NULL)
    {
      printf("%s: ", pb_context);
    }
    if (err_sub_code >= IA_MAX_ERROR_SUB_CODE ||
        p_mod_err_info->ppppb_error_msg_pointers[is_fatal][err_class][mod_name][err_sub_code] ==
            NULL)
    {
      printf("error unlisted\n");
    }
    else
    {
      printf(
          "%s\n",
          p_mod_err_info->ppppb_error_msg_pointers[is_fatal][err_class][mod_name][err_sub_code]);
    }
  }
  return IA_MPEGH_DEC_NO_ERROR;
}


int main()
{
    impeghd_error_handler_init();
    printf("Hello!");
    
}