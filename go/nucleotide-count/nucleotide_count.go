package dna

import "fmt"

type Nucleotide rune

// Histogram is a mapping from nucleotide to its count in given DNA.
type Histogram map[Nucleotide]int

// DNA is a list of nucleotides.
type DNA []Nucleotide

// Counts generates a histogram of valid nucleotides in the given DNA.
// Returns an error if d contains an invalid nucleotide.
func (d DNA) Counts() (Histogram, error) {
	histogram := Histogram{'A': 0, 'C': 0, 'G': 0, 'T': 0}

	for _, nucleotide := range d {
		_, valid := histogram[nucleotide]
		if valid {
			histogram[nucleotide]++
		} else {
			return nil, fmt.Errorf("Invalid nucleotide: %v", nucleotide)
		}
	}

	return histogram, nil
}
