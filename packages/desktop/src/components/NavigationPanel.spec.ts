import { expect, test,describe } from "vitest";
import { render } from "vitest-browser-svelte";
import NavigationPanel from "./NavigationPanel.svelte";

describe("Navigation Panel",() => {
    // TODO: collapse navbar
    test("has navigation links",async() => {
        const screen = render(NavigationPanel);
        const transactionsLink = screen.getByRole("link",{name:"Transactions"});
        const spendingLink = screen.getByRole("link",{name:"Spending"});

        await expect.element(transactionsLink).toBeVisible();
        await expect.element(spendingLink).toBeVisible();
        await expect.element(transactionsLink).toHaveAttribute("href","/");
        await expect.element(spendingLink).toHaveAttribute("href","/analytics");
    });


    test("open settings panel", async () => {
        const screen = render(NavigationPanel);
        await screen.getByLabelText("Open settings").click();
        await expect
            .element(screen.getByLabelText("Settings panel"))
            .toBeVisible();
    });

    // TODO: test default state
    test("expand and collapse sidebar", async () => {
        const screen = render(NavigationPanel);
        await screen.getByRole("button",{name: "Collapse sidebar"}).click();
        await expect.element(screen.getByTestId("nav-panel")).toHaveAttribute("data-expanded","false");
        await screen.getByRole("button",{name: "Collapse sidebar"}).click();
        await expect.element(screen.getByTestId("nav-panel")).toHaveAttribute("data-expanded","true");
    });

    test("hide links in collapsed sidebar", async () => {
        const screen = render(NavigationPanel);
        await screen.getByRole("button",{name: "Collapse sidebar"}).click();
        const transactionsLink = screen.getByRole("link",{name:"Transactions"});
        const spendingLink = screen.getByRole("link",{name:"Spending"});

        await expect.element(transactionsLink).not.toBeInTheDocument();
        await expect.element(spendingLink).not.toBeInTheDocument();
    });
});