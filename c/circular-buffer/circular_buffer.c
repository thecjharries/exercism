#include "circular_buffer.h"

#include <stdlib.h>
#include <errno.h>

circular_buffer_t *new_circular_buffer(int capacity)
{
    circular_buffer_t *buffer = malloc(sizeof(circular_buffer_t));
    buffer->buffer = malloc(capacity * sizeof(buffer_value_t));
    buffer->capacity = capacity;
    buffer->head = 0;
    buffer->tail = 0;
    buffer->full = 0;
    return buffer;
}

void clear_buffer(circular_buffer_t *buffer)
{
    buffer->head = 0;
    buffer->tail = 0;
    buffer->full = 0;
}

void delete_buffer(circular_buffer_t *buffer)
{
    free(buffer->buffer);
    free(buffer);
}

unsigned int write(circular_buffer_t *buffer, buffer_value_t value)
{
    if (buffer->full)
    {
        errno = ENOBUFS;
        return EXIT_FAILURE;
    }
    buffer->buffer[buffer->head] = value;
    buffer->head = (buffer->head + 1) % buffer->capacity;
    if (buffer->head == buffer->tail)
    {
        buffer->full = 1;
    }
    return EXIT_SUCCESS;
}

unsigned int overwrite(circular_buffer_t *buffer, buffer_value_t value)
{
    if (!buffer->full)
    {
        return write(buffer, value);
    }
    buffer->buffer[buffer->head] = value;
    buffer->head = (buffer->head + 1) % buffer->capacity;
    buffer->tail = (buffer->tail + 1) % buffer->capacity;
    return EXIT_SUCCESS;
}

unsigned int read(circular_buffer_t *buffer, buffer_value_t *value)
{
    if ((buffer->head == buffer->tail) && !buffer->full)
    {
        errno = ENODATA;
        return EXIT_FAILURE;
    }
    *value = buffer->buffer[buffer->tail];
    buffer->tail = (buffer->tail + 1) % buffer->capacity;
    buffer->full = 0;
    return EXIT_SUCCESS;
}
