package main

import (
	"fmt"
	"net/http"
	"os"
	"strings"
	"testing"

	"github.com/joho/godotenv"
)

const TEST_PORT = "127.0.0.1:8080"

func TestMain(t *testing.T) {
	if err := godotenv.Load(); err != nil {
		fmt.Println("Unable to load .env:", err)
		return
	}

	var AUTH_KEY = os.Getenv("AuthKey")

	go main()

	const URL = "http://" + TEST_PORT

	t.Run("GET / OK", func(t *testing.T) {
		t.Parallel()

		resp, err := http.Get(URL)
		if err != nil {
			t.Fatal(err)
		}

		if resp.StatusCode != http.StatusOK {
			t.Log("Want: ", http.StatusOK, " Got: ", resp.StatusCode)
			t.Fail()
		}
	})

	t.Run("POST / No Auth", func(t *testing.T) {
		t.Parallel()

		resp, err := http.Post(URL, "application/text", strings.NewReader("This should not be emailed"))
		if err != nil {
			t.Fatal(err)
		}

		if resp.StatusCode != http.StatusUnauthorized {
			t.Log("Want: ", http.StatusUnauthorized, " Got: ", resp.StatusCode)
			t.Fail()
		}
	})

	t.Run("POST / No Body", func(t *testing.T) {
		t.Parallel()

		req, err := http.NewRequest(http.MethodPost, URL, nil)
		if err != nil {
			t.Fatal(err)
		}
		req.Header.Add("Authorization", AUTH_KEY)

		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			t.Fatal(err)
		}

		if resp.StatusCode != http.StatusBadRequest {
			t.Log("Want: ", http.StatusBadRequest, " Got: ", resp.StatusCode)
			t.Fail()
		}
	})

	t.Run("POST / OK", func(t *testing.T) {
		t.Parallel()

		req, err := http.NewRequest(http.MethodPost, URL, strings.NewReader("go-mail test run for POST /"))
		if err != nil {
			t.Fatal(err)
		}
		req.Header.Add("Authorization", AUTH_KEY)

		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			t.Fatal(err)
		}

		if resp.StatusCode != http.StatusOK {
			t.Log("Want: ", http.StatusOK, " Got: ", resp.StatusCode)
			t.Fail()
		}
	})

	t.Run("POST /custom OK", func(t *testing.T) {
		t.Parallel()

		body := `{"to":["mail@priteshtupe.com"],"subject":"Hello","message":"go-mail test run for POST /custom"}`

		req, err := http.NewRequest(http.MethodPost, URL+"/custom", strings.NewReader(body))
		if err != nil {
			t.Fatal(err)
		}
		req.Header.Add("Authorization", AUTH_KEY)

		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			t.Fatal(err)
		}

		if resp.StatusCode != http.StatusOK {
			t.Log("Want: ", http.StatusOK, " Got: ", resp.StatusCode)
			t.Fail()
		}
	})
}
