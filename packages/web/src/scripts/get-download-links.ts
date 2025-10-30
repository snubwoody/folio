import {getDownloadLinks} from "@lib/downloads";
const downloadLinks = await getDownloadLinks();

let element = document.querySelector("#download-button");
element?.setAttribute("href", downloadLinks.exe);