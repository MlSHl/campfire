<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { getFootstep, updateFootstep, deleteFootstep } from '$lib/db/footsteps';
	import type { Footstep } from '$lib/db/types';
    import MobileBar from "$lib/components/MobileBar.svelte";

	let step = $state<Footstep | null>(null);
	let loading = $state(true);
	let saveTimeout: ReturnType<typeof setTimeout> | null = null;

	onMount(() => {
		const previousBodyOverflow = document.body.style.overflow;
		document.body.style.overflow = 'hidden';

		return () => {
			document.body.style.overflow = previousBodyOverflow;
			if (saveTimeout) clearTimeout(saveTimeout);
		};
	});

	onMount(async () => {
		step = (await getFootstep(page.params.id)) ?? null;
		loading = false;
	});

	async function saveStep() {
		if (!step) return;
		await updateFootstep(step);
	}

	async function deleteStep() {
		if (!step) return;
		await deleteFootstep(step.id);
		goto('/footsteps');
	}
</script>

{#if loading}
	<div class="min-h-screen px-4 py-4 text-zinc-500 dark:text-zinc-400">
		Loading...
	</div>
{:else if step}
	<div class="min-h-screen text-zinc-900 dark:text-zinc-100">
    <MobileBar isMobile={true} onBackClick={saveStep} backDestination={"/footsteps"} onButtonClick={saveStep} buttonText={"Save"} />
		<div class="mx-auto max-w-2xl px-4 pt-5">
				<input
					type="text"
					bind:value={step.name}
					placeholder="Untitled step"
                    class="w-full bg-transparent text-center text-lg font-semibold tracking-tight text-zinc-900 placeholder:text-zinc-400 focus:outline-none dark:text-zinc-100 dark:placeholder:text-zinc-500 md:text-4xl border-b border-transparent focus:border-orange-200/80 focus:dark:border-stone-800"
				/>

			<div class="mt-2 overflow-hidden rounded-2xl border border-orange-200/80 bg-white/50 shadow-sm dark:border-stone-800 dark:bg-[#151a28]/60">
				<div class="p-4">
					<textarea
						bind:value={step.content}
						placeholder="Write notes, links, thoughts..."
						class="min-h-[55vh] w-full resize-none bg-transparent text-base leading-7 text-zinc-800 placeholder:text-zinc-400 focus:outline-none dark:text-zinc-200 dark:placeholder:text-zinc-500"
					></textarea>
				</div>

				<button
					onclick={deleteStep}
					class="block w-full border-t border-orange-200 bg-orange-50/60 px-4 py-3 text-sm font-medium text-red-600 hover:bg-red-500/10 dark:border-stone-800 dark:bg-[#151a28] dark:text-red-400 dark:hover:bg-red-500/10"
				>
					delete
				</button>
			</div>
		</div>
	</div>
{:else}
	<div class="min-h-screen px-4 py-4 text-zinc-600 dark:text-zinc-300">
		Step not found.
	</div>
{/if}
