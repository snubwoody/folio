package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"time"

	"github.com/golang-jwt/jwt/v5"
	"github.com/joho/godotenv"
)

const installationId = 94393199
const apiVersion = "2022-11-28"

func main() {
	err := godotenv.Load()
	if err != nil {
		fmt.Printf("Error loading .env: %s", err)
	}
    
	// token,err := getAccessToken()
	// if err != nil {
	// 	fmt.Printf("Error loading .env: %s", err)
	// }
    // err = createIssue(token)
	// if err != nil {
	// 	fmt.Printf("Error loading .env: %s", err)
	// }

    http.HandleFunc("GET /",getRoot)
    err = http.ListenAndServe(":8080",nil)
    if err != nil{
        fmt.Printf("Error: %s\n",err)
        os.Exit(1)
    }
}

func getRoot(w http.ResponseWriter, r *http.Request){
    io.WriteString(w,"Server is up an running")
}

// TODO: make client?
func createIssue(accessToken string) error{
    url := "https://api.github.com/repos/snubwoody/folio/issues"
    client := &http.Client{}
    b := map[string]any{
        "title": "[Feature request]",
    }
    jsonData,err := json.Marshal(b)
    if err != nil{
        return nil
    }
	r, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
    if err != nil{
        return nil
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

    fmt.Printf("%s\n",responseBody)

    return nil
}

type AccessTokenResponse struct{
    Token string `json:"token"`
    ExpiresAt string `json:"expires_at"`
}

func getAccessToken() (string,error){
    jwt, err := createJwt()
	if err != nil {
        return "",err
	}
	url := fmt.Sprintf("https://api.github.com/app/installations/%d/access_tokens", installationId)
	client := &http.Client{}
	r, err := http.NewRequest("POST", url, nil)
	r.Header.Set("Accept", "application/vnd.github.raw+json")
	r.Header.Set("Authorization", fmt.Sprintf("Bearer %s", jwt))
	r.Header.Set("X-Github-Api-Version", fmt.Sprintf("%s", apiVersion))

	resp, err := client.Do(r)
	if err != nil {
        return "",err
	}
	var responseBody AccessTokenResponse
	err = json.NewDecoder(resp.Body).Decode(&responseBody)
	if err != nil {
        return "",err
	}
	err = resp.Body.Close()
	if err != nil {
		return "",err
	}
    return responseBody.Token,nil
}

func createJwt() (string, error) {
	now := time.Now().Unix()
	privateKey := os.Getenv("WORKER_PRIVATE_KEY")
	clientId := "Iv23libGSoaUaiaWkC3D"
	claims := jwt.MapClaims{
		"iss": clientId,
		"iat": now,
		"exp": now + 600, // Expiry of 10 minutes
	}
	token := jwt.NewWithClaims(jwt.SigningMethodRS256, claims)
	k, err := jwt.ParseRSAPrivateKeyFromPEM([]byte(privateKey))
	if err != nil {
		fmt.Printf("Error: %s", err)
		return "", nil
	}
	s, err := token.SignedString(k)
	if err != nil {
		return "", err
	}
	return s, nil
}
