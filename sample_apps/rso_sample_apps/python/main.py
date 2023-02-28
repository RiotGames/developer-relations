"""
FastAPI app for showing how to implement your own RSO client,
get an access token and make requests on behalf a user.
"""
import os

import requests
import uvicorn
from dotenv import load_dotenv
from fastapi import FastAPI
from fastapi.responses import HTMLResponse
from requests.auth import HTTPBasicAuth

load_dotenv('../.env')

app = FastAPI()

TIMEOUT = 10
CONFIG = {
    "RSO_BASE_URL": os.getenv("RSO_BASE_URL"),
    "RSO_CLIENT_ID": os.getenv("RSO_CLIENT_ID"),
    "RSO_CLIENT_SECRET": os.getenv("RSO_CLIENT_SECRET"),
    "APP_BASE_URL": os.getenv("APP_BASE_URL"),
    "APP_CALLBACK_PATH": os.getenv("APP_CALLBACK_PATH"),
    "CLIENT_ID": os.getenv("CLIENT_ID"),
    "RESPONSE_TYPE": os.getenv("RESPONSE_TYPE"),
    "SCOPE": os.getenv("SCOPE"),
    "RGAPI_TOKEN": os.getenv("RGAPI_TOKEN")
}
CONFIG['TOKEN_URL'] = f"{CONFIG['RSO_BASE_URL']}/token"
CONFIG["APP_CALLBACK_URL"] = f"{CONFIG['APP_BASE_URL']}{CONFIG['APP_CALLBACK_PATH']}"
CONFIG["AUTHORIZE_URL"] = f"{CONFIG['RSO_BASE_URL']}/authorize"

CONFIG["SIGN_IN_URL"] = f"{CONFIG['AUTHORIZE_URL']}"
CONFIG["SIGN_IN_URL"] += f"?redirect_uri={CONFIG['APP_CALLBACK_URL']}"
CONFIG["SIGN_IN_URL"] += f"&client_id={CONFIG['RSO_CLIENT_ID']}"
CONFIG["SIGN_IN_URL"] += f"&response_type={CONFIG['RESPONSE_TYPE']}"
CONFIG["SIGN_IN_URL"] += f"&scope={CONFIG['SCOPE']}"


@app.get("/", response_class=HTMLResponse)
def login():
    return f"""
    <h1>login</h1>
    <a href="{CONFIG["SIGN_IN_URL"]}">Sign In --> {CONFIG["SIGN_IN_URL"]}</a>
    """


@app.get(CONFIG["APP_CALLBACK_PATH"], response_class=HTMLResponse)
def oauth_callback(code: str):
    resp = requests.post(CONFIG['TOKEN_URL'],
                         auth=HTTPBasicAuth(CONFIG["RSO_CLIENT_ID"],
                                            CONFIG["RSO_CLIENT_SECRET"]),
                         data={
        "grant_type": "authorization_code",
        "code": code,
        "redirect_uri": CONFIG["APP_CALLBACK_URL"]
    },
        timeout=TIMEOUT
    )

    query_string = ""

    if resp.json() is not None:
        for k, v in resp.json().items():
            query_string += f'{k}={v}&'

    return f"""<script>window.location.href = "/show-data/?{query_string}";</script>"""


def json2table(json):
    style = """
            <style type="text/css">
        .tg  {border-collapse:collapse;border-spacing:0;}
        .tg td{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
        overflow:hidden;padding:10px 5px;word-break:normal;}
        .tg th{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
        font-weight:normal;overflow:hidden;padding:10px 5px;word-break:normal;}
        .tg .tg-0lax{text-align:left;vertical-align:top}
        </style>"""

    html = """
        <table class="tg">
        <thead>
        <tr>
            <th class="tg-0lax">key</th>
            <th class="tg-0lax">value</th>
        </tr>
        </thead>

        <tbody>"""

    for key, value in json.items():
        html += f"""
                <tr>
                    <td class="tg-0lax">{key}</td>
                    <td class="tg-0lax">{value}<br></td>
                </tr>"""

    html += """
    </tbody>
    </table>
        """

    return style + html


@app.get("/show-data/", response_class=HTMLResponse)
def show_data(access_token: str):
    """
    tBD
    """
    account_data = get_account_data(access_token)
    account_html = f"""
            <h2>account data queried using RSO Access Token:</h2>
            <p>{json2table(json=account_data)}</p>
        """

    champion_rotation_data = get_champion_rotation_data(CONFIG["RGAPI_TOKEN"])
    champion_rotation_html = f"""
            <h2>champion rotation data queried using RGAPI token</h2>
            <p>{json2table(json=champion_rotation_data)}</p>
        """

    return f"""
        {account_html}
        {champion_rotation_html}
    """


def get_account_data(access_token):
    resp = requests.get(
        "https://americas.api.riotgames.com/riot/account/v1/accounts/me",
        headers={
            "Authorization": f"Bearer {access_token}"
        },
        timeout=TIMEOUT
    )
    return resp.json()


def get_champion_rotation_data(token: str):
    resp = requests.get(
        "https://na1.api.riotgames.com/lol/platform/v3/champion-rotations",
        headers={
            "X-Riot-Token": token
        },
        timeout=TIMEOUT
    )
    return resp.json()


if __name__ == "__main__":
    uvicorn.run("main:app",
                host="localhost",
                port=3000,
                log_level="debug",
                reload=True
                )
