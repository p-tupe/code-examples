// This is a simple mail server implementation that accepts an email body content on POST /
// or a custom (To,Subject,Message) on POST /custom route
// and sends it over via SMTP using go's builtin [net/smtp](https://pkg.go.dev/net/smtp) package.
package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"net/smtp"
	"os"
	"strings"

	_ "github.com/joho/godotenv/autoload"
)

// From .env file
var (
	Username = os.Getenv("Username")
	Password = os.Getenv("Password")
	Host     = os.Getenv("Host")
	Port     = os.Getenv("Port")
	From     = os.Getenv("From")
	To       = os.Getenv("To")
	AuthKey  = os.Getenv("AuthKey")

	auth smtp.Auth
)

func middleware(w http.ResponseWriter, r *http.Request) int {
	slog.Info("Request:", "method", r.Method, "host", r.Host, "path", r.URL.Path)

	key := r.Header.Get("Authorization")

	if r.Method != http.MethodGet && key != AuthKey {
		slog.Error("Invalid Authorization key: " + key)
		slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "unauthorized")
		return http.StatusUnauthorized
	}

	w.Header().Add("Access-Control-Allow-Origin", "*")
	w.Header().Add("Access-Control-Allow-Methods", "GET, POST, OPTIONS, HEAD")
	w.Header().Add("Access-Control-Allow-Headers", "Content-Type,Authorization")

	return 0
}

func allOtherRoutes(w http.ResponseWriter, r *http.Request) {
	if ok := middleware(w, r); ok != 0 {
		w.WriteHeader(ok)
		return
	}
	w.WriteHeader(http.StatusOK)
}

func simpleMail(w http.ResponseWriter, r *http.Request) {
	if ok := middleware(w, r); ok != 0 {
		w.WriteHeader(ok)
		return
	}

	body, err := io.ReadAll(r.Body)
	if err != nil {
		slog.Error("Unable to read request body: " + err.Error())
		slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "bad request")
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	if len(body) == 0 {
		slog.Error("Empty request body encountered!")
		slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "bad request")
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	msg := fmt.Appendf([]byte{}, "From: %s\r\nTo: %s\r\nSubject: Automated Email\r\n\r\n%s\r\n",
		From, To, string(body))

	err = smtp.SendMail(Host+":"+Port, auth, From, strings.Split(To, ","), msg)
	if err != nil {
		slog.Error("Unable to send email: " + err.Error())
		slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "server down")
		w.WriteHeader(http.StatusServiceUnavailable)
		return
	}

	slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "ok")
	w.WriteHeader(http.StatusOK)
}

func customMail(w http.ResponseWriter, r *http.Request) {
	if ok := middleware(w, r); ok != 0 {
		w.WriteHeader(ok)
		return
	}

	var body struct {
		To      []string `json:"to"`
		Subject string   `json:"subject"`
		Message string   `json:"message"`
	}

	err := json.NewDecoder(r.Body).Decode(&body)
	if err != nil {
		slog.Error("Unable to parse request body: " + err.Error())
		slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "bad request")
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	if body.Subject == "" || body.Message == "" || len(body.To) == 0 {
		slog.Error("Unable to find required data: " + err.Error())
		slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "bad request")
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	msg := fmt.Appendf([]byte{}, "From: %s\r\nTo: %s\r\nSubject: %s\r\n\r\n%s\r\n",
		From, strings.Join(body.To, ","), body.Subject, body.Message)

	err = smtp.SendMail(Host+":"+Port, auth, From, body.To, msg)
	if err != nil {
		slog.Error("Unable to send email: " + err.Error())
		slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "server down")
		w.WriteHeader(http.StatusServiceUnavailable)
		return
	}

	slog.Info("Response:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "status", "server down")
	w.WriteHeader(http.StatusOK)
}

func main() {
	if Username == "" || Password == "" || Host == "" || Port == "" || From == "" || To == "" {
		slog.Error("Required variables not found!")
		os.Exit(1)
	}

	auth = smtp.PlainAuth("", Username, Password, Host)

	http.HandleFunc("/", allOtherRoutes)
	http.HandleFunc("POST /", simpleMail)
	http.HandleFunc("POST /custom", customMail)

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	slog.Info("Mail Server listening on :" + port)
	if err := http.ListenAndServe(":"+port, nil); err != nil {
		slog.Error(err.Error())
	}
}
