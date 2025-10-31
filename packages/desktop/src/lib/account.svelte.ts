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
import type { AppStore } from "./state.svelte";

interface EditAccount{
    name?: string
    startingBalance?: string
}

export class AccountStore {
    #rootStore: AppStore;

    constructor(store: AppStore) {
        this.#rootStore = store;
    }

    async addAccount(name: string, startingBalance: string) {
        try {
            await invoke("create_account", { name, startingBalance });
        } catch (e) {
            console.error(e);
        }
        await this.#rootStore.load();
    }

    async editAccount(id: string, opts: EditAccount) {
        try {
            await invoke("edit_account", { id, opts });
        } catch (e) {
            console.error(e);
        }
        await this.#rootStore.load();
    }
}
