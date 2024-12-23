import { writable } from "svelte/store"

export const fontFilters = writable({
    search: "",
})
