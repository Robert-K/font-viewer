<script lang="ts">
    import { ScrollArea } from "$lib/components/ui/scroll-area/index.js"
    import { Separator } from "$lib/components/ui/separator/index.js"

    import { invoke } from "@tauri-apps/api/core"

    import FamilyPreview from "./family-preview.svelte"

    import { previewOptions } from "$lib/stores/preview-options"
    import { tick } from "svelte"

    let scrollElement = $state<HTMLElement | null>(null)

    let installed_families = $state<FontFamily[]>([])

    previewOptions.subscribe(async (options) => {
        if (scrollElement) {
            let scrollPercentage =
                scrollElement.scrollTop / scrollElement.scrollHeight
            console.log(scrollElement.scrollTop)
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
    {#each installed_families as family, i}
        {#if i > 0}
            <Separator />
        {/if}
        <FamilyPreview {family} />
    {/each}
</ScrollArea>
