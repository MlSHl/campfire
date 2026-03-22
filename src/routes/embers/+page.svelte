<script lang="ts">
	import { onMount } from 'svelte';
	import type { Ember } from '$lib/db/types';
	import { getAllEmbers, createEmber, deleteEmber, updateEmber } from '$lib/db/embers';
    import DesktopEmbers from "$lib/components/embers/DesktopEmbers.svelte";
    import MobileEmbers from "$lib/components/embers/MobileEmbers.svelte";

	let embers = $state<Ember[]>([]);
	const columns = ['activity', 'today', 'week', 'total', 'actions'];

	onMount(async () => {
		embers = await getAllEmbers();
	});

    async function addEmber() {
        await createEmber({name:"",hoursToday:0,hoursThisWeek:0,hoursTotal:0}); 
		embers = await getAllEmbers();
    }
    
    async function removeEmber(id: string) {
        await deleteEmber(id);
		embers = await getAllEmbers();
    }

	function updateEmberField(
		id: string,
		field: 'name' | 'hoursToday' | 'hoursThisWeek' | 'hoursTotal',
		value: string
	) {
		embers = embers.map((ember) =>
			ember.id === id
				? {
						...ember,
						[field]:
							field === 'name'
								? value
								: Number(value) || 0
				  }
				: ember
		);
	}

    async function saveEmber(id: string) {
		const ember = embers.find((e) => e.id === id);
		if (!ember) return;
		await updateEmber(ember);
	}

</script>

<div class="hidden md:block">
    <DesktopEmbers {columns} {embers} {addEmber} {removeEmber} {updateEmberField} {saveEmber}/>
</div>
<div class="md:hidden" >
    <MobileEmbers {columns} {embers} {addEmber} {removeEmber} {updateEmberField} {saveEmber}/>
</div>
