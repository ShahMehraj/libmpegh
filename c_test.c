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
  impeghd_error_info.ppppb_error_msg_pointers[0][1][0] = impeghd_ppb_config_non_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[1][1][0] = impeghd_ppb_config_fatal;
  impeghd_error_info.ppppb_error_msg_pointers[0][2][0] = impeghd_ppb_init_non_fatal;
}
int main() {
    //
    return 0;
}
