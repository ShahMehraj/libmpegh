typedef struct
{
  char *frame;
  unsigned int frame_length;
  unsigned int presentation_time;
} it_mp4_frame_cntxt;

typedef struct
{
  char *header;
  unsigned int header_length;
} it_mp4_header_cntxt;

typedef struct ia_file_wrapper
{
  void *mp4_cntxt;
  void *file_cntxt;
  void *interim_buffer;
  signed int avail_buffer;
  it_mp4_header_cntxt header_cntxt;
  it_mp4_frame_cntxt frame_cntxt;
  signed int header_given;
  unsigned int is_mp4_file;
  unsigned int is_mp4_mhm1;
  unsigned int is_mp4_dash;
  FILE *input_file;
  signed int size_dash;
  signed int loc;
  signed int offset_dash;
  unsigned int is_execution;

} ia_file_wrapper;