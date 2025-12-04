package main

import (
	"fmt"
	"strings"
)

type Grid [][]int

func main() {
	grid := Grid{}

	// Generate a gird of paper rolls, splitting string into slices
	for row := range strings.FieldsSeq(ip) {
		rowSlice := make([]int, 0, len(row))
		for col := range strings.SplitSeq(row, "") {
			if col == "." {
				rowSlice = append(rowSlice, 0)
			} else {
				rowSlice = append(rowSlice, 1)
			}
		}
		grid = append(grid, rowSlice)
	}

	// For each position, check if findNeighbours < 4.
	// If so, remove the roll
	counter := 0
	more := false
	for !more {
		more = true
		for r := range grid {
			for c := range grid[r] {
				if grid[r][c] == 1 && findNeighbours(grid, r, c) < 4 {
					grid[r][c] = 0
					counter++
					more = false
				}
			}
		}
	}
	fmt.Println("Num Rolls removed: ", counter)
}

// findNeighbours of a cell in a given grid by row/column
func findNeighbours(grid Grid, r, c int) int {
	count := 0

	topEdge := r == 0
	bottomEdge := r == len(grid)-1
	leftEdge := c == 0
	rightEdge := c == len(grid[0])-1

	// Left
	if !leftEdge {
		count += grid[r][c-1]
	}

	// Right
	if !rightEdge {
		count += grid[r][c+1]
	}

	if !topEdge {
		// Top
		count += grid[r-1][c]

		// Top Left
		if !leftEdge {
			count += grid[r-1][c-1]
		}

		// Top Right
		if !rightEdge {
			count += grid[r-1][c+1]
		}
	}

	if !bottomEdge {
		// Bottom
		count += grid[r+1][c]

		// Bottom Left
		if !leftEdge {
			count += grid[r+1][c-1]
		}

		// Bottom Right
		if !rightEdge {
			count += grid[r+1][c+1]
		}
	}

	return count
}
