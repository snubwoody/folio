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
import type { Budget } from "./types";
import { logger } from "./utils/logger";

// TODO: just manage state manually
/**
 * @deprecated
 */
export class AppStore {
    budgets: Budget[] = $state([]);

    async editBudget(id: string, amount: string) {
        await invoke("edit_budget", { id, amount });
        await this.load();
    }

    async load() {
        this.budgets = (await invoke("fetch_budgets")) as Budget[];
        logger.debug("Loaded budgets from backend");
    }
}

export const appStore = new AppStore();
