#include <stdlib.h>
#include <stdint.h>

void for_each_interesting_number(void (*callback)(int32_t number, void *data), void *data);