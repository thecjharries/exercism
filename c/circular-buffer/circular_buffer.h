#ifndef CIRCULAR_BUFFER_H
#define CIRCULAR_BUFFER_H

typedef int buffer_value_t;

typedef struct
{
    buffer_value_t *buffer;
    int capacity;
    int head;
    int tail;
} circular_buffer_t;

#endif
