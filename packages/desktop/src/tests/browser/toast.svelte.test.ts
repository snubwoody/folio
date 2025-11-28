import ToastGroup from "$components/popups/ToastGroup.svelte";
import ToastComponent from "$components/popups/Toast.svelte";
import { test, beforeEach, expect } from "vitest";
import { render } from "vitest-browser-svelte";
import { toastStore,addToast,type Toast } from "$lib/toast.svelte";

beforeEach(() => {
    toastStore.clear();
});

test("Show toasts in toast store", async () => {
    // Set a long timeout to reduce flakyness
    addToast({ title: "New update available" },20000);
    addToast({ title: "Something went wrong" },20000);
    const page = render(ToastGroup);
    const toasts = page.getByRole("listitem").all();
    expect(toasts).toHaveLength(2);
    expect(toasts[0].getByText("New update available")).toBeInTheDocument();
    expect(toasts[1].getByText("Something went wrong")).toBeInTheDocument();
});

test("Display toast info", async () => {
    const toast: Toast = {
        id: "",
        title: "Something went wrong",
        body: "We don't know what it is"
    };
    const page = render(ToastComponent,{ toast });
    const item = page.getByRole("listitem");
    expect(item).toBeInTheDocument();
    expect(item.getByText("Something went wrong")).toBeInTheDocument();
    expect(item.getByText("We don't know what it is")).toBeInTheDocument();
});

test("Display toast buttons", async () => {
    const toast: Toast = {
        id: "",
        title: "Something went wrong",
        primaryAction:{
            text: "Primary",
            action: () => {}
        },
        secondaryAction:{
            text: "Secondary",
            action: () => {}
        }
    };
    const page = render(ToastComponent,{ toast });
    const item = page.getByRole("listitem");
    expect(item).toBeInTheDocument();
    expect(item.getByRole("button",{ name:"Primary" })).toBeInTheDocument();
    expect(item.getByRole("button",{ name:"Secondary" })).toBeInTheDocument();
});

test("Click primary button", async () => {
    let num = 0;
    const toast: Toast = {
        id: "",
        title: "Something went wrong",
        primaryAction:{
            text: "Primary",
            action: () => num = 10
        }
    };
    const page = render(ToastComponent,{ toast });
    const item = page.getByRole("listitem");
    await item.getByRole("button",{ name:"Primary" }).click();
    expect(num).toBe(10);
});

test("Click secondary button", async () => {
    let num = 0;
    const toast: Toast = {
        id: "",
        title: "Something went wrong",
        secondaryAction:{
            text: "Secondary",
            action: () => num = 10
        }
    };
    const page = render(ToastComponent,{ toast });
    const item = page.getByRole("listitem");
    await item.getByRole("button",{ name:"Secondary" }).click();
    expect(num).toBe(10);
});

