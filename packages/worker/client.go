package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
)

const IssueUrl = "https://api.github.com/repos/snubwoody/folio/issues"

var Client *ApiClient

// ApiClient is a client for interacting with github.
//
// Important endpoints:
//
// [POST /repos/{users}/{repo}/issues]
//
// [POST /repos/{users}/{repo}/issues]: https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#create-an-issue
type ApiClient struct {
	// The github api endpoint for creating issues
	url    string
	client *http.Client
}

func NewClient(url string) *ApiClient {
	return &ApiClient{
		url:    url,
		client: &http.Client{},
	}
}

func (c *ApiClient) CreateIssue(accessToken, title, body string) error {
	client := &http.Client{}
	b := map[string]any{
		"title": title,
		"body":  body,
	}
	jsonData, err := json.Marshal(b)
	if err != nil {
		return nil
	}
	r, err := http.NewRequest("POST", c.url, bytes.NewBuffer(jsonData))
	if err != nil {
		return err
	}
	r.Header.Set("Accept", "application/vnd.github.raw+json")
	r.Header.Set("Authorization", fmt.Sprintf("Token %s", accessToken))
	r.Header.Set("X-Github-Api-Version", fmt.Sprintf("%s", apiVersion))

	resp, err := client.Do(r)

	var responseBody map[string]any

	err = json.NewDecoder(resp.Body).Decode(&responseBody)
	if err != nil {
		return err
	}

	err = resp.Body.Close()
	if err != nil {
		return err
	}
	return nil
}
