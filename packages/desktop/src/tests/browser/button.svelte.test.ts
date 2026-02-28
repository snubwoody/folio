import { expect, test,describe } from "vitest";
import { render } from "vitest-browser-svelte";
import { TextButton } from "$components/button";

describe("TextButton",() => {
    test("theme props sets data-theme attribute",async() => {
        const screen = render(TextButton,{ theme:"primary" });
        screen.getByRole("button");
        expect(screen.getByRole("button")).toHaveAttribute("data-theme","primary");
    });
});