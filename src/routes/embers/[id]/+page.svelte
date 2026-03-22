<script lang="ts">
	import type { Ember } from '$lib/db/types';
	import MobileBar from '$lib/components/MobileBar.svelte';
	import { getEmber, deleteEmber, updateEmber } from '$lib/db/embers';
	import { onMount, onDestroy } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';

	let ember = $state<Ember | undefined>(undefined);
	let saveTimer: ReturnType<typeof setTimeout> | null = null;

	onMount(async () => {
		ember = await getEmber(page.params.id);
	});

	onDestroy(() => {
		if (saveTimer) clearTimeout(saveTimer);
	});

	function scheduleSave() {
		if (!ember) return;

		if (saveTimer) clearTimeout(saveTimer);

		saveTimer = setTimeout(async () => {
			if (!ember) return;
			await updateEmber(ember);
			saveTimer = null;
		}, 300);
	}

	async function flushSave() {
		if (!ember) return;

		if (saveTimer) {
			clearTimeout(saveTimer);
			saveTimer = null;
		}

		await updateEmber(ember);
	}

	async function handleDelete() {
		if (!ember) return;
		await deleteEmber(ember.id);
		await goto('/embers');
	}

	async function onBackClick() {
		await flushSave();
	}

	function changeField(field: 'hoursToday' | 'hoursThisWeek' | 'hoursTotal', delta: number) {
		if (!ember) return;

		ember[field] = Math.max(0, ember[field] + delta);
		scheduleSave();
	}
</script>

<!-- TODO: refactor this
Make new component for each time card.
Change time stored in indexedDB into minutes and add proper formatting
e.g. store 150 and display 2 hours 30 minutes.
Additionally, add proper time wheel UI for mobile.
/-->
<MobileBar
	isMobile={true}
	{onBackClick}
	backDestination="/embers"
	onButtonClick={handleDelete}
	buttonText="Delete"
/>

{#if ember}
	<div class="min-h-[calc(100vh-5rem)] px-4 py-4 text-zinc-900 dark:text-zinc-100">
		<div class="rounded-2xl border border-orange-200 bg-orange-50/60 p-4 dark:border-stone-800 dark:bg-[#151a28]">
			<div class="text-xs uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
				Activity
			</div>
			<input
				type="text"
				bind:value={ember.name}
				oninput={scheduleSave}
				placeholder="Untitled activity"
				class="mt-2 w-full bg-transparent text-lg font-semibold text-zinc-900 outline-none placeholder:text-zinc-400 dark:text-zinc-100 dark:placeholder:text-zinc-500"
			/>
		</div>

		<div class="mt-4 space-y-3">
			<div class="flex items-center justify-between rounded-2xl border border-orange-200 bg-orange-50/60 px-4 py-4 dark:border-stone-800 dark:bg-[#151a28]">
				<div class="text-sm font-medium text-zinc-700 dark:text-zinc-300">Today</div>

				<div class="flex items-center gap-3">
					<button
						type="button"
						onclick={() => changeField('hoursToday', -1)}
						class="inline-flex h-9 w-9 items-center justify-center rounded-xl bg-white/60 text-lg font-medium text-zinc-900 transition hover:bg-white/80 dark:bg-white/[0.06] dark:text-zinc-100 dark:hover:bg-white/[0.1]"
					>
						−
					</button>

					<div class="min-w-[2rem] text-center text-base font-semibold text-zinc-900 dark:text-zinc-100">
						{ember.hoursToday}
					</div>

					<button
						type="button"
						onclick={() => changeField('hoursToday', 1)}
						class="inline-flex h-9 w-9 items-center justify-center rounded-xl bg-white/60 text-lg font-medium text-zinc-900 transition hover:bg-white/80 dark:bg-white/[0.06] dark:text-zinc-100 dark:hover:bg-white/[0.1]"
					>
						+
					</button>
				</div>
			</div>

			<div class="flex items-center justify-between rounded-2xl border border-orange-200 bg-orange-50/60 px-4 py-4 dark:border-stone-800 dark:bg-[#151a28]">
				<div class="text-sm font-medium text-zinc-700 dark:text-zinc-300">Week</div>

				<div class="flex items-center gap-3">
					<button
						type="button"
						onclick={() => changeField('hoursThisWeek', -1)}
						class="inline-flex h-9 w-9 items-center justify-center rounded-xl bg-white/60 text-lg font-medium text-zinc-900 transition hover:bg-white/80 dark:bg-white/[0.06] dark:text-zinc-100 dark:hover:bg-white/[0.1]"
					>
						−
					</button>

					<div class="min-w-[2rem] text-center text-base font-semibold text-zinc-900 dark:text-zinc-100">
						{ember.hoursThisWeek}
					</div>

					<button
						type="button"
						onclick={() => changeField('hoursThisWeek', 1)}
						class="inline-flex h-9 w-9 items-center justify-center rounded-xl bg-white/60 text-lg font-medium text-zinc-900 transition hover:bg-white/80 dark:bg-white/[0.06] dark:text-zinc-100 dark:hover:bg-white/[0.1]"
					>
						+
					</button>
				</div>
			</div>

			<div class="flex items-center justify-between rounded-2xl border border-orange-200 bg-orange-50/60 px-4 py-4 dark:border-stone-800 dark:bg-[#151a28]">
				<div class="text-sm font-medium text-zinc-700 dark:text-zinc-300">Total</div>

				<div class="flex items-center gap-3">
					<button
						type="button"
						onclick={() => changeField('hoursTotal', -1)}
						class="inline-flex h-9 w-9 items-center justify-center rounded-xl bg-white/60 text-lg font-medium text-zinc-900 transition hover:bg-white/80 dark:bg-white/[0.06] dark:text-zinc-100 dark:hover:bg-white/[0.1]"
					>
						−
					</button>

					<div class="min-w-[2rem] text-center text-base font-semibold text-zinc-900 dark:text-zinc-100">
						{ember.hoursTotal}
					</div>

					<button
						type="button"
						onclick={() => changeField('hoursTotal', 1)}
						class="inline-flex h-9 w-9 items-center justify-center rounded-xl bg-white/60 text-lg font-medium text-zinc-900 transition hover:bg-white/80 dark:bg-white/[0.06] dark:text-zinc-100 dark:hover:bg-white/[0.1]"
					>
						+
					</button>
				</div>
			</div>
		</div>
	</div>
{:else}
	<div class="px-4 py-4 text-zinc-500 dark:text-zinc-400">
		Loading...
	</div>
{/if}
