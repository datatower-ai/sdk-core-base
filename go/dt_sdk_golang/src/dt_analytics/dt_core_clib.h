#include "stdint.h"

int8_t dt_init(char*);
int8_t dt_add_event(char*);
void dt_flush();
void dt_close();
void dt_toggle_logger(uint8_t);