package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func main() {
	sum := 0

	for id := range strings.SplitSeq(ip, ",") {
		rnge := strings.Split(id, "-")

		from, err := strconv.Atoi(rnge[0])
		if err != nil {
			panic(err)
		}

		to, err := strconv.Atoi(rnge[1])
		if err != nil {
			panic(err)
		}

		for x := from; x <= to; x++ {
			num := strconv.Itoa(x)

			for i := 1; i <= len(num)/2; i++ {
				match := true
				chunks := slices.Chunk([]byte(num), i)
				var pattern string

				for chunkByte := range chunks {
					chunkStr := string(chunkByte)

					if len(pattern) == 0 {
						pattern = chunkStr
					}

					if pattern != chunkStr {
						match = false
						break
					}
				}

				if match {
					sum += x
					break
				}
			}
		}
	}

	fmt.Println("Sum: ", sum)
}
