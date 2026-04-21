<script lang='ts'>
	import type { Ember } from '$lib/db/types';

	type Props = {
		columns: string[];
		ember: Ember;
		updateEmberField: (
			id: string,
			field: 'name' | 'hoursToday' | 'hoursThisWeek' | 'hoursTotal',
			value: string
		) => void;
		saveEmber: (id: string) => void | Promise<void>;
	};

	let { columns, ember, updateEmberField, saveEmber }: Props = $props();
</script>

<a
	href={`/embers/${ember.id}`}
	class="my-2 block w-full rounded-2xl border border-orange-200 bg-orange-50/60 px-4 py-3 text-left transition hover:bg-orange-100 dark:border-stone-800 dark:bg-[#151a28] dark:text-zinc-100 dark:hover:bg-zinc-800/80"
>
	<div class="truncate text-lg font-semibold text-zinc-900 dark:text-zinc-100">
		{ember.name || 'Untitled activity'}
	</div>

	<div class="mt-4 grid grid-cols-3 gap-3">
		<div class="rounded-xl bg-white/50 px-3 py-2 text-center dark:bg-white/[0.03]">
			<div class="text-sm uppercase tracking-wide text-zinc-500 dark:text-zinc-400">Today</div>
			<div class="mt-1 text-base font-medium text-zinc-800 dark:text-zinc-200">{ember.hoursToday}</div>
		</div>

		<div class="rounded-xl bg-white/50 px-3 py-2 text-center dark:bg-white/[0.03]">
			<div class="text-sm uppercase tracking-wide text-zinc-500 dark:text-zinc-400">Week</div>
			<div class="mt-1 text-base font-medium text-zinc-800 dark:text-zinc-200">{ember.hoursThisWeek}</div>
		</div>

		<div class="rounded-xl bg-white/50 px-3 py-2 text-center dark:bg-white/[0.03]">
			<div class="text-sm uppercase tracking-wide text-zinc-500 dark:text-zinc-400">Total</div>
			<div class="mt-1 text-base font-medium text-zinc-800 dark:text-zinc-200">{ember.hoursTotal}</div>
		</div>
	</div>
</a>
