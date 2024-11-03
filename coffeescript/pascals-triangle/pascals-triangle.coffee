class PascalsTriangle
  rows: (num) ->
    triangle = []
    for i in [0...num]
        triangle[i] = []
        for j in [0..i]
            if j == 0 or j == i
                triangle[i][j] = 1
            else
                triangle[i][j] = triangle[i-1][j-1] + triangle[i-1][j]
    triangle

module.exports = PascalsTriangle
