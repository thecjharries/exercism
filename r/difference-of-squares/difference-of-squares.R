sum_of_squares <- function(n) {
  n * (n + 1) * (2 * n + 1) / 6
}

square_of_sum <- function(n) {
  sum <- n * (n + 1) / 2
  sum * sum
}

difference_of_squares <- function(natural_number) {
  square_of_sum(natural_number) - sum_of_squares(natural_number)
}
