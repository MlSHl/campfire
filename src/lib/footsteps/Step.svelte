<script lang="ts">
    import type { StepType } from '$lib/types/step.type';
    type Props = {
        step: StepType;
        completeStep: (id: number) => void;
        addStep: (step: StepType) => number;
        updateStep: (step: StepType, id: number) => void;
    };
    let { step, completeStep }:Props = $props();
</script>

<label
	class="flex items-center gap-3 px-3 py-2 transition-colors hover:bg-zinc-100 dark:hover:bg-zinc-800/60"
>
	<input
		type="checkbox"
		checked={step.completed}
		onchange={() => completeStep(step.id)}
		class="h-4 w-4 shrink-0 accent-orange-500"
	/>
	<input
        type="textfield"
		class="w-full focus:outline-none text-sm text-zinc-800 dark:text-zinc-100"
		class:line-through={step.completed}
		class:text-zinc-400={step.completed}
		class:dark:text-zinc-500={step.completed}
	/>
</label>
