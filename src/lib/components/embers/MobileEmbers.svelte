<script lang="ts">
	import type { Ember } from '$lib/db/types';
    import EmberCard from "$lib/components/embers/EmberCard.svelte";

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

{#each embers as ember}
    <EmberCard {columns} {ember} {updateEmberField} {saveEmber}/>
{/each}

