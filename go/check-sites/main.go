// package check-sites is a health monitoring tool
// that pings a list of sites (urls) from config.json
// and ensures they're up.
//
// USAGE: check-sites <config>.json
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"os/exec"
	"strings"
	"time"
)

type Config struct {
	Interval time.Duration `json:"interval"`
	Sites    []string      `json:"sites"`
}

var notify, exists = os.LookupEnv("NOTIFY")

func notifyAndLog(msg string) {
	slog.Info(msg)

	if exists {
		if err := exec.Command(notify, msg).Run(); err != nil {
			slog.Error(err.Error())
		}
	}
}

func main() {
	notifyAndLog("Pingmon started...")

	if len(os.Args) < 2 {
		slog.Error("no config ")
		return
	}

	cfg, err := readConfig(os.Args[1])
	if err != nil {
		slog.Error("Error while reading config", "err", err)
		return
	}

	slog.Info("Added", "sites", cfg.Sites)
	slog.Info("For", "interval", cfg.Interval*time.Minute)

	tick := time.Tick(cfg.Interval * time.Minute)
	for {
		<-tick
		for _, site := range cfg.Sites {
			go check(site)
		}
	}
}

func readConfig(path string) (Config, error) {
	var cfg Config

	f, err := os.Open(path)
	if err != nil {
		return cfg, err
	}
	defer f.Close()

	err = json.NewDecoder(f).Decode(&cfg)
	if err != nil {
		return cfg, err
	}

	if len(cfg.Sites) == 0 {
		return cfg, errors.New("config.sites invalid")
	}

	if cfg.Interval == 0 {
		return cfg, errors.New("config.interval invalid")
	}

	return cfg, nil
}

// check starts a ticker that checks for a
// successful response from `site` every 30m
func check(site string) {
	resp, err := http.Get(site)
	if err != nil {
		msg := fmt.Sprintf("Error while checking site %s: %v\n", site, err)
		notifyAndLog(msg)
		alert(msg)
		return
	}

	if resp.StatusCode > http.StatusMultipleChoices {
		msg := fmt.Sprintf("Error while checking site %s: %v\n", site, resp.Status)
		notifyAndLog(msg)
		alert(msg)
		return
	}

	slog.Info(site, ":", resp.Status)
}

// alert sends a `msg` to the email
func alert(msg string) {
	slog.Info("Generating alert...")

	req, err := http.NewRequest(http.MethodPost, "https://app.priteshtupe.com/mail", strings.NewReader(msg))
	if err != nil {
		slog.Error("Error while creating alert", "err", err)
		return
	}
	req.Header.Add("Authorization", "+ugwY4jSfRC2dx3RwPYR7dDwFT0ilK42TrhOUGpjqOA4Cg==")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		slog.Error("Error while sending alert", "err", err)
		return
	}

	if resp.StatusCode != http.StatusOK {
		slog.Error("Error while sending alert", "status", resp.Status)
		return
	}

	slog.Info("Alert sent!")
}
