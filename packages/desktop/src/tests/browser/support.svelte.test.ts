import { test, expect, beforeEach } from "vitest";
import SettingsPanel from "$components/settings/SettingsPanel.svelte";
import { render } from "vitest-browser-svelte";
import { mockIPC } from "@tauri-apps/api/mocks";
import { appStore } from "$lib/state.svelte";

beforeEach(() => {
    appStore.categories = [];
});

mockIPC((cmd) => {
    if (cmd === "fetch_incomes" || cmd === "fetch_expenses") {
        return [];
    }
    if (cmd === "settings") {
        return { currencyCode: "USD" };
    }
    if (cmd === "currencies") {
        return ["USD", "CAD", "ZAR", "ZMW", "TSH"];
    }
});

test("Support section in settings panel", async () => {
    const page = render(SettingsPanel);
    expect(page.getByRole("heading",{ name:"Support" })).toBeInTheDocument();
    expect(page.getByRole("button",{ name:"Open form" }).nth(1)).toBeInTheDocument();
});