package spiralmatrix

func SpiralMatrix(size int) [][]int {
	if size == 0 {
		return [][]int{}
	}

	matrix := make([][]int, size)
	for i := range matrix {
		matrix[i] = make([]int, size)
	}

	x, y := 0, 0
	dx, dy := 1, 0
	for i := 1; i <= size*size; i++ {
		matrix[y][x] = i
		if x+dx < 0 || x+dx >= size || y+dy < 0 || y+dy >= size || matrix[y+dy][x+dx] != 0 {
			dx, dy = -dy, dx
		}
		x, y = x+dx, y+dy
	}

	return matrix
}
