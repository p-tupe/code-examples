package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func main() {
	first, second, err := ParseIP(ip)
	if err != nil {
		panic(err)
	}

	slices.Sort(first)
	slices.Sort(second)

	distance := 0
	for i, f := range first {
		s := second[i]
		d := f - s
		if d > 0 {
			distance += d
		} else {
			distance -= d
		}
	}

	fmt.Println("Distance: ", distance)
}

// ParseIP takes in the raw string input and
// returns the 2 slices of location ids.
// It returns an error if a number cannot be parsed,
// or it the arrays aren't of same length
func ParseIP(str string) ([]int, []int, error) {
	first := make([]int, 0, len(str)/2)
	second := make([]int, 0, len(str)/2)

	for row := range strings.SplitSeq(str, "\n") {
		nums := strings.Fields(row)

		num1, err := strconv.Atoi(nums[0])
		if err != nil {
			return nil, nil, err
		}
		first = append(first, num1)

		num2, err := strconv.Atoi(nums[1])
		if err != nil {
			return nil, nil, err
		}
		second = append(second, num2)
	}

	return first, second, nil
}
