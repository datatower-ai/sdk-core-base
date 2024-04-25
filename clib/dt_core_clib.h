#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int8_t dt_init(const char *raw_config);

int8_t dt_add_event(const char *raw_event);

void dt_flush(void);

void dt_close(void);

void dt_toggle_logger(uint8_t enable);
