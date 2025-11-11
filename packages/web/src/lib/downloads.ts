
export type ReleaseInfo = {
    id: number,
    tag_name: string,
    name: string,
    author: ReleaseAuthor,
    assets: Asset[]
    published_at: string
    created_at: string,
    updated_at: string,
    draft: boolean,
    prerelease: boolean,
    immutable: boolean,
};

export type ReleaseAuthor = {
    login: string,
};

export type Asset = {
    url: string,
    id: number,
    name: string,
    content_type: string,
    size: number,
    digest: string,
    browser_download_url: string
};

export type DownloadLinks = {
    dmg: string
    deb: string
    exe: string
};

const githubAccessToken = import.meta.env.PERSONAL_ACCESS_TOKEN;

/**
 * Fetches the latest Github release.
 */
export const getLatestRelease = async(): Promise<ReleaseInfo> => {
    const url = "https://api.github.com/repos/snubwoody/folio/releases/latest";
    const response  = await fetch(url,{
        headers: {
            "Authorization": `Token ${githubAccessToken}`
        }
    });
    return await response.json() as ReleaseInfo;
};

export const getDownloadLinks = async (): Promise<DownloadLinks> => {
    const release = await getLatestRelease();
    const map = new Map<string,Asset>();
    for (const asset of release.assets) {
        if (asset.name.includes(".dmg")) {
            map.set("dmg",asset);
        }

        if (asset.name.includes(".exe")) {
            map.set("exe",asset);
        }

        if (asset.name.includes(".deb")) {
            map.set("deb",asset);
        }
    }

    return {
        exe: map.get("exe")!.browser_download_url,
        dmg: map.get("dmg")!.browser_download_url,
        deb: map.get("deb")!.browser_download_url
    };
};