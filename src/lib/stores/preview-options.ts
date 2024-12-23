import { writable } from "svelte/store"

interface PreviewOptions {
    fontSize: number
    fontWeight: number
    fontStyle: string
    text: string
}

export const previewOptions = writable<PreviewOptions>({
    fontSize: 16,
    fontWeight: 400,
    fontStyle: "normal",
    text: "The quick brown fox jumps over the lazy dog",
})
