<script lang="ts">
	type Props = {
		clearCompleted: () => void;
	};

    let { clearCompleted }: Props = $props();
</script>

<button
    type="button"
    onclick={clearCompleted}
    class="w-38/100 text-center pointer-events-auto bg-orange-100/80 px-4 py-3 text-sm font-medium text-zinc-900 transition hover:bg-red-500/10 dark:bg-zinc-800/40 dark:text-zinc-100 dark:hover:bg-red-500/10"
>
    clear
</button>
