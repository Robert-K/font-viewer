<script lang="ts">
    import { Slider } from "$lib/components/ui/slider/index.js"
    import { LucideALargeSmall } from "lucide-svelte"
    import { previewOptions } from "$lib/stores/preview-options.js"
</script>

<div class="flex items-center gap-4">
    <LucideALargeSmall />
    <Slider
        value={[$previewOptions.fontSize]}
        min={1}
        max={100}
        step={1}
        class="w-32"
        onValueChange={(v) => {
            $previewOptions.fontSize = v[0];
        }}
    />
</div>
