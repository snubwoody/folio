import { test, expect } from "vitest";
import SettingsPanel from "$components/settings/SettingsPanel.svelte";
import { render } from "vitest-browser-svelte";
import {mockIPC} from "@tauri-apps/api/mocks";

mockIPC(() => {

});
test("Support section in settings panel", async () => {
    const page = render(SettingsPanel);
    expect(page.getByRole("heading",{ name:"Support" })).toBeInTheDocument();
    expect(page.getByRole("button",{ name:"Open form" }).nth(1)).toBeInTheDocument();
});