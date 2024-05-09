class AllYourBase {

    private decimal = 0

    AllYourBase(inputBase, digits) {
        if (2 > inputBase)
            throw new ArithmeticException("Invalid base")
        digits.each { digit ->
            if (!(0..<inputBase).contains(digit))
                throw new ArithmeticException("Invalid digit")
            decimal = decimal * inputBase + digit
        }
    }

    def rebase(outputBase) {
        if (2 > outputBase)
            throw new ArithmeticException("Invalid base")
        if (0 === decimal)
            return [0]
        def digits = []
        def reduction = decimal
        while (0 < reduction) {
            digits.push(reduction % outputBase)
            reduction = reduction.intdiv(outputBase)
        }
        digits
    }
}
