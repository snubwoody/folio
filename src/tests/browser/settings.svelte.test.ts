import { beforeEach,test,expect } from "vitest";
import { appStore } from "$lib/state.svelte";
import Sidebar from "$components/Sidebar.svelte";
import { render } from "vitest-browser-svelte";

beforeEach(()=>{
    appStore.categories = [];
});

test("Open settings panel", async () => {
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await expect.element(page.getByLabelText("Settings panel")).toBeInTheDocument();
});

test("Default to general section", async () => {
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    const selected = await page.getByText("General").query()?.getAttribute("data-selected");
    expect(selected).toBe("true");
});

test("Show categories", async () => {
    appStore.categories = [
        { id:"1", title:"Rent" },
        { id:"2", title:"Groceries" },
    ];
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await page.getByText("Categories").click();

    const items = page.getByRole("listitem").all();
    expect(items[0].element().textContent?.trim()).toBe("Rent");
    expect(items[1].element().textContent?.trim()).toBe("Groceries");
});

test("Show income streams", async () => {
    appStore.incomeStreams = [
        { id:"1", title:"Salary" },
        { id:"2", title:"Dividends" },
    ];
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await page.getByText("Income streams").click();

    const items = page.getByRole("listitem").all();
    expect(items[0].element().textContent?.trim()).toBe("Salary");
    expect(items[1].element().textContent?.trim()).toBe("Dividends");
});