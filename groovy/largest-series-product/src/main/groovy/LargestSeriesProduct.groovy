class LargestSeriesProduct {
    private static final Closure<Integer> PRODUCT = {
        it.inject(1) { acc, i -> acc * i }
    }
    static largestProduct(String digits, int span) {
        if (span < 0) {
            throw new IllegalArgumentException("span must not be negative")
        }
        if (span > digits.size()) {
            throw new IllegalArgumentException("span must be smaller than string length")
        }
        if (span == 0) {
            return 1
        }
        if (digits =~ /\D/) {
            throw new IllegalArgumentException("digits input must only contain digits")
        }
        digits.collect { it as int }
              .collate(span, 1, false)
              .collect(PRODUCT)
              .max()
    }
}
