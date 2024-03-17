class EmptyBufferException() extends Exception {}

class FullBufferException() extends Exception {}

class CircularBuffer(var capacity: Int) {
    private var buffer: List[Int] = List()

    def write(value: Int) = {
        if (capacity == buffer.length) {
            throw new FullBufferException()
        }
        buffer = buffer :+ value
    }

    def read(): Int = {
        if (buffer.isEmpty) {
            throw new EmptyBufferException()
        }
        val value = buffer.head
        buffer = buffer.tail
        value
    }

    def overwrite(value: Int) = {
        if (capacity == buffer.length) {
            read()
        }
        write(value)
    }

    def clear() = {
        buffer = List()
    }
}
