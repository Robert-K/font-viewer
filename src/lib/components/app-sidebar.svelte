<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js"
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js"
    import { ChevronDown } from "lucide-svelte"

    import { fontFilters, FontSource } from "$lib/stores/font-filters"
</script>

<Sidebar.Root>
    <Sidebar.Header>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <DropdownMenu.Root>
                    <DropdownMenu.Trigger>
                        {#snippet child({ props })}
                            <Sidebar.MenuButton {...props}>
                                {$fontFilters.source}
                                <ChevronDown class="ml-auto" />
                            </Sidebar.MenuButton>
                        {/snippet}
                    </DropdownMenu.Trigger>
                    <DropdownMenu.Content
                        class="w-[--bits-dropdown-menu-anchor-width]"
                    >
                        {#each Object.values(FontSource) as source}
                            <DropdownMenu.Item
                                onclick={()=>$fontFilters.source = source}
                            >
                                {source}
                            </DropdownMenu.Item>
                        {/each}
                    </DropdownMenu.Content>
                </DropdownMenu.Root>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Header>
</Sidebar.Root>
