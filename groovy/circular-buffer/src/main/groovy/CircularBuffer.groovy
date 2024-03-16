class EmptyBufferException extends Exception {}
class FullBufferException extends Exception {}

class CircularBuffer {

    private int[] buffer
    private int capacity
    private int readIndex

    CircularBuffer(int capacity) {
        this.capacity = capacity
        buffer = new int[capacity]
        readIndex = 0
    }

    def clear() {
        throw new UnsupportedOperationException('Clear implementation is missing')
    }

    def read() {
        throw new UnsupportedOperationException('Read implementation is missing')
    }

    def write(int item) {
        throw new UnsupportedOperationException('Write implementation is missing')
    }

    def overwrite(int item) {
        throw new UnsupportedOperationException('Overwrite implementation is missing')
    }
}
