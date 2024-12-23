<script lang="ts">
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js"
    import { Separator } from "$lib/components/ui/separator/index.js"

    import { invoke } from "@tauri-apps/api/core"

    import FamilyPreview from "./family-preview.svelte"

    import { previewOptions } from "$lib/stores/preview-options"
    import { fontFilters } from "$lib/stores/font-filters"
    import { tick } from "svelte"

    let scrollElement = $state<HTMLElement | null>(null)

    let installed_families = $state<FontFamily[]>([])

    let filtered_families = $derived(
        installed_families.filter((family) => {
            return family.family_name
                .toLowerCase()
                .includes($fontFilters.search.toLowerCase())
        })
    )

    previewOptions.subscribe(async (options) => {
        if (scrollElement) {
            let scrollPercentage =
                scrollElement.scrollTop / scrollElement.scrollHeight
            await tick()
            scrollElement.scrollTop =
                scrollPercentage * scrollElement.scrollHeight
        }
    })

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

<ScrollArea bind:viewportRef={scrollElement} orientation="vertical">
    {#each filtered_families as family, i}
        {#if i > 0}
            <Separator />
        {/if}
        <FamilyPreview {family} />
    {:else}
        <div class="pl-5 pt-1 font-semibold text-muted-foreground">
            No fonts found
        </div>
    {/each}
</ScrollArea>
