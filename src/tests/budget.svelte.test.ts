import BudgetOverview from "../components/analytics/BudgetOverview.svelte";
import { test } from "vitest";
import { render } from "vitest-browser-svelte";

test("Total expenses",()=>{
    const page = render(BudgetOverview);
});

test("Filter total expenses by month",()=>{

});