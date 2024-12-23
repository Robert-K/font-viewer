import { writable } from "svelte/store"

export const previewOptions = writable({
    fontSize: 16,
    fontWeight: 400,
    fontStyle: "normal",
    text: "The quick brown fox jumps over the lazy dog",
})
