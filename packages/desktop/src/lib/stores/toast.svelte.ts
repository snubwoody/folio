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

export const createToastStore = () => {
    let toasts: Toast[] = $state([]);

    /**
     * Removes a {@link Toast} from the toast store.
     * @param id The id of the toast to remove.
     */
    const removeToast = (id: string) => toasts = toasts.filter(toast => toast.id !== id);

    /**
     * Empties the toast store.
     */
    const clear = () => toasts.length = 0;

    return {
        /**
         * Appends a {@link Toast} to the toast store.
         * @param toast
         * @param timeout The amount of time before removing the toast
         */
        addToast: (toast: Toast,timeout: number = 2000) => {
            // TODO: add timeout
            toasts.push(toast);
            setTimeout(() => {
                removeToast(toast.id);
            },timeout);
        },
        removeToast,
        clear,
        get toasts() {
            return toasts;
        }
    };
};

export interface ToastAction {
    /**
     * The text displayed in the button
     */
    text: string;
    /**
     * The event that gets called when the button is clicked.
     */
    action: () => void;
}

export interface Toast extends ToastParams {
    id: string;
}

/**
 * Parameters for creating a toast
 */
export interface ToastParams{
    title: string,
    body?: string,
    primaryAction?: ToastAction,
    secondaryAction?: ToastAction,
}

/**
 * Global toast store
 */
export const toastStore = createToastStore();

/**
 * Appends a {@link Toast} to the toast store.
 * @param params The parameters for the toast.
 * @param timeout The amount of time before removing the toast.
 */
export const addToast = (params: ToastParams,timeout: number = 8500) => {
    const toast: Toast = { id: randomId(),...params };
    toastStore.addToast(toast,timeout);
};

/**
 * Generates a unique random id.
 */
export const randomId = ():string => {
    let id = "";
    // Prevent the extremely low chance of Math.random returning 0
    // which would cause the id to be an empty string
    while (id === "") {
        id = Math.random().toString(36).slice(2, 10);
    }
    return id;
};

