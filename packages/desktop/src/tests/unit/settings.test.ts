import { test, expect, afterEach } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { describe } from "node:test";
import { SettingsStore } from "$lib/stores/settings.svelte";

afterEach(() => {
    clearMocks();
});

describe("SettingsStore",() => {
    test("set currency code",async () => {
        mockIPC((cmd) => {
            if (cmd === "set_currency_code" ) {
            } else {
                throw "Invalid command";
            }
        });

        const settingsStore = new SettingsStore();
        await settingsStore.setCurrencyCode("ZAR");
        expect(settingsStore.settings.currencyCode).toBe("ZAR");
    });
});

