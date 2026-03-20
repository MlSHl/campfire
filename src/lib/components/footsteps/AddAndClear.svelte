<script lang="ts">
	type Props = {
		addFootstep: () => void;
		clearCompleted: () => void;
	};

	let { addFootstep, clearCompleted }: Props = $props();

</script>

<div class="flex">
    <button
        type="button"
        onclick={addFootstep}
        class="pointer-events-auto w-3/5 md:w-2/5 border-r border-black/10 dark:border-white/10 bg-orange-100/80 px-4 py-3 text-left text-sm font-medium text-zinc-900 transition hover:bg-orange-100 dark:bg-zinc-800/40 dark:text-zinc-100 dark:hover:bg-zinc-700"
    >
        next step
    </button>
    <button
        type="button"
        onclick={clearCompleted}
        class="w-2/5 md:w-3/5 text-center pointer-events-auto bg-orange-100/80 px-4 py-3 text-sm font-medium text-zinc-900 transition hover:bg-red-500/10 dark:bg-zinc-800/40 dark:text-zinc-100 dark:hover:bg-red-500/10"
    >
        clear
    </button>
</div>
