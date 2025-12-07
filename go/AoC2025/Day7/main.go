// This code has a bug (part-2 unsolved)
package main

import (
	"fmt"
	"strings"
)

func main() {
	grid := [][]string{}

	for row := range strings.Lines(ip) {
		col := make([]string, 0, len(row))
		for cell := range strings.SplitSeq(row, "") {
			if cell == "S" {
				cell = "|"
			}
			col = append(col, cell)
		}
		grid = append(grid, col)
	}

	for i, row := range grid {
		// Skip last row
		if i == len(grid)-1 {
			continue
		}

		for j, cell := range row {
			if cell == "|" {
				// if splitter below
				if grid[i+1][j] == "^" {
					// mark the cells on either side as beams
					grid[i+1][j-1] = "|"
					grid[i+1][j+1] = "|"
				} else {
					// mark the one directly below as beam
					grid[i+1][j] = "|"
				}
			}
		}
	}

	fmt.Println(ip)
	fmt.Println(grid)
	// fmt.Println("Timeline Conunt:", currTimelines)
}
