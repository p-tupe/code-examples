// This is a simple mail server implementation that accepts an email body content on POST /
// in form of a custom (To,Subject,Message) json body or simple plain text,
// and sends it over via SMTP using go's builtin [net/smtp](https://pkg.go.dev/net/smtp) package.
package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"mime"
	"net/http"
	"net/smtp"
	"os"
	"strings"

	"github.com/joho/godotenv"
)

var (
	username string
	password string
	host     string
	port     string
	from     string
	to       string // optional if using json body
	authKey  string

	auth smtp.Auth
)

type emailBody struct {
	To      []string `json:"to"`
	Subject string   `json:"subject"`
	Message string   `json:"message"`
}

func middleware(w http.ResponseWriter, r *http.Request) int {
	logR(r, "pending")

	if r.Method == http.MethodPost && r.Header.Get("Authorization") != authKey {
		return http.StatusUnauthorized
	}

	w.Header().Add("Access-Control-Allow-Origin", "*")
	w.Header().Add("Access-Control-Allow-Methods", "GET, POST, OPTIONS, HEAD")
	w.Header().Add("Access-Control-Allow-Headers", "Content-Type,Authorization")

	return 0
}

func allOtherRoutes(w http.ResponseWriter, r *http.Request) {
	if ok := middleware(w, r); ok != 0 {
		logR(r, "unauthorized")
		w.WriteHeader(ok)
		return
	}
	logR(r, "ok")
	w.WriteHeader(http.StatusOK)
}

// sendMail is a Post request handler on / that
// accepts either a plain text body or a json of form [emailBody]
// and sends the parse body as email.
func sendMail(w http.ResponseWriter, r *http.Request) {
	if ok := middleware(w, r); ok != 0 {
		logR(r, "unauthorized")
		w.WriteHeader(ok)
		return
	}

	data, err := io.ReadAll(r.Body)
	if err != nil {
		logR(r, err.Error())
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	if len(data) == 0 {
		logR(r, "empty body")
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var msg []byte

	if r.Header.Get("Content-Type") == mime.TypeByExtension(".json") {
		var body emailBody
		err = json.Unmarshal(data, &body)
		if err != nil {
			logR(r, err.Error())
			w.WriteHeader(http.StatusBadRequest)
			return
		}

		if body.Subject == "" || body.Message == "" || len(body.To) == 0 {
			logR(r, "invalid json keys")
			w.WriteHeader(http.StatusBadRequest)
			return
		}

		msg = fmt.Appendf([]byte{}, "From: %s\r\nTo: %s\r\nSubject: %s\r\n\r\n%s\r\n",
			from, strings.Join(body.To, ","), body.Subject, body.Message)

		err = smtp.SendMail(host+":"+port, auth, from, body.To, msg)
	} else {
		msg = fmt.Appendf([]byte{}, "From: %s\r\nTo: %s\r\nSubject: Automated Email\r\n\r\n%s\r\n",
			from, to, string(data))

		err = smtp.SendMail(host+":"+port, auth, from, strings.Split(to, ","), msg)
	}

	if err != nil {
		logR(r, err.Error())
		w.WriteHeader(http.StatusServiceUnavailable)
		return
	}

	logR(r, "ok")
	w.WriteHeader(http.StatusOK)
}

func main() {
	if err := godotenv.Load(); err != nil {
		fmt.Println("Unable to load .env:", err)
		return
	}

	username = os.Getenv("Username")
	password = os.Getenv("Password")
	host = os.Getenv("Host")
	port = os.Getenv("Port")
	from = os.Getenv("From")
	to = os.Getenv("To")
	authKey = os.Getenv("AuthKey")

	if username == "" || password == "" || host == "" || port == "" || from == "" || authKey == "" {
		slog.Error("Required variables not found, please update .env file.")
		os.Exit(1)
	}

	auth = smtp.PlainAuth("", username, password, host)

	http.HandleFunc("/", allOtherRoutes)
	http.HandleFunc("POST /", sendMail)

	port := os.Getenv("ServerPort")
	if port == "" {
		port = "8080"
	}

	slog.Info("Mail Server listening on :" + port)
	if err := http.ListenAndServe(":"+port, nil); err != nil {
		slog.Error(err.Error())
	}
}

func logR(r *http.Request, msg string) {
	slog.Info("Request:", "method", r.Method, "host", r.Host, "path", r.URL.Path, "resp", msg)
}
