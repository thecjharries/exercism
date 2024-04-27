class Squares {
    let squareOfSum: Int
    let sumOfSquares: Int
    let differenceOfSquares: Int

    init(_ n: Int) {
        let sum = n * (n + 1) / 2
        squareOfSum = sum * sum
        sumOfSquares = n * (n + 1) * (2 * n + 1) / 6
        differenceOfSquares = squareOfSum - sumOfSquares
    }
}
