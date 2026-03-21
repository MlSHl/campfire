<script lang="ts">
	import { goto } from '$app/navigation';
	import BackIcon from '$lib/icons/BackIcon.svelte';

	type Props = {
		backDestination: string;
		onBackClick: () => Promise<void> | void;
	};

	let { backDestination, onBackClick }: Props = $props();

	async function handleClick(event: MouseEvent) {
		event.preventDefault();
		await onBackClick();
		await goto(backDestination);
	}
</script>

<a
	href={backDestination}
	onclick={handleClick}
	aria-label="Go back"
	class="inline-flex h-10 w-10 items-center justify-center rounded-xl text-zinc-700 transition active:bg-black/5 dark:text-zinc-200 dark:active:bg-white/5"
>
	<BackIcon />
</a>
