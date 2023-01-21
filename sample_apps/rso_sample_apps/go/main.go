package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strings"

	"github.com/joho/godotenv"
)

var CONFIG map[string]string

func loadConfig() {
	wd, err := os.Getwd()
	if err != nil {
		panic(err)
	}
	parent := filepath.Dir(wd)

	err = godotenv.Load(filepath.Join(parent, ".env"))
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	CONFIG = make(map[string]string)

	CONFIG["RSO_BASE_URL"] = os.Getenv("RSO_BASE_URL")
	CONFIG["RSO_CLIENT_ID"] = os.Getenv("RSO_CLIENT_ID")
	CONFIG["RSO_CLIENT_SECRET"] = os.Getenv("RSO_CLIENT_SECRET")
	CONFIG["APP_BASE_URL"] = os.Getenv("APP_BASE_URL")
	CONFIG["APP_CALLBACK_PATH"] = os.Getenv("APP_CALLBACK_PATH")
	CONFIG["CLIENT_ID"] = os.Getenv("CLIENT_ID")
	CONFIG["RESPONSE_TYPE"] = os.Getenv("RESPONSE_TYPE")
	CONFIG["SCOPE"] = os.Getenv("SCOPE")
	CONFIG["RGAPI_TOKEN"] = os.Getenv("RGAPI_TOKEN")

	CONFIG["TOKEN_URL"] = fmt.Sprintf("%s/token", CONFIG["RSO_BASE_URL"])
	CONFIG["APP_CALLBACK_URL"] = fmt.Sprintf("%s%s", CONFIG["APP_BASE_URL"], CONFIG["APP_CALLBACK_PATH"])
	CONFIG["AUTHORIZE_URL"] = fmt.Sprintf("%s/authorize", CONFIG["RSO_BASE_URL"])

	CONFIG["SIGN_IN_URL"] = CONFIG["AUTHORIZE_URL"]
	CONFIG["SIGN_IN_URL"] += fmt.Sprintf("?redirect_uri=%s", CONFIG["APP_CALLBACK_URL"])
	CONFIG["SIGN_IN_URL"] += fmt.Sprintf("&client_id=%s", CONFIG["RSO_CLIENT_ID"])
	CONFIG["SIGN_IN_URL"] += fmt.Sprintf("&response_type=%s", CONFIG["RESPONSE_TYPE"])
	CONFIG["SIGN_IN_URL"] += fmt.Sprintf("&scope=%s", CONFIG["SCOPE"])
}

func login(w http.ResponseWriter, req *http.Request) {
	fmt.Println("login")
	html := fmt.Sprintf(
		"<h1>login</h1><a href=\"%s\">Sign In --> %s</a>",
		CONFIG["SIGN_IN_URL"], CONFIG["SIGN_IN_URL"])

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.Write([]byte(html))
}

func oauthCallback(w http.ResponseWriter, req *http.Request) {
	type TokenURLResponse struct {
		AccessToken  string `json:"access_token"`
		RefreshToken string `json:"refresh_token"`
		Scope        string `json:"scope"`
		IDToken      string `json:"id_token"`
		TokenType    string `json:"token_type"`
		ExpiresIn    int    `json:"expires_in"`
	}

	code := req.URL.Query().Get("code")

	form := url.Values{}
	form.Add("grant_type", "authorization_code")
	form.Add("code", code)
	form.Add("redirect_uri", CONFIG["APP_CALLBACK_URL"])

	postReq, _ := http.NewRequest(
		"POST",
		CONFIG["TOKEN_URL"],
		strings.NewReader(form.Encode()),
	)
	postReq.SetBasicAuth(CONFIG["RSO_CLIENT_ID"], CONFIG["RSO_CLIENT_SECRET"])
	postReq.Header.Set("Content-Type", "application/x-www-form-urlencoded")

	postRes, _ := http.DefaultClient.Do(postReq)
	defer postRes.Body.Close()

	body, _ := ioutil.ReadAll(postRes.Body)

	var tokenURLResponse TokenURLResponse
	json.Unmarshal(body, &tokenURLResponse)

	queryString := fmt.Sprintf("access_token=%s", tokenURLResponse.AccessToken)

	html := fmt.Sprintf(
		"<script>window.location.href=\"/show-data/?%s\";</script>",
		queryString)
	w.Write([]byte(html))
}

