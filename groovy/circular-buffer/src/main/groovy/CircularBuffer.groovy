class EmptyBufferException extends Exception {}
class FullBufferException extends Exception {}

class CircularBuffer {

    private int[] buffer
    private int size
    private int readIndex

    CircularBuffer(int capacity) {
        buffer = new int[capacity]
        readIndex = 0
        size = 0
    }

    def clear() {
        size = 0
    }

    def read() {
        if (0 == size) {
            throw new EmptyBufferException()
        }
        int item = buffer[readIndex]
        readIndex = (readIndex + 1) % buffer.length
        size--
        return item
    }

    def write(int item) {
        if (buffer.length == size) {
            throw new FullBufferException()
        }
        overwrite(item)
    }

    def overwrite(int item) {
        buffer[(readIndex + size) % buffer.length] = item
        if (buffer.length == size) {
            readIndex = (readIndex + 1) % buffer.length
        } else {
            size++
        }
    }
}
