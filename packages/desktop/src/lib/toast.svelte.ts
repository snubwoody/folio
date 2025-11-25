
// TODO: add action
export const createToastStore = () => {
    let toasts: Toast[] = $state([]);

    return {
        /**
         * Add a toast
         * @param toast
         */
        addToast: (toast: Toast) => {
            toasts.push(toast);
        },
        get toasts() {
            return toasts;
        }
    };
};

export type Toast = {
    title: string,
    body?: string,
};

export const toastStore = createToastStore();

export const addToast = (toast: Toast) => {
    toastStore.addToast(toast);
};