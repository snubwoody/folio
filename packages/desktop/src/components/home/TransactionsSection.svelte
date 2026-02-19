<!--
Copyright (C) 2025 Wakunguma Kalimukwa

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
-->
<script lang="ts">
    import ExpenseTable from "./ExpenseTable.svelte";
    import TransactionTable from "./TransactionTable.svelte";
    import IncomeTable from "./IncomeTable.svelte";
    import { SegmentedTabs,TabContent,TabButton,TabBar } from "$components/select";
    import AddTransaction from "$components/home/AddTransaction.svelte";

    type TabType = "Expenses" | "Income";
    let activeTab = $state<TabType>("Expenses");
</script>

<section class="space-y-2">
    <TransactionTable/>
</section>
