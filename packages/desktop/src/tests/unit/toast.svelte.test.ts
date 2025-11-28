import { test, expect, beforeEach,vi } from "vitest";
import {addToast, createToastStore, toastStore} from "$lib/toast.svelte";

beforeEach(() => {
    toastStore.clear();
});

test("Add toast", () => {
    const toastStore = createToastStore();
    toastStore.addToast({id:"1",title: "Hi"});
    toastStore.addToast({id:"2",title: "Hey"});
    expect(toastStore.toasts).toHaveLength(2);
    expect(toastStore.toasts[0].id).toBe("1");
    expect(toastStore.toasts[1].id).toBe("2");
});

test("Remove toast after timeout", async() => {
    vi.useFakeTimers();
    const toastStore = createToastStore();
    toastStore.addToast({id:"1",title: "Hi"},500);
    expect(toastStore.toasts).toHaveLength(1);
    vi.advanceTimersByTime(500);
    expect(toastStore.toasts).toHaveLength(0);
});

test("Remove global toast after timeout", async() => {
    vi.useFakeTimers();
    toastStore.addToast({id:"1",title: "Hi"},500);
    expect(toastStore.toasts).toHaveLength(1);
    vi.advanceTimersByTime(500);
    expect(toastStore.toasts).toHaveLength(0);
});

test("Global add toast", () => {
    addToast({title: "Toast 1"});
    addToast({title: "Toast 2"});
    expect(toastStore.toasts).toHaveLength(2);
    expect(toastStore.toasts[0].title).toBe("Toast 1");
    expect(toastStore.toasts[1].title).toBe("Toast 2");
});

test("Clear toast store", () => {
    const toastStore = createToastStore();
    toastStore.addToast({id:"",title: "Toast 1"});
    expect(toastStore.toasts).toHaveLength(1);
    toastStore.clear();
    expect(toastStore.toasts).toHaveLength(0);
});

test("Remove a toast", () => {
    const toastStore = createToastStore();
    toastStore.addToast({id:"1",title: "Toast 1"});
    toastStore.addToast({id:"2",title: "Toast 2"});
    expect(toastStore.toasts).toHaveLength(2);
    toastStore.removeToast("2");
    expect(toastStore.toasts).toHaveLength(1);
    expect(toastStore.toasts[0].title).toBe("Toast 1");
});

test("Remove all toasts with the same id", () => {
    const toastStore = createToastStore();
    toastStore.addToast({id:"1",title: "Toast 1"});
    toastStore.addToast({id:"1",title: "Toast 2"});
    expect(toastStore.toasts).toHaveLength(2);
    toastStore.removeToast("1");
    expect(toastStore.toasts).toHaveLength(0);
});
