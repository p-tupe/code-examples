package bookstore

const COST_PER_BOOK = 800               // in cents
var DISCOUNT_PER_SET = map[int]float64{ // in percent
	1: 0.00,
	2: 0.05,
	3: 0.10,
	4: 0.20,
	5: 0.25,
}

func Cost(books []int) int {
	cost := 0.0

	// Convert list of books into count of copies for each
	// Index 0-4 is for book id 1-5 resp
	// Eg books {1,2,2,3} => copies {1,2,1,0,0}
	var copies [5]int
	for _, book := range books {
		copies[book-1]++
	}

	// Convert all copies into sets of unique books
	// Index 1-5 is for sets of len 1-5 resp
	// Eg copies {1,2,1,0,0} => sets {_,1,0,1,0,0}
	var sets [6]int
	for {
		setLen := 0
		for i, c := range copies {
			if c != 0 {
				setLen++
				copies[i]--
			}
		}
		if setLen == 0 {
			break
		}
		sets[setLen]++
	}

	// Convert sets of 5/3 to sets of 4/4
	// Eg sets {_,0,0,1,0,1} => sets {_,0,0,0,2,0}
	for sets[5] > 0 && sets[3] > 0 {
		sets[5]--
		sets[3]--
		sets[4] += 2
	}

	// Calculate cost with discounts applied per set
	for i, set := range sets {
		cost += COST_PER_BOOK * float64(set*i) * (1.0 - DISCOUNT_PER_SET[i])
	}

	return int(cost)
}
