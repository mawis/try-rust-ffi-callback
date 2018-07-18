#include "ccode.h"

void register_callback(callback_func f, void* usr) {
    (*f)(usr);
}
