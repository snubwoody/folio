package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
)

// No trailing commas, it will break everything.

const GithubApiEndpoint = "https://api.github.com"
const InstallationId = 94393199
const ApiVersion = "2022-11-28"

var Client *ApiClient

// ApiClient is a client for interacting with github.
//
// Important endpoints:
//
// [POST /repos/{users}/{repo}/issues]
//
// [POST /repos/{users}/{repo}/issues]: https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#create-an-issue
type ApiClient struct {
    // The github api endpoint
    baseUrl string
    client  *http.Client
}

func NewClient(url string) *ApiClient {
    return &ApiClient{
        baseUrl: url,
        client:  &http.Client{},
    }
}

func (c *ApiClient) issueUrl() string {
    return fmt.Sprintf("%s/repos/snubwoody/folio/issues", c.baseUrl)
}

func (c *ApiClient) accessTokenUrl() string {
    return fmt.Sprintf("%s/app/installations/%d/access_tokens", c.baseUrl, InstallationId)
}

func (c *ApiClient) CreateIssue(accessToken, title, body string) error {
    b := map[string]any{
        "title": title,
        "body":  body,
    }
    jsonData, err := json.Marshal(b)
    if err != nil {
        return nil
    }
    r, err := http.NewRequest("POST", c.issueUrl(), bytes.NewBuffer(jsonData))
    if err != nil {
        return err
    }
    r.Header.Set("Accept", "application/vnd.github.raw+json")
    r.Header.Set("Authorization", fmt.Sprintf("Token %s", accessToken))
    r.Header.Set("X-Github-Api-Version", fmt.Sprintf("%s", ApiVersion))

    resp, err := c.client.Do(r)

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

func (c *ApiClient) GetAccessToken() (string, error) {
    jwt, err := createJwt()
    if err != nil {
        return "", err
    }
    client := &http.Client{}
    r, err := http.NewRequest("POST", c.accessTokenUrl(), nil)
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
