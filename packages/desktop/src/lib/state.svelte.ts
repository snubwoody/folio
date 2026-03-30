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
    Budget
} from "./lib";
import { logger } from "./logger";

// TODO: just manage state manually
/**
 * @deprecated
 */
export class AppStore {
    budgets: Budget[] = $state([]);

    async createBudget(amount: string, categoryId: string) {
        await invoke("create_budget", { amount, categoryId });
        await this.load();
    }

    async editBudget(id: string, amount: string) {
        await invoke("edit_budget", { id, amount });
        await this.load();
    }

    async deleteBudget(id: string) {
        await invoke("delete_budget", { id });
        // FIXME: no longer exists
        await this.load();
    }

    async load() {
        this.budgets = (await invoke("fetch_budgets")) as Budget[];
        logger.debug("Loaded budgets from backend");
    }
}

export const appStore = new AppStore();
