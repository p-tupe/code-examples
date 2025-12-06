// Code for part 2 is buggy
package main

import (
	"fmt"
	"strconv"
	"strings"
)

// This is the solution for the first part
func main() {
	froms := []uint64{}
	tos := []uint64{}
	freshCounter := 0

	for row := range strings.FieldsSeq(ip) {
		if strings.Contains(row, "-") {
			rng := strings.Split(row, "-")

			from, err := strconv.ParseUint(rng[0], 10, 64)
			if err != nil {
				panic(err)
			}
			froms = append(froms, from)

			to, err := strconv.ParseUint(rng[1], 10, 64)
			if err != nil {
				panic(err)
			}
			tos = append(tos, to)
		} else {
			id, err := strconv.ParseUint(row, 10, 64)
			if err != nil {
				panic(err)
			}

			// Since IDs come _after_ range, we can safely check them right here
			for i, from := range froms {
				to := tos[i]

				if id >= from && id <= to {
					freshCounter++
					break
				}
			}
		}
	}

	fmt.Println("freshCounter", freshCounter)
}

// 313651768181637 = too low
// 377601631202416 = too high

// This is the solution for part 2
/*
 * 					start 					   end
 *						 |---------------|
 * from 							 to
 * 	 |---------------|  <-- Tail Overlap
 * 										from 						 to
 *	Head overlap --> 	 |---------------|
 * 							from 	 			 to
 *								 |---------| <-- Full Overlap
 * 																	from 							 to
 *									No Overlap	-->	 |---------------|
 *  from 	 to
 *  	 |---| <-- No Overlap
 */
// func main() {
// 	idRange := map[uint64]uint64{}
// 	var rangeCount uint64
//
// 	for row := range strings.FieldsSeq(ip) {
// 		if row == "" {
// 			break
// 		}
//
// 		if strings.Contains(row, "-") {
// 			rng := strings.Split(row, "-")
//
// 			from, err := strconv.ParseUint(rng[0], 10, 64)
// 			if err != nil {
// 				panic(err)
// 			}
//
// 			to, err := strconv.ParseUint(rng[1], 10, 64)
// 			if err != nil {
// 				panic(err)
// 			}
//
// 			// Check if "from"-"to" is part of any overlap
// 			var subCount uint64
// 			for start, end := range idRange {
// 				// No Overlap
// 				if from > end || to < start {
// 					continue
// 				}
//
// 				// Full Overlap
// 				if from >= start && to <= end {
// 					subCount += to - from + 1
// 				}
//
// 				// Tail Overlap
// 				if from < start && to >= start {
// 					subCount += to - start + 1
// 				}
//
// 				// Head Overlap
// 				if to > end && from <= end {
// 					subCount += end - from + 1
// 				}
// 			}
//
// 			idRange[from] = to
// 			rangeCount += to - from + 1 - subCount
// 		}
// 	}
//
// 	fmt.Println("Range Count: ", rangeCount)
// }
