
const downloadRepo = "https://github.com/snubwoody/folio-site";

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
}

export type ReleaseAuthor = {
    login: string,
}

export type Asset = {
    url: string,
    id: number,
    name: string,
    content_type: string,
    size: number,
    digest: string,
    browser_download_url: string
}

export type DownloadLinks = {
    dmg: string
    rpm: string
    appImage: string
    deb: string
    exe: string
    msi: string
};

export const getLatestRelease = async(): Promise<ReleaseInfo> => {
    const url = "https://api.github.com/repos/snubwoody/folio-site/releases/latest";
    const response  = await fetch(url);
    return await response.json() as ReleaseInfo;
}

export const getDownloadLinks = async (): Promise<DownloadLinks> => {
    const release = await getLatestRelease();
    let map = new Map<string,Asset>();
    for (const asset of release.assets) {
        if (asset.name.includes(".msi")) {
            map.set("msi",asset);
        }

        if (asset.name.includes(".dmg")) {
            map.set("dmg",asset);
        }

        if (asset.name.includes(".exe")) {
            map.set("exe",asset);
        }

        if (asset.name.includes(".AppImage")) {
            map.set("appImage",asset);
        }

        if (asset.name.includes(".rpm")) {
            map.set("rpm",asset);
        }
        if (asset.name.includes(".deb")) {
            map.set("deb",asset);
        }
    }

    const links: DownloadLinks = {
        msi: map.get("msi")!.browser_download_url,
        exe: map.get("exe")!.browser_download_url,
        appImage: map.get("appImage")!.browser_download_url,
        dmg: map.get("dmg")!.browser_download_url,
        deb: map.get("deb")!.browser_download_url,
        rpm: map.get("rpm")!.browser_download_url,
    };

    return links;
}