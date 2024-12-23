<script lang="ts">
    import FontList from "$lib/components/font-list.svelte"
    import { invoke } from "@tauri-apps/api/core"

    import DarkModeToggle from "$lib/components/dark-mode-toggle.svelte"

    import * as Sidebar from "$lib/components/ui/sidebar/index.js"
    import AppSidebar from "$lib/components/app-sidebar.svelte"

    let installed_families: FontFamily[] = []

    invoke<Font[]>("list_fonts").then((fonts) => {
        fonts.forEach((font) => {
            const family = installed_families.find(
                (f) => f.family_name === font.family_name
            )
            if (family) {
                family.fonts.push(font)
            } else {
                installed_families.push({
                    family_name: font.family_name,
                    fonts: [font],
                })
            }
        })
    })
</script>

<Sidebar.Provider>
    <AppSidebar />
    <main class="h-screen w-full flex flex-col">
        <div class="flex items-center justify-between p-2 border-b">
            <Sidebar.Trigger />
            <DarkModeToggle />
        </div>
        <FontList />
    </main>
</Sidebar.Provider>
