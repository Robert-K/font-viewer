<script lang="ts">
    import FontList from "$lib/components/font-list.svelte"

    import * as Sidebar from "$lib/components/ui/sidebar/index.js"
    import AppSidebar from "$lib/components/app-sidebar.svelte"

    import FontSearch from "$lib/components/font-search.svelte"

    import FontSizeSlider from "$lib/components/font-size-slider.svelte"
    import FontWeightSlider from "$lib/components/font-weight-slider.svelte"
    import FontStyleToggle from "$lib/components/font-style-toggle.svelte"

    import DarkModeToggle from "$lib/components/dark-mode-toggle.svelte"
</script>

<Sidebar.Provider class="max-w-full" open={false}>
    <AppSidebar />
    <main class="h-screen w-full flex flex-col min-w-0">
        <div class="flex items-center justify-between gap-4 p-2 border-b">
            <div class="flex items-center gap-2">
                <Sidebar.Trigger />
                <FontSearch />
            </div>

            <div class="flex items-center gap-2">
                <FontSizeSlider />
                <FontWeightSlider />
                <FontStyleToggle />
            </div>
            
            <DarkModeToggle />
        </div>
        <FontList />
    </main>
</Sidebar.Provider>
