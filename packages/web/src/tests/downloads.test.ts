
// FIXME: no longer valid
import { getLatestRelease } from "@lib/downloads.ts";
import { test } from "vitest";
test("Get latest release",async ()=>{
    await getLatestRelease();
});

//
// test("Get rpm download",async ()=>{
//     const downloadLinks = await getDownloadLinks();
//     expect(downloadLinks.rpm)
//         .include(".rpm")
//         .include("snubwoody/folio-site");
// });
//
// test("Get exe download",async ()=>{
//     const downloadLinks = await getDownloadLinks();
//     expect(downloadLinks.exe)
//         .include(".exe")
//         .include("snubwoody/folio-site");
// });
//
// test("Get dmg download",async ()=>{
//     const downloadLinks = await getDownloadLinks();
//     expect(downloadLinks.dmg)
//         .include(".dmg")
//         .include("snubwoody/folio-site");
// });
//
// test("Get msi download",async ()=>{
//     const downloadLinks = await getDownloadLinks();
//     expect(downloadLinks.msi)
//         .include(".msi")
//         .include("snubwoody/folio-site");
// });
//
// test("Get AppImage download",async ()=>{
//     const downloadLinks = await getDownloadLinks();
//     expect(downloadLinks.appImage)
//         .include(".AppImage")
//         .include("snubwoody/folio-site");
// });
//
// test("Get deb download",async ()=>{
//     const downloadLinks = await getDownloadLinks();
//     expect(downloadLinks.deb)
//         .include(".deb")
//         .include("snubwoody/folio-site");
// });

