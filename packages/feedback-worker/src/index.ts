import { App } from "octokit";
import dotenv from "dotenv";
import jwt from "jsonwebtoken";

dotenv.config();

// TODO: add logging library

const appId = "2280940";
const clientId = "Iv23libGSoaUaiaWkC3D";
const apiVersion = "2022-11-28";
const installationId = 94393199;
const privateKey = process.env.WORKER_PRIVATE_KEY ?? "";

const createJwt = (clientId: string,privateKey: string): string => {
    const now = Math.floor(Date.now() / 1000);
    const payload = {
        iat: now,
        exp: now + 600,
        iss: clientId
    };

    return jwt.sign(payload,privateKey,{algorithm: "RS256"});
};

const jwtToken = createJwt(clientId,privateKey);

interface AccessTokenResponse{
    token: string,
    expires_at: string,
} 

/**
 * Request an installation access token.
 * 
 * [Api docs](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-an-installation-access-token-for-an-app) 
 */
export const getAccessToken = async(installationId: number, jwtToken: string): Promise<string> => {
    const url = `https://api.github.com/app/installations/${installationId}/access_tokens`;
    const tokenResponse = await fetch(url,{
        method: "POST",
        headers:{
            "Accept": "application/vnd.github.raw+json",
            "Authorization": `Bearer ${jwtToken}`,
            "X-Github-Api-Version": apiVersion
        }
    });
    const json = await tokenResponse.json() as AccessTokenResponse;
    return json.token;
};

const accessToken = await getAccessToken(installationId,jwtToken);



interface CreateIssueBody{
    title: string,
    body?: string,
    labels?: string[]
}

interface CreateIssueResponse{
    /** The url of the newly created issue. */
    url: string,
    repository_url: string,
    id: number, 
    title: string,
    body: string
}

export const createIssue = async(accessToken: string) => {
    const body = await fetch("https://api.github.com/repos/snubwoody/folio/issues",{
        method: "POST",
        headers:{
            accept: "application/vnd.github.raw+json",
            "Authorization": `Token ${accessToken}`
        },
        body:JSON.stringify({title: "[Bug report] App crashing"})
    });
    
    const response = await body.json() as CreateIssueResponse;
    console.log(response);
};
