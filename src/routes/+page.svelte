<script lang="ts">
    import FontList from "$lib/components/font-list.svelte"

    import * as Sidebar from "$lib/components/ui/sidebar/index.js"
    import AppSidebar from "$lib/components/app-sidebar.svelte"

    import FontSizeSlider from "$lib/components/font-size-slider.svelte"
    import FontWeightSlider from "$lib/components/font-weight-slider.svelte"
    import FontStyleToggle from "$lib/components/font-style-toggle.svelte"
    
    import DarkModeToggle from "$lib/components/dark-mode-toggle.svelte"
</script>

<Sidebar.Provider class="max-w-full">
    <AppSidebar />
    <main class="h-screen w-full flex flex-col min-w-0">
        <div class="flex items-center justify-between p-2 border-b">
            <Sidebar.Trigger />
            <FontSizeSlider />
            <FontWeightSlider />
            <FontStyleToggle />
            <DarkModeToggle />
        </div>
        <FontList />
    </main>
</Sidebar.Provider>
