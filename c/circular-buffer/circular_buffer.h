#ifndef CIRCULAR_BUFFER_H
#define CIRCULAR_BUFFER_H

typedef int buffer_value_t;

typedef struct
{
    buffer_value_t *buffer;
    int capacity;
    int head;
    int tail;
    int full;
} circular_buffer_t;

circular_buffer_t *new_circular_buffer(int capacity);
void clear_buffer(circular_buffer_t *buffer);
void delete_buffer(circular_buffer_t *buffer);
unsigned int write(circular_buffer_t *buffer, buffer_value_t value);
unsigned int overwrite(circular_buffer_t *buffer, buffer_value_t value);
unsigned int read(circular_buffer_t *buffer, buffer_value_t *value);

#endif
