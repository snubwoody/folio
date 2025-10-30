import { getDownloadLinks } from "@lib/downloads";
const downloadLinks = await getDownloadLinks();

const element = document.querySelector("#download-button");
element?.setAttribute("href", downloadLinks.exe);