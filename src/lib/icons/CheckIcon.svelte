<script lang="ts">
	type Props = {
		className?: string;
	};

	let {
		className = 'h-3.5 w-3.5 text-white opacity-0 transition peer-checked:opacity-100'
	}: Props = $props();
</script>

<svg
	class={className}
	viewBox="0 0 20 20"
	fill="none"
	stroke="currentColor"
	stroke-width="3"
	stroke-linecap="round"
	stroke-linejoin="round"
	aria-hidden="true"
>
	<path d="M5 10l3 3 7-7" />
</svg>
