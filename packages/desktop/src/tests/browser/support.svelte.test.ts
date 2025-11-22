import { test, expect } from "vitest";
import SettingsPanel from "$components/settings/SettingsPanel.svelte";
import { render } from "vitest-browser-svelte";

test("Support section in settings panel", async () => {
    const page = render(SettingsPanel);
    expect(page.getByRole("heading",{ name:"Support" })).toBeInTheDocument();
    expect(page.getByRole("button",{ name:"Open form" }).nth(1)).toBeInTheDocument();
});