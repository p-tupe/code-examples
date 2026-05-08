package main

import (
	"bufio"
	"encoding/json"
	"errors"
	"io"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./test.log")
	if err != nil {
		log.Fatalln(err)
	}

	stats := Stats{
		Errors_per_endpoint: map[string]int{},
		P95_latency_ms:      map[string]int{},
	}
	_durations := map[string][]int{}
	_ips := map[string]int{}

	reader := bufio.NewReader(file)
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			if errors.Is(io.EOF, err) {
				break
			}

			log.Println(err)
		}
		stats.Total_requests++

		l := parseLog(line)
		if l == nil {
			stats.Skipped_lines++
			continue
		}

		if l.status > 399 {
			stats.Errors_per_endpoint[l.path] += 1
		}

		_durations[l.path] = append(_durations[l.path], l.duration)
		_ips[l.ip]++
	}

	for path, durations := range _durations {
		slices.Sort(durations)
		p95Idx := len(durations) * 95 / 100
		stats.P95_latency_ms[path] = durations[p95Idx]
	}

	for ip, count := range _ips {
		stats.Top_ips = append(stats.Top_ips, ipCount{ip, count})
	}

	slices.SortFunc(stats.Top_ips, func(a, b ipCount) int {
		return b.Count - a.Count
	})
	stats.Top_ips = append([]ipCount(nil), stats.Top_ips[:10]...)

	results, err := os.OpenFile("./results.json", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644)
	if err != nil {
		log.Fatalln(err)
	}

	err = json.NewEncoder(results).Encode(stats)
	if err != nil {
		log.Fatalln(err)
	}
}

type ipCount struct {
	Ip    string `json:"ip"`
	Count int    `json:"count"`
}

type Stats struct {
	Total_requests      int            `json:"total_requests"`
	Skipped_lines       int            `json:"skipped_lines"`
	Errors_per_endpoint map[string]int `json:"errors_per_endpoint"`
	P95_latency_ms      map[string]int `json:"p95_latency_ms"`
	Top_ips             []ipCount      `json:"top_ips"`
}

type Log struct {
	timestamp string
	level     string
	ip        string
	path      string
	method    string
	status    int
	duration  int
}

func parseLog(line string) *Log {
	fields := strings.Fields(line)
	if len(fields) != 7 {
		return nil
	}

	status, err := strconv.Atoi(fields[5])
	if err != nil {
		log.Println(err)
	}

	duration, err := strconv.Atoi(strings.TrimSuffix(fields[6], "ms"))
	if err != nil {
		log.Println(err)
	}

	return &Log{
		timestamp: fields[0],
		level:     fields[1],
		ip:        fields[2],
		path:      fields[3],
		method:    fields[4],
		status:    status,
		duration:  duration,
	}
}
