import { describe } from "node:test";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { afterEach, expect, test } from "vitest";
import { SettingsStore } from "$lib/stores/settings.svelte";

afterEach(() => {
    clearMocks();
});

describe("SettingsStore", () => {
    test("set currency code", async () => {
        mockIPC((cmd) => {
            if (cmd !== "set_currency_code") {
                throw "Invalid command";
            }
        });

        const settingsStore = new SettingsStore();
        await settingsStore.setCurrencyCode("ZAR");
        expect(settingsStore.settings.currencyCode).toBe("ZAR");
    });
});
