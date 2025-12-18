// https://exercism.org/tracks/go/exercises/book-store/solutions/rabestro
package bookstore

import (
	"math"
	"sort"
)

func Costx(booksInStore []int) int {
	books := []int{0, 0, 0, 0, 0}
	for _, book := range booksInStore {
		books[book-1]++
	}
	sort.Ints(books)

	p1 := books[4] - books[3]
	p2 := books[3] - books[2]
	p3 := books[2] - books[1]
	p4 := books[1] - books[0]
	p5 := books[0]

	pc := int(math.Min(float64(p3), float64(p5)))

	return 8 * (100*p1 + 95*2*p2 + 90*3*(p3-pc) + 80*4*(p4+2*pc) + 75*5*(p5-pc))
}
