
import { getDownloadLinks } from "../lib/downloads.ts";
import { test,expect } from "vitest";

test("Get exe download",async ()=>{
    const downloadLinks = await getDownloadLinks();
    expect(downloadLinks.exe)
        .include(".exe")
        .include("snubwoody/folio");
});

test("Get dmg download",async ()=>{
    const downloadLinks = await getDownloadLinks();
    expect(downloadLinks.dmg)
        .include(".dmg")
        .include("snubwoody/folio");
});

test("Get deb download",async ()=>{
    const downloadLinks = await getDownloadLinks();
    expect(downloadLinks.deb)
        .include(".deb")
        .include("snubwoody/folio");
});

