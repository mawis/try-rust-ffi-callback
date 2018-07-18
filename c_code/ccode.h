#ifndef __CCODE_H
#define __CCODE_H

typedef void (*callback_func)(void* usr);

void register_callback(callback_func f, void* usr);

#endif
