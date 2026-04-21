<script lang="ts">
	import type { Log } from '$lib/db/types';
	import { updateLog } from '$lib/db/logbook';
	import { goto } from '$app/navigation';
	import MobileBar from '$lib/components/MobileBar.svelte';

	type Props = {
		log: Log | undefined;
		isMobile: boolean;
		deletePassedLog: () => void;
		updateSelectedLogField: (field: 'name' | 'content', value: string) => void;
	};

	let { log, isMobile, deletePassedLog, updateSelectedLogField }: Props = $props();

	async function handleBlur() {
		if (log) updateLog(log);
	}

	function handleDelete() {
		deletePassedLog();
		if (isMobile) goto('/logbook');
	}
</script>

<div class="flex h-full flex-col">
	<MobileBar
		{isMobile}
		onBackClick={handleBlur}
		backDestination="/logbook"
		onButtonClick={handleDelete}
		buttonText="Delete"
	/>

	{#if log}
		<div class={`flex min-h-0 flex-1 flex-col ${isMobile ? 'px-4 pb-4 pt-2' : 'px-2 py-2 md:px-2 md:py-2'}`}>
			<input
				type="text"
				value={log.name}
				oninput={(e) => updateSelectedLogField('name', (e.currentTarget as HTMLInputElement).value)}
				onblur={handleBlur}
				placeholder="Untitled note"
				class={`w-full border-0 bg-transparent pb-2 font-semibold tracking-tight text-zinc-900 outline-none placeholder:text-zinc-500 focus:outline-none dark:text-zinc-100 dark:placeholder:text-zinc-500 ${
					isMobile ? 'text-2xl' : 'text-3xl'
				}`}
			/>

			<div class="h-px w-full bg-orange-200 dark:bg-stone-800"></div>

			<textarea
				oninput={(e) => updateSelectedLogField('content', (e.currentTarget as HTMLTextAreaElement).value)}
				onblur={handleBlur}
				placeholder="Begin logging..."
				class={`mt-5 min-h-0 w-full flex-1 resize-none border-0 bg-transparent outline-none placeholder:text-stone-400 focus:outline-none dark:placeholder:text-slate-500 ${
					isMobile
						? 'text-xl leading-6 text-stone-800 dark:text-[#b7ada1]'
						: 'text-base leading-7 text-stone-800 dark:text-[#b7ada1]'
				}`}
			>{log.content}</textarea>
		</div>
	{:else}
		<div class={`flex min-h-0 flex-1 items-start ${isMobile ? 'px-4 py-4' : 'px-8 py-8 md:px-10 md:py-10'}`}>
			<div class="text-stone-500 dark:text-slate-500">
				Select a log
			</div>
		</div>
	{/if}
</div>
