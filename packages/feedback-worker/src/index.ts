import { App } from "octokit";
import dotenv from "dotenv";
import jwt from "jsonwebtoken";

console.log("Hello world");

dotenv.config();

// TODO: add logging library

const appId = "2280940";
const clientId = "Iv23libGSoaUaiaWkC3D";
const installationId = 94393199;
const privateKey = process.env.WORKER_PRIVATE_KEY ?? "";

// const app = new App({
//     appId,
//     privateKey: workerKey
// });

// const octokit = app.getInstallationOctokit(installationId);
// const now = Date.now();

const now = Math.floor(Date.now() / 1000);
const payload = {
    iat: now,
    exp: now + 600,
    iss: clientId,
};

const jwtToken = jwt.sign(payload,privateKey,{algorithm: "RS256"});
const apiVersion = "2022-11-28";

const url = `https://api.github.com/app/installations/${installationId}/access_tokens`;
const tokenResponse = await fetch(url,{
    method: "POST",
    headers:{
        "Accept": "application/vnd.github.raw+json",
        "Authorization": `Bearer ${jwtToken}`,
        "X-Github-Api-Version": apiVersion
    },
    body:JSON.stringify({title: "[Bug report] App crashing"})
});

const json = await tokenResponse.json();
const token = json.token;
console.log(json);

const body = await fetch("https://api.github.com/repos/snubwoody/folio/issues",{
    method: "POST",
    headers:{
        accept: "application/vnd.github.raw+json",
        "Authorization": `Token ${token}`
    },
    body:JSON.stringify({title: "[Bug report] App crashing"})
});

const response = await body.text();
console.log(response);