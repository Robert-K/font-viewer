<script lang="ts">
    import { Input } from "$lib/components/ui/input/index.js"
    import { fontFilters } from "$lib/stores/font-filters.js"
</script>

<Input type="text" bind:value={$fontFilters.search} placeholder="Search..." />