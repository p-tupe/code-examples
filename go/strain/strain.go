package strain

import "slices"

func Keep[E any, Slice ~[]E](slice Slice, predicate func(e E) bool) Slice {
	newSlice := make([]E, 0, len(slice))

	for _, ele := range slice {
		if predicate(ele) {
			newSlice = append(newSlice, ele)
		}
	}

	return slices.Clip(newSlice)
}

func Discard[E any, Slice ~[]E](slice Slice, predicate func(e E) bool) Slice {
	return Keep(slice, func(e E) bool { return !predicate(e) })
}
