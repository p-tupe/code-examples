package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

const inputFile = "./input.txt"

func main() {
	first, second := parseIP()

	distance := calcDist(first, second)
	fmt.Println("Distance:", distance)

	similarity := calcSim(first, second)
	fmt.Println("Similarity:", similarity)
}

// parseIP reads the raw string from inputFile and
// returns the 2 slices of location ids.
func parseIP() ([]int, []int) {
	rawIp, err := os.ReadFile(inputFile)
	if err != nil {
		panic(err)
	}

	str := string(rawIp)
	first := make([]int, 0, len(str)/2)
	second := make([]int, 0, len(str)/2)

	for row := range strings.SplitSeq(str, "\n") {
		nums := strings.Fields(row)
		if len(nums) != 2 {
			break
		}

		num1, err := strconv.Atoi(nums[0])
		if err != nil {
			panic(err)
		}
		first = append(first, num1)

		num2, err := strconv.Atoi(nums[1])
		if err != nil {
			panic(err)
		}
		second = append(second, num2)
	}

	return first, second
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
