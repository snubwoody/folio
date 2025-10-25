<script>
    import TextField from "$components/TextField.svelte";
</script>

<div class="flex items-center justify-between w-full">
    <div class="space-y-0.5">
        <p>Currency code</p>
        <p class="text-text-muted text-sm">The ISO currency code</p>
    </div>
    <TextField value="USD" />
</div>
