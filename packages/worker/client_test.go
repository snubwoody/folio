package main

import (
	"bytes"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"github.com/stretchr/testify/assert"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestCreateIssue(t *testing.T) {
	assert := assert.New(t)
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var body struct {
			Title string `json:"title"`
			Body  string `json:"body"`
		}
		assert.Equal("/repos/snubwoody/folio/issues", r.URL.Path)
		err := json.NewDecoder(r.Body).Decode(&body)
		if err != nil {
			t.Error(err)
		}
		assert.Equal(body.Title, "[TITLE]")
		assert.Equal(body.Body, "Body")
		w.Write([]byte(`{"": ""}`))

	}))
	defer ts.Close()
	client := NewClient(ts.URL)
	err := client.CreateIssue("", "[TITLE]", "Body")
	assert.NoError(err)
}

func TestGetAccessToken(t *testing.T) {
	assert := assert.New(t)
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		url := fmt.Sprintf("/app/installations/%d/access_tokens", InstallationId)
		assert.Equal(url, r.URL.Path)
		w.Write([]byte(`{"": ""}`))

	}))
	defer ts.Close()
	block := &pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: make([]byte, 0),
	}
	buf := new(bytes.Buffer)
	err := pem.Encode(buf, block)
	assert.NoError(err)
	os.Setenv("WORKER_PRIVATE_KEY", buf.String())
	client := NewClient(ts.URL)
	token, err := client.GetAccessToken()
	//fmt.Println(token)
	fmt.Printf("Token: %s, Error: %s", token, err)
}

// CreateTestKey creates a rsa private key for testing.
func CreateTestKey() (string, error) {
	k, err := rsa.GenerateKey(rand.Reader, 2024)
	privateKeyDER := x509.MarshalPKCS1PrivateKey(k)
	block := &pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: privateKeyDER,
	}
	buf := new(bytes.Buffer)
	err = pem.Encode(buf, block)
	if err != nil {
		return "", err
	}
	return buf.String(), nil
}
func TestCreateJwt(t *testing.T) {
	assert := assert.New(t)
	key, err := CreateTestKey()
	assert.NoError(err)
	err = os.Setenv("WORKER_PRIVATE_KEY", key)
	assert.NoError(err)
	_, err = createJwt()
	assert.NoError(err)
}

//func TestSubmitBugReport(t *testing.T) {
//	assert := assert.New(t)
//	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
//		var body struct {
//			Title string `json:"title"`
//			Body  string `json:"body"`
//		}
//		err := json.NewDecoder(r.Body).Decode(&body)
//		if err != nil {
//			t.Error(err)
//		}
//		assert.Equal(body.Title, "[Bug report] App crashing")
//		//assert.Equal(body.Body, "Body")
//		w.Write([]byte(`{"": ""}`))
//	}))
//	defer ts.Close()
//	Client = NewClient(fmt.Sprintf("%s/repos/snubwoody/folio/issues", ts.URL))
//	br := BugReport{Title: "App crashing", Description: "A detailed description", Os: "Windows 11", Version: "v1.2.2"}
//	err := submitBugReport(br)
//	assert.NoError(err)
//}
