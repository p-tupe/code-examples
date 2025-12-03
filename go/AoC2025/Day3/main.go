// This code has a bug
package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func main() {
	sum := 0

	for bank := range strings.FieldsSeq(ip) {
		joltages := strings.Split(bank, "")

		// Find the highest digit with len(tail) > 11
		maxIdx := -1
		for {
			maxIdx = slices.Index(joltages, slices.Max(joltages))

			if len(joltages[maxIdx:]) < 12 {
				joltages = slices.Replace(joltages, maxIdx, maxIdx+1, "0")
			} else {
				break
			}
		}

		// Then remove all min digits until len(tail) == 12
		joltages = strings.Split(bank, "")[maxIdx:]
		for len(joltages) > 12 {
			minIdx := slices.Index(joltages, slices.Min(joltages))
			joltages = slices.Delete(joltages, minIdx, minIdx+1)
		}

		// Reamining digit is the max
		maxNum, err := strconv.Atoi(strings.Join(joltages, ""))
		if err != nil {
			panic(err)
		}

		sum += maxNum
	}

	fmt.Println("Sum: ", sum)
}
