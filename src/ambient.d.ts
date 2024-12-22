declare global {
    interface Font {
        family_name: string
        font_name: string
        path: string
        style: string
        weight: number
        stretch: number
    }

    interface FontFamily {
        family_name: string
        fonts: Font[]
    }
}

export {}
