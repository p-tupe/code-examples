// Package read-stdin shows how write a func to read from standard input
// similar to common unix commands like `cat` or `echo`.
//
// If given an argument, it is considered a file. If no argument is given,
// it starts ingesting from stdin until end of line.
//
// Usage:
//
//	go run main.go test.txt
//	echo "hello" | go run main.go
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var r *bufio.Reader

	if len(os.Args) > 1 {
		f, err := os.Open(os.Args[1:][0])
		if err != nil {
			panic(err)
		}
		defer f.Close()
		r = bufio.NewReader(f)
	} else {
		r = bufio.NewReader(os.Stdin)
	}

	str, err := r.ReadString('\n')
	if err != nil {
		panic(err)
	}
	fmt.Println("Received:", str)
}
