#include "gen.h"

void for_each_interesting_number(void (*callback)(int32_t number, void *data), void *data)
 {
    callback(2, data);
    callback(3, data);
    callback(5, data);
    callback(7, data);
    callback(11, data);
    callback(13, data);
    callback(17, data);
    callback(19, data);
}