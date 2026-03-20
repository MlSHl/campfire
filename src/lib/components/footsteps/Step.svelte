<script lang="ts">
	import type { Footstep } from '$lib/db/types';
	import CheckIcon from '$lib/icons/CheckIcon.svelte';

	type Props = {
		step: Footstep;
		toggleComplete: (id: string) => void;
		updateFootstep: (step: Footstep) => void;
		openDetails: (step: Footstep) => void;
	};

	let { step = $bindable(), toggleComplete, updateFootstep, openDetails }: Props = $props();

	function handleDetailsKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			openDetails(step);
		}
	}

	let mirrorWidth = $state(0);
	const MIN_WIDTH = 72;
	const MAX_WIDTH = 260;

	let inputWidth = $derived.by(() =>
		Math.max(MIN_WIDTH, Math.min(mirrorWidth + 2, MAX_WIDTH))
	);

	let saveTimer: ReturnType<typeof setTimeout> | null = null;

	function scheduleSave() {
		if (saveTimer) clearTimeout(saveTimer);
		saveTimer = setTimeout(() => {
			updateFootstep(step);
			saveTimer = null;
		}, 300);
	}

	function flushSave() {
		if (saveTimer) {
			clearTimeout(saveTimer);
			saveTimer = null;
		}
		updateFootstep(step);
	}
</script>

<div class="flex items-center gap-3 px-3 transition-colors hover:bg-zinc-100 dark:hover:bg-zinc-800/60">
	<label class="relative flex items-center">
		<input
			type="checkbox"
			checked={step.completed === 1}
			onchange={() => toggleComplete(step.id)}
			class="peer sr-only"
		/>
		<span
			class="flex h-5 w-5 items-center justify-center rounded-md border border-zinc-400 bg-white transition
				peer-checked:border-emerald-800 peer-checked:bg-emerald-800
				dark:peer-checked:border-orange-500 dark:peer-checked:bg-orange-500
				dark:border-zinc-600 dark:bg-zinc-900"
		>
			<CheckIcon />
		</span>
	</label>

	<div class="relative shrink-0">
		<span
			bind:offsetWidth={mirrorWidth}
			aria-hidden="true"
			class="pointer-events-none invisible absolute left-0 top-0 inline-block whitespace-pre py-2 text-sm text-zinc-800 dark:text-zinc-100"
		>
			{step.name || 'next...'}
		</span>

		<input
			type="text"
			placeholder="next..."
			bind:value={step.name}
			oninput={scheduleSave}
			onblur={flushSave}
			style={`width:${inputWidth+40}px`}
			class="min-w-[72px] max-w-[260px] bg-transparent py-2 text-sm text-zinc-800 focus:outline-none dark:text-zinc-100"
			class:line-through={step.completed}
			class:text-zinc-400={step.completed}
			class:dark:text-zinc-500={step.completed}
		/>
	</div>

    <div
		role="button"
		tabindex="0"
		aria-label="Open step details"
		onclick={() => openDetails(step)}
		onkeydown={handleDetailsKeydown}
		class="h-12 flex-1"
	></div>
</div>
