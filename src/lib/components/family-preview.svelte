<script lang="ts">
    import { previewOptions } from "$lib/stores/preview-options.js"

    import { inview } from "svelte-inview"

    interface Props {
        family: FontFamily
    }

    let { family }: Props = $props()

    let isInView = $state(false)
</script>

<div
    use:inview={{}}
    oninview_change={({ detail }) => (isInView = detail.inView)}
    style="height: {$previewOptions.fontSize * 1.5}px;"
    class="flex"
>
    <div
        style="
            font-family: {family.family_name}; 
            font-size: {$previewOptions.fontSize}px;
            display: {isInView ? 'block' : 'none'};
        "
        class="whitespace-nowrap overflow-hidden"
    >
        {family.family_name}
    </div>
</div>
