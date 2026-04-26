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
import type {Currency} from "$lib/types";

export class SettingsStore{
    #settings: Settings = $state({ currencyCode: "ZMW", sidebarOpen: true });
    #currency: Currency = $state({name:"Zambian Kwacha",precision: 2,symbol:"K",code:"ZMW"});

    get settings():Settings {
        return this.#settings;
    }

    get currency():Currency {
        return this.#currency;
    }

    /**
     * Resets to default settings.
     */
    reset(){
        this.#settings = { currencyCode: "ZMW", sidebarOpen: true };
        this.#currency = {name:"Zambian Kwacha",precision: 2,symbol:"K",code:"ZMW"};
    }

    async setCurrencyCode(currency: string) {
        try{
            // TODO: test this
            await invoke("set_currency_code", { currencyCode: currency });
            this.#settings.currencyCode = currency;
            this.#currency = await invoke<Currency>("active_currency");
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
        this.#currency = await invoke<Currency>("active_currency");
        logger.debug("Loaded settings from backend");
    }
}

export function mockSettings(){
    mockIPC((cmd) => {
        if (cmd === "settings") {
            return { currencyCode: "ZMW" };
        }
        if (cmd === "currencies") {
            return ["AED", "CAD", "ZAR", "ZMW", "TSH"];
        }
        if (cmd === "set_currency_code") {
            return;
        }
    });
}

export const settingsStore = new SettingsStore();
