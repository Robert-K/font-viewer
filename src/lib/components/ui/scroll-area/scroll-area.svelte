<script lang="ts">
	import { ScrollArea as ScrollAreaPrimitive, type WithoutChild } from "bits-ui";
	import { Scrollbar } from "./index.js";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		viewportRef = $bindable(null),
		viewportClass = "",
		class: className,
		orientation = "vertical",
		scrollbarXClasses = "",
		scrollbarYClasses = "",
		children,
		...restProps
	}: WithoutChild<ScrollAreaPrimitive.RootProps> & {
		viewportRef?: HTMLElement | null | undefined;
		viewportClass?: string | null | undefined;
		orientation?: "vertical" | "horizontal" | "both" | undefined;
		scrollbarXClasses?: string | undefined;
		scrollbarYClasses?: string | undefined;
	} = $props();
</script>

<ScrollAreaPrimitive.Root bind:ref {...restProps} class={cn("relative overflow-hidden", className)}>
	<ScrollAreaPrimitive.Viewport bind:ref={viewportRef} class={cn("h-full w-full rounded-[inherit]", viewportClass)}>
		{@render children?.()}
	</ScrollAreaPrimitive.Viewport>
	{#if orientation === "vertical" || orientation === "both"}
		<Scrollbar orientation="vertical" class={scrollbarYClasses} />
	{/if}
	{#if orientation === "horizontal" || orientation === "both"}
		<Scrollbar orientation="horizontal" class={scrollbarXClasses} />
	{/if}
	<ScrollAreaPrimitive.Corner />
</ScrollAreaPrimitive.Root>
