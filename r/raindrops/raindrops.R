raindrops <- function(number) {
  result <- ""
  if (0 == number %% 3) {
    result <- paste(result, "Pling", sep = "")
  }
  if (0 == number %% 5) {
    result <- paste(result, "Plang", sep = "")
  }
  if (0 == number %% 7) {
    result <- paste(result, "Plong", sep = "")
  }
  if ("" == result) {
    result <- as.character(number)
  }
  return(result)
}
