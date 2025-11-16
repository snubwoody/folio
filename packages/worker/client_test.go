package main

import (
	"encoding/json"
	"fmt"
	"github.com/stretchr/testify/assert"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestCreateIssue(t *testing.T) {
	assert := assert.New(t)
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var body struct {
			Title string `json:"title"`
			Body  string `json:"body"`
		}
		err := json.NewDecoder(r.Body).Decode(&body)
		if err != nil {
			t.Error(err)
		}
		assert.Equal(body.Title, "[TITLE]")
		assert.Equal(body.Body, "Body")
		w.Write([]byte(`{"": ""}`))

	}))
	defer ts.Close()
	client := NewClient(fmt.Sprintf("%s/repos/snubwoody/folio/issues", ts.URL))
	err := client.CreateIssue("", "[TITLE]", "Body")
	assert.NoError(err)
}
