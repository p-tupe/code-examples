// Package Day1 solves part1/2 of Advent of Code 2025
// Use `go run .` to see the outputs for sample inputs.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	pass := 0
	dial := 50

	for rotation := range strings.FieldsSeq(ip_test) {
		dir := rotation[0]
		amt, err := strconv.Atoi(rotation[1:])
		if err != nil {
			panic(err)
		}

		if dir == 'L' {
			for range amt {
				dial -= 1

				if dial == 0 {
					pass++
				}

				if dial == -1 {
					dial = 99
				}
			}
		} else {
			for range amt {
				dial += 1

				if dial == 100 {
					dial = 0
					pass++
				}
			}
		}
	}

	fmt.Println("Password:", pass)
}
