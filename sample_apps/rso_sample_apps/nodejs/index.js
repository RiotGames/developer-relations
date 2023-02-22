import dotenv from 'dotenv';
import express from 'express';
import fetch from 'node-fetch';
import url from 'url';

const app = express();
const port = 3000;

dotenv.config({ path: '../.env' });

const TIMEOUT = 10
var CONFIG = {
    RSO_BASE_URL: process.env.RSO_BASE_URL,
    RSO_CLIENT_ID: process.env.RSO_CLIENT_ID,
    RSO_CLIENT_SECRET: process.env.RSO_CLIENT_SECRET,
    APP_BASE_URL: process.env.APP_BASE_URL,
    APP_CALLBACK_PATH: process.env.APP_CALLBACK_PATH,
    CLIENT_ID: process.env.CLIENT_ID,
    RESPONSE_TYPE: process.env.RESPONSE_TYPE,
    SCOPE: process.env.SCOPE,
    RGAPI_TOKEN: process.env.RGAPI_TOKEN
}

CONFIG['TOKEN_URL'] = `${CONFIG.RSO_BASE_URL}/token`
CONFIG['APP_CALLBACK_URL'] = `${CONFIG.APP_BASE_URL}${CONFIG.APP_CALLBACK_PATH}`
CONFIG['AUTHORIZE_URL'] = `${CONFIG.RSO_BASE_URL}/authorize`

CONFIG['SIGN_IN_URL'] = `${CONFIG.AUTHORIZE_URL}`
CONFIG['SIGN_IN_URL'] += `?redirect_uri=${CONFIG.APP_CALLBACK_URL}`
CONFIG['SIGN_IN_URL'] += `&client_id=${CONFIG.RSO_CLIENT_ID}`
CONFIG['SIGN_IN_URL'] += `&response_type=${CONFIG.RESPONSE_TYPE}`
CONFIG['SIGN_IN_URL'] += `&scope=${CONFIG.SCOPE} `

app.get('/', function (req, res) {
    res.send(`
    <h1>login</h1>
    <a href="${CONFIG.SIGN_IN_URL}" > Sign In-- > ${CONFIG.SIGN_IN_URL}</a> `
    );
});

app.get(CONFIG.APP_CALLBACK_PATH, async function (req, res) {
    var code = req.query.code;

    const params = new URLSearchParams();
    params.append("grant_type", "authorization_code");
    params.append("code", code);
    params.append("redirect_uri", CONFIG.APP_CALLBACK_URL);

    const response = await fetch(CONFIG.TOKEN_URL, {
        method: "POST",
        headers: {
            Authorization: 'Basic ' + Buffer.from(CONFIG.RSO_CLIENT_ID + ":" + CONFIG.RSO_CLIENT_SECRET).toString('base64')
        },
        body: params
    }
    );

    const data = await response.json();
    var query_string = new URLSearchParams(data).toString();
    res.send(`<script>window.location.href = "/show-data/?${query_string}?";</script>`);
});

app.get("/show-data", async function (req, res) {
    const access_token = url.parse(req.url, true).query.access_token;

    var account_data = await get_account_data(access_token);
    var account_html = `
        <h2>account data queried using RSO Access Token:</h2>
        <p>${json2table(account_data)}</p>
    `

    var champion_rotation_data = await get_champion_rotation_data(CONFIG.RGAPI_TOKEN);
    var champion_rotation_html = `
        <h2>champion rotation data queried using RGAPI token </h2>
        <p>${json2table(champion_rotation_data)}</p>
    `
    res.send(`
        ${account_html}
        ${champion_rotation_html}
    `)
});

async function get_champion_rotation_data(token) {
    const response = await fetch('https://na1.api.riotgames.com/lol/platform/v3/champion-rotations',
        {
            method: "GET",
            headers: {
                "X-Riot-Token": token
            }

        });
    const data = await response.json();
    return data;
}

async function get_account_data(access_token) {
    const response = await fetch('https://americas.api.riotgames.com/riot/account/v1/accounts/me',
        {
            method: "GET",
            headers: {
                Authorization: `Bearer ${access_token}`
            }

        });
    const data = await response.json();
    return data;
}

function json2table(json) {
    const style = `
            <style type="text/css">
        .tg  {border-collapse:collapse;border-spacing:0;}
        .tg td{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
        overflow:hidden;padding:10px 5px;word-break:normal;}
        .tg th{border-color:black;border-style:solid;border-width:1px;font-family:Arial, sans-serif;font-size:14px;
        font-weight:normal;overflow:hidden;padding:10px 5px;word-break:normal;}
        .tg .tg-0lax{text-align:left;vertical-align:top}
        </style>`;

    var html = `
        <table class="tg">
        <thead>
        <tr>
            <th class="tg-0lax">key</th>
            <th class="tg-0lax">value</th>
        </tr>
        </thead>

        <tbody>`;

    for (const [key, value] of Object.entries(json)) {
        html += `
                <tr>
                    <td class="tg-0lax">${key}</td>
                    <td class="tg-0lax">${value}<br></td>
                </tr>`
    }

    html += `
    </tbody>
    </table>
        `

    return style + html;

}
app.listen(port, () => console.log(`Example app listening on post ${port}!`));
