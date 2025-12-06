package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	grid := [][]string{}
	for row := range strings.SplitSeq(ip, "\n") {
		cols := make([]string, 0, len(row))
		for col := range strings.SplitSeq(row, "") {
			cols = append(cols, col)
		}
		grid = append(grid, cols)
	}

	total := 0
	str := ""
	for col := len(grid[0]) - 1; col >= 0; col-- {
		for row := 0; row <= len(grid)-1; row++ {
			val := grid[row][col]

			switch val {
			case "+", "*":
				total += calc(str, val)
				str = ""
			default:
				str += val
			}
		}
	}

	fmt.Println("Total", total)
}

func calc(str string, op string) int {
	var output int
	if op == "*" {
		output = 1
	}

	for xStr := range strings.FieldsSeq(str) {
		x, err := strconv.Atoi(xStr)
		if err != nil {
			panic(err)
		}
		if op == "*" {
			output *= x
		} else {
			output += x
		}
	}
	return output
}
