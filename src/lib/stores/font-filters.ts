import { writable } from "svelte/store"

export enum FontSource {
    All = "All Fonts",
    System = "System Fonts",
    User = "User Fonts",
    GoogleFonts = "Google Fonts",
}

interface FontFilters {
    search: string
    source: FontSource
}

export const fontFilters = writable<FontFilters>({
    search: "",
    source: FontSource.All,
})
