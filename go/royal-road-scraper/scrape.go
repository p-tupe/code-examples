// RoyalRoadScrapper is a cli utility to download stories from royalroad.com.
//
// USAGE:
//
//	go run scrape.go royalroad.com/fiction/x/y/chapter/x/y > story.txt
package main

import (
	"fmt"
	"net/http"
	"os"
	"strings"

	"github.com/PuerkitoBio/goquery"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Error while parsing URL!\n\rUSAGE: go run scrape.go <URL>")
		return
	}
	url := os.Args[1]

	for {
		res, err := http.Get(url)
		if err != nil {
			fmt.Println("Error while fetching url: ", err)
			return
		}
		if res.StatusCode != 200 {
			fmt.Println("Error while parsing response:", res.Status)
			return
		}
		defer res.Body.Close()
		url = ""

		doc, err := goquery.NewDocumentFromReader(res.Body)
		title := doc.Find("div.row.fic-header > h1").Text()
		content := doc.Find("div.chapter-content").Text()
		content = strings.ReplaceAll(content, "\n", "\n\n")
		content = strings.Trim(content, "\n ")
		os.Stdout.Write([]byte("\n" + title + "\n\n" + content + "\n"))

		for _, s := range doc.Find("a.btn-primary").EachIter() {
			content := s.Text()
			content = strings.ReplaceAll(content, "\n", "\n\n")
			isNextBtn := strings.Contains(content, "Next Chapter")
			if isNextBtn {
				nextUrl, exists := s.Attr("href")
				if exists {
					url = "https://www.royalroad.com" + nextUrl
					break
				}
			}
		}

		if url == "" {
			break
		}
	}
}
