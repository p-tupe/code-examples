package strand

import "regexp"

var DNA2RNA = map[string]string{"G": "C", "C": "G", "T": "A", "A": "U"}

var re = regexp.MustCompile(`(A|C|G|T)`)

func ToRNA(dna string) string {
	return re.ReplaceAllStringFunc(dna, func(s string) string { return DNA2RNA[s] })
}
