// This packages shows how to quickly connect to an sqlite database
// using an external package.
package main

import (
	"database/sql"
	"fmt"

	_ "github.com/ncruces/go-sqlite3/driver"
)

func main() {
	db, _ := sql.Open("sqlite3", ":memory:")
	row := db.QueryRow(`SELECT sqlite_version();`)

	v := ""
	if err := row.Scan(&v); err != nil {
		fmt.Println("Error: ", err)
	}

	fmt.Println(v)
}
