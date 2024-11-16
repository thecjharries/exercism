function triangle(n)
    n == 0 && return []
    n < 0 && throw(DomainError("n must be a positive integer."))
    return [[binomial(i, j) for j in 0:i] for i in 0:n-1]
end
