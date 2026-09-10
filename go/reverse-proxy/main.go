// `reverse-proxy` takes in http requests at PORT
// and forwards them to another server.
// Think of it as baby-caddy.
//
// USAGE:
//
//	./reverse-proxy <config>.json
//
// SAMPLE CONFIG:
//
//	{
//		"port": 3000,
//		"paths": [ { "from": "/frontend-path", "to": "http://localhost:port" } ]
//	}
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"strconv"
	"strings"
)

type path struct {
	From string `json:"from"`
	To   string `json:"to"`
}

type config struct {
	Port  int    `json:"port"`
	Paths []path `json:"paths"`
}

type pathMap = [][2]string

func main() {
	config, err := getConfig()
	if err != nil {
		slog.Error("error while loading config: ", "err", err)
		return
	}

	pathMap, err := getPathMap(&config)
	if err != nil {
		slog.Error("error while making path map: ", "err", err)
		return
	}

	mux := http.NewServeMux()

	// internal
	mux.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		slog.Info("request received:", "from", r.URL.Path)
		w.WriteHeader(200)
	})

	proxy := &httputil.ReverseProxy{
		ErrorLog: slog.NewLogLogger(slog.Default().Handler(), slog.Level(slog.LevelError)),
		Rewrite: func(pr *httputil.ProxyRequest) {
			slog.Info("proxying request received:", "from", pr.In.URL)

			for _, path := range pathMap {
				if p, ok := strings.CutPrefix(pr.In.URL.Path, path[0]); ok {
					if p == "" {
						p = "/"
					}
					if !strings.HasPrefix(p, "/") {
						p = "/" + p
					}
					pr.Out.URL.Path = p
					pr.Out.URL.RawPath = ""
					if target, err := url.Parse(path[1]); err != nil {
						slog.Error("bad target url", "err", err)
					} else {
						pr.SetURL(target)
						slog.Info("proxied", "from", pr.In.URL.Path, "to", pr.Out.URL.String())
					}
					break
				}
			}
		},
	}

	mux.Handle("/", proxy)

	slog.Info("starting reverse-proxy on", "port", config.Port)
	if err := http.ListenAndServe(":"+strconv.Itoa(config.Port), mux); err != nil {
		slog.Error("error starting reverse-proxy", "err", err)
		return
	}
}

func getConfig() (config, error) {
	var c config

	if len(os.Args) < 2 || os.Args[1] == "" {
		return c, errors.New("config file path is empty. Usage: ./reverse-proxy <config>.json")
	}

	configFile, err := os.Open(os.Args[1])
	if err != nil {
		return c, fmt.Errorf("cannot open config file: %w", err)
	}

	err = json.NewDecoder(configFile).Decode(&c)
	if err != nil {
		return c, fmt.Errorf("cannot parse config file: %w", err)
	}

	return c, nil
}

func getPathMap(config *config) (pathMap, error) {
	var pm pathMap

	for _, p := range config.Paths {
		if _, err := url.Parse(p.From); err != nil {
			return nil, fmt.Errorf("cannot parse 'from' url: %w", err)
		} else if _, err := url.Parse(p.From); err != nil {
			return nil, fmt.Errorf("cannot parse 'to' url: %w", err)
		} else {
			pm = append(pm, [2]string{p.From, p.To})
		}
	}

	return pm, nil
}
