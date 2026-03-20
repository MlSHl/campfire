<script lang="ts">
	import type { Footstep } from '$lib/db/types';
	import CheckIcon from '$lib/icons/CheckIcon.svelte';

	type Props = {
		step: Footstep;
		toggleComplete: (id: string) => void;
		updateFootstep: (step: Footstep) => void;
		openDetails: (step: Footstep) => void;
	};

	let { step, toggleComplete, updateFootstep, openDetails }: Props = $props();

    let name = $state('');

    $effect(() => {
        name = step.name;
    });

	function handleDetailsKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			openDetails(step);
		}
	}

    function handleBlur() {
        if (name != step.name) {
            updateFootstep({... step, name});
        }
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

	<input
		type="text"
		placeholder="next step"
		bind:value={name}
        onblur={handleBlur}
		class="w-3/5 md:w-2/5 py-2 focus:outline-none text-sm text-zinc-800 dark:text-zinc-100"
		class:line-through={step.completed}
		class:text-zinc-400={step.completed}
		class:dark:text-zinc-500={step.completed}
	/>

	<div class="h-10 w-px bg-zinc-900/10 dark:bg-white/10"></div>

	<div
		role="button"
		tabindex="0"
		aria-label="Open step details"
		onclick={() => openDetails(step)}
		onkeydown={handleDetailsKeydown}
		class="h-12 w-2/5 md:w-3/5"
	></div>
</div>
