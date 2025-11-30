package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func main() {
	first, second, err := parseIP(ip)
	if err != nil {
		panic(err)
	}

	distance := calcDist(first, second)
	fmt.Println("Distance:", distance)

	similarity := calcSim(first, second)
	fmt.Println("Similarity:", similarity)
}

// parseIP takes in the raw string input and
// returns the 2 slices of location ids.
// It returns an error if a number cannot be parsed,
// or it the arrays aren't of same length
func parseIP(str string) ([]int, []int, error) {
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

// calcDist takes in 2 slices of ints, sorts them
// in asc order, and sums the difference between
// each pair at same index.
func calcDist(first []int, second []int) int {
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

	return distance
}

// calcSim takes in 2 slices of ints, and for each number
// in first list, calculates the frequency at which it appears
// in second list; similarity = sum of all | number * frequency
func calcSim(first []int, second []int) int {
	sim := 0

	for _, f := range first {
		count := 0
		for _, s := range second {
			if f == s {
				count++
			}
		}
		sim += f * count
	}

	return sim
}
