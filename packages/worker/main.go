// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"time"

	"github.com/golang-jwt/jwt/v5"
	"github.com/joho/godotenv"
)

const ClientId = "Iv23libGSoaUaiaWkC3D"

func main() {
	Client = NewClient(GithubApiEndpoint)
	err := godotenv.Load()
	if err != nil {
		fmt.Printf("Error loading .env: %s", err)
	}

	http.HandleFunc("GET /", getRoot)
	http.HandleFunc("POST /api/v1/feature", postFeature)
	http.HandleFunc("POST /api/v1/bug", postBug)

	fmt.Println("Listening for requests on port :8080")
	err = http.ListenAndServe(":8080", nil)
	if errors.Is(err, http.ErrServerClosed) {
		fmt.Println("Closing server")
		return
	}

	if err != nil {
		fmt.Printf("Error: %s\n", err)
		os.Exit(1)
	}
}

type BugReport struct {
	Title       string `json:"title"`
	Description string `json:"description,omitempty"`
	Os          string `json:"os,omitempty"`
	// The app version
	Version string `json:"version,omitempty"`
}

type FeatureRequest struct {
	Title       string `json:"title"`
	Description string `json:"description,omitempty"`
	Os          string `json:"os,omitempty"`
	// The app version
	Version string `json:"version,omitempty"`
}

type Response struct {
	// The http status code
	Status uint `json:"status"`
	// The response body, if the request succeeded.
	Body *ResponseBody `json:"body,omitempty"`
	// The response error, if the request failed.
	Error *ResponseError `json:"error,omitempty"`
}

type ResponseBody struct {
	IssueNumber uint   `json:"issue_number"`
	IssueUrl    string `json:"issue_url"`
}

type ResponseError struct {
	Details string `json:"details"`
}

// ToMarkdown converts the bug report into markdown
// for use in the github issue.
func (b *BugReport) ToMarkdown() string {
	s := fmt.Sprintf("App version: %s\n", b.Version)
	s += fmt.Sprintf("OS: %s\n\n", b.Os)
	s += "## Description\n"
	s += fmt.Sprintf("%s\n", b.Description)
	return s
}

// ToMarkdown converts the feature request into markdown
// for use in the github issue.
func (f *FeatureRequest) ToMarkdown() string {
	s := fmt.Sprintf("App version: %s\n", f.Version)
	s += fmt.Sprintf("OS: %s\n\n", f.Os)
	s += "## Description\n"
	s += fmt.Sprintf("%s\n", f.Description)
	return s
}

func postBug(w http.ResponseWriter, r *http.Request) {
	var body BugReport
	err := json.NewDecoder(r.Body).Decode(&body)
	if err != nil {
		w.WriteHeader(http.StatusUnprocessableEntity)
		io.WriteString(w, fmt.Sprintf("Invalid request body:%s", err))
		return
	}
	err = submitBugReport(body)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		fmt.Printf("Error %s\n", err)
		io.WriteString(w, "An unknown error occurred")
		return
	}
	w.WriteHeader(http.StatusCreated)
	fmt.Println("Created new bug report")
}

func postFeature(w http.ResponseWriter, r *http.Request) {
	var body FeatureRequest
	err := json.NewDecoder(r.Body).Decode(&body)
	if err != nil {
		w.WriteHeader(http.StatusUnprocessableEntity)
		io.WriteString(w, fmt.Sprintf("Invalid request body:%s", err))
		return
	}
	err = submitFeatureRequest(body)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		fmt.Printf("Error %s\n", err)
		io.WriteString(w, "An unknown error occurred")
		return
	}
	w.WriteHeader(http.StatusCreated)
	fmt.Println("Created new feature request")
}

func getRoot(w http.ResponseWriter, _ *http.Request) {
	io.WriteString(w, "Server is up and running")
}

// TODO: return response
func submitBugReport(report BugReport) error {
	token, err := getAccessToken()
	if err != nil {
		return err
	}
	title := fmt.Sprintf("[Bug report] %s", report.Title)
	err = Client.CreateIssue(token, title, report.ToMarkdown())
	if err != nil {
		return err
	}

	return nil
}

func submitFeatureRequest(request FeatureRequest) error {
	token, err := getAccessToken()
	if err != nil {
		return err
	}
	title := fmt.Sprintf("[Feature request] %s", request.Title)
	err = Client.CreateIssue(token, title, request.ToMarkdown())
	if err != nil {
		return err
	}
	return nil
}

type AccessTokenResponse struct {
	Token     string `json:"token"`
	ExpiresAt string `json:"expires_at"`
}

func getAccessToken() (string, error) {
	jwt, err := createJwt()
	if err != nil {
		return "", err
	}
	url := fmt.Sprintf("https://api.github.com/app/installations/%d/access_tokens", InstallationId)
	client := &http.Client{}
	r, err := http.NewRequest("POST", url, nil)
	if err != nil {
		return "", err
	}
	r.Header.Set("Accept", "application/vnd.github.raw+json")
	r.Header.Set("Authorization", fmt.Sprintf("Bearer %s", jwt))
	r.Header.Set("X-Github-Api-Version", fmt.Sprintf("%s", ApiVersion))

	resp, err := client.Do(r)
	if err != nil {
		return "", err
	}
	var responseBody AccessTokenResponse
	err = json.NewDecoder(resp.Body).Decode(&responseBody)
	if err != nil {
		return "", err
	}
	err = resp.Body.Close()
	if err != nil {
		return "", err
	}
	return responseBody.Token, nil
}

func createJwt() (string, error) {
	now := time.Now().Unix()
	privateKey := os.Getenv("WORKER_PRIVATE_KEY")
	claims := jwt.MapClaims{
		"iss": ClientId,
		"iat": now,
		"exp": now + 600, // Expiry of 10 minutes
	}
	token := jwt.NewWithClaims(jwt.SigningMethodRS256, claims)
	k, err := jwt.ParseRSAPrivateKeyFromPEM([]byte(privateKey))
	if err != nil {
		return "", err
	}
	s, err := token.SignedString(k)
	if err != nil {
		return "", err
	}
	return s, nil
}
