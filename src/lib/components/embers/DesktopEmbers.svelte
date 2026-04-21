<script lang="ts">
	import type { Ember } from '$lib/db/types';

	type Props = {
		columns: string[];
		embers: Ember[];
		addEmber: () => void;
		removeEmber: (id: string) => void;
		updateEmberField: (
			id: string,
			field: 'name' | 'hoursToday' | 'hoursThisWeek' | 'hoursTotal',
			value: string
		) => void;
		saveEmber: (id: string) => void | Promise<void>;
	};

	let { columns, embers, addEmber, removeEmber, updateEmberField, saveEmber }: Props = $props();
</script>

<div class="w-full overflow-x-auto">
	<table class="w-full table-fixed border-collapse">
		<thead>
			<tr>
				{#each columns as column}
					<th
						class={`border-b border-orange-200/70 px-4 py-3 text-center text-sm font-semibold uppercase tracking-wide text-zinc-700 dark:border-stone-800 dark:text-zinc-300 ${
							column === 'activity'
								? 'w-[32%]'
								: column === 'actions'
									? 'w-[12%]'
									: 'w-[18.666%]'
						}`}
					>
						{column}
					</th>
				{/each}
			</tr>
		</thead>

		<tbody>
			{#if embers.length === 0}
				<tr>
					<td colspan={5} class="px-4 py-8 text-center text-stone-500 dark:text-slate-500">
						No activities yet
					</td>
				</tr>
			{:else}
				{#each embers as row (row.id)}
					<tr class="border-b border-orange-200/70 transition hover:bg-orange-100/50 dark:border-stone-800 dark:hover:bg-[#151a28]">
						<td class="px-4 py-4">
							<input
								type="text"
								value={row.name}
								oninput={(e) =>
									updateEmberField(row.id, 'name', (e.currentTarget as HTMLInputElement).value)}
								onblur={() => saveEmber(row.id)}
								placeholder="untitled"
								class="w-full bg-transparent text-center text-lg font-medium text-zinc-900 outline-none placeholder:text-zinc-400 dark:text-zinc-100 dark:placeholder:text-zinc-500"
							/>
						</td>

						<td class="px-4 py-4">
							<input
								type="text"
								value={row.hoursToday}
								oninput={(e) =>
									updateEmberField(row.id, 'hoursToday', (e.currentTarget as HTMLInputElement).value)}
								onblur={() => saveEmber(row.id)}
								class="w-full bg-transparent text-center text-lg text-zinc-700 outline-none dark:text-zinc-300"
							/>
						</td>

						<td class="px-4 py-4">
							<input
								type="text"
								value={row.hoursThisWeek}
								oninput={(e) =>
									updateEmberField(row.id, 'hoursThisWeek', (e.currentTarget as HTMLInputElement).value)}
								onblur={() => saveEmber(row.id)}
								class="w-full bg-transparent text-center text-lg text-zinc-700 outline-none dark:text-zinc-300"
							/>
						</td>

						<td class="px-4 py-4">
							<input
								type="text"
								value={row.hoursTotal}
								oninput={(e) =>
									updateEmberField(row.id, 'hoursTotal', (e.currentTarget as HTMLInputElement).value)}
								onblur={() => saveEmber(row.id)}
								class="w-full bg-transparent text-center text-lg text-zinc-700 outline-none dark:text-zinc-300"
							/>
						</td>

						<td class="truncate px-4 py-4 text-center text-lg text-zinc-700 dark:text-zinc-300">
							<button
								onclick={() => removeEmber(row.id)}
								class="rounded-md border px-2 py-1 hover:bg-red-500/20"
							>
								delete
							</button>
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>

	<button
		onclick={addEmber}
		class="pointer-events-auto w-full border-r border-black/10 bg-orange-100/80 px-4 py-3 text-center text-lg font-medium text-zinc-900 transition hover:bg-orange-100 dark:border-white/10 dark:bg-zinc-800/40 dark:text-zinc-100 dark:hover:bg-zinc-700"
	>
		+
	</button>
</div>
