// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
import { invoke } from "@tauri-apps/api/core";
import type {
    Settings
} from "../types";
import { logger } from "../utils/logger";
import { mockIPC } from "@tauri-apps/api/mocks";

export class SettingsStore{
    #settings: Settings = $state({ currencyCode: "USD", sidebarOpen: true });

    get settings():Settings {
        return this.#settings;
    }

    /**
     * Resets to default settings.
     */
    reset(){
        this.#settings = { currencyCode: "USD", sidebarOpen: true };
    }

    async setCurrencyCode(currency: string) {
        try{
            await invoke("set_currency_code", { currency });
            this.#settings.currencyCode = currency;
        }catch (e) {
            console.error(e);
        }
    }

    async setSidebarState(open: boolean) {
        try{
            await invoke("set_sidebar_state", { open });
            this.#settings.sidebarOpen = open;
        }catch (e) {
            console.error(e);
        }
    }

    async load(){
        this.#settings = await invoke<Settings>("settings");
        logger.debug("Loaded settings from backend");
    }
}

export function mockSettings(){
    mockIPC((cmd) => {
        if (cmd === "settings") {
            return { currencyCode: "USD" };
        }
        if (cmd === "currencies") {
            return ["USD", "CAD", "ZAR", "ZMW", "TSH"];
        }
        if (cmd === "set_currency_code") {
            return;
        }
    });
}

export const settingsStore = new SettingsStore();
