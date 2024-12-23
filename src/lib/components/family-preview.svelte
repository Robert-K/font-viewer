<script lang="ts">
    import { previewOptions } from "$lib/stores/preview-options.js"

    import { inview } from "svelte-inview"

    interface Props {
        family: FontFamily
    }

    let { family }: Props = $props()

    let isInView = $state(false)
</script>

<div class="pl-5 pt-1">
    <h2 class="font-semibold text-muted-foreground">{family.family_name}</h2>
    <div
        use:inview={{rootMargin: "10%"}}
        oninview_change={({ detail }) => (isInView = detail.inView)}
        style="height: {$previewOptions.fontSize * 1.5}px;"
    >
        <input
            style="
            font-family: {family.family_name}; 
            font-size: {$previewOptions.fontSize}px;
            font-weight: {$previewOptions.fontWeight};
            font-style: {$previewOptions.fontStyle};
            display: {isInView ? 'block' : 'none'};
        "
            class="whitespace-nowrap overflow-hidden w-full outline-none"
            bind:value={$previewOptions.text}
            type="text"
        />
    </div>
</div>