func showData(w http.ResponseWriter, req *http.Request) {
	accessToken := req.URL.Query().Get("access_token")

	accountData := getAccountData(accessToken)
	accountHTML := fmt.Sprintf(
		"<h2>account data queried using RSO Access Token:</h2><p>%v</p>",
		accountData)

	championRotationData := getChampionRotationData(CONFIG["RGAPI_TOKEN"])
	championRotationHTML := fmt.Sprintf(
		"<h2>champion rotation data queried using RGAPI token</h2><p>%v</p>",
		championRotationData)

	html := fmt.Sprintf("%s %s", accountHTML, championRotationHTML)
	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.Write([]byte(html))
}

func getAccountData(accessToken string) string {
	type AccountData struct {
		Puuid    string `html:"l=Puuid,e=span,c=puuid"`
		GameName string `html:"l=GameName,e=span,c=gamename"`
		TagLine  string `html:"l=TagLine,e=span,c=tagline"`
	}

	req, _ := http.NewRequest(
		"GET",
		"https://americas.api.riotgames.com/riot/account/v1/accounts/me",
		nil,
	)
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", accessToken))
	res, _ := http.DefaultClient.Do(req)
	defer res.Body.Close()

	body, _ := ioutil.ReadAll(res.Body)

	var accountData AccountData
	json.Unmarshal(body, &accountData)

	html, _ := structToHTML(map[string]string{
		"puuid":    fmt.Sprint(accountData.Puuid),
		"gameName": fmt.Sprint(accountData.GameName),
		"tagLine":  fmt.Sprint(accountData.TagLine),
	})

	return html
}

func getChampionRotationData(token string) string {
	type ChampionRotationData struct {
		FreeChampionIds              []int
		FreeChampionIdsForNewPlayers []int
		MaxNewPlayerLevel            int
	}

	req, _ := http.NewRequest(
		"GET",
		"https://na1.api.riotgames.com/lol/platform/v3/champion-rotations",
		nil,
	)
	req.Header.Add("X-Riot-Token", token)
	res, _ := http.DefaultClient.Do(req)
	defer res.Body.Close()

	body, _ := ioutil.ReadAll(res.Body)

	var championRotationData ChampionRotationData
	json.Unmarshal(body, &championRotationData)

	html, _ := structToHTML(map[string]string{
		"freeChampionIds":              fmt.Sprint(championRotationData.FreeChampionIds),
		"freeChampionIdsForNewPlayers": fmt.Sprint(championRotationData.FreeChampionIdsForNewPlayers),
		"maxNewPlayerLevel":            fmt.Sprint(championRotationData.MaxNewPlayerLevel),
	})

	return html
}

func structToHTML(data map[string]string) (string, error) {
	style := `
	<style type="text/css">
.tg  {border-collapse:collapse;border-spacing:0;}
.tg td{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
overflow:hidden;padding:10px 5px;word-break:normal;}
.tg th{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
font-weight:normal;overflow:hidden;padding:10px 5px;word-break:normal;}
.tg .tg-0lax{text-align:left;vertical-align:top}
</style>`
	html := `
	<table class="tg">
	<thead>
	<tr>
		<th class="tg-0lax">key</th>
		<th class="tg-0lax">value</th>
	</tr>
	</thead>

	<tbody>`

	for key, value := range data {
		html += fmt.Sprintf(`
		<tr>
			<td class="tg-0lax">%s</td>
			<td class="tg-0lax">%s<br></td>
		</tr>`, key, value)
	}

	html += `
    </tbody>
    </table>
        `

	return style + html, nil
}

func main() {
	loadConfig()

	http.HandleFunc("/", login)
	http.HandleFunc(CONFIG["APP_CALLBACK_PATH"], oauthCallback)
	http.HandleFunc("/show-data/", showData)

	http.ListenAndServe(":3000", nil)
}
