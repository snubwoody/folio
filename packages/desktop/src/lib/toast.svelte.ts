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

export type ToastAction = {
    text: string;
    action: () => void;
};

export interface Toast extends ToastParams {
    id: string;
};

export interface ToastParams{
    title: string,
    body?: string,
    primaryAction?: ToastAction,
    secondaryAction?: ToastAction,
}

export const toastStore = createToastStore();

/**
 * Appends a {@link Toast} to the toast store.
 * @param params The parameters for the toast.
 * @param timeout The amount of time before removing the toast.
 */
export const addToast = (params: ToastParams,timeout: number = 2000) => {
    const toast: Toast = {id: randomId(),...params};
    toastStore.addToast(toast,timeout);
};

export const randomId = ():string => {
    let id = "";
    // Prevent the extremely low chance of Math.random returning 0
    // which would cause the id to be an empty string
    while (id === "") {
        id = Math.random().toString(36).slice(2, 10);
    }
    return id;
};

