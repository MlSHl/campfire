<script lang="ts">
	import type { Log } from '$lib/db/types';
    import { goto } from "$app/navigation";

	type Props = {
		log: Log;
		selectedLog: Log | undefined;
		selectLog: (log: Log) => void;
        isMobile: boolean;
	};

	let { log, selectedLog, selectLog, isMobile }: Props = $props();

	function handleClick() {
		selectLog(log);
        if (isMobile) goto(`/logbook/${log.id}`);
	}

	let isSelected = $derived(!isMobile && selectedLog?.id === log.id);
</script>

<button
	onclick={handleClick}
	class={`w-full px-4 py-2 text-left transition ${
		isSelected
			? 'bg-orange-100 dark:bg-[#151a28] border-r-2 border-r-emerald-800 dark:border-r-orange-500'
			: 'hover:bg-orange-100 dark:hover:bg-[#151a28]'
	}`}
>
<div class="flex items-center justify-between gap-3 border-b border-zinc-200 pb-2 dark:border-stone-800 md:border-b-0 md:pb-0 ">
		<div class="min-w-0 flex-1">
			<div class="truncate text-lg font-medium text-zinc-800 dark:text-zinc-100">
				{log.name?.trim() || 'Untitled note'}
			</div>

			<div class="truncate mt-1 text-base text-zinc-500 dark:text-zinc-400">
                {log.content?.trim() || 'New log'}
			</div>
		</div>
	</div>
</button>
