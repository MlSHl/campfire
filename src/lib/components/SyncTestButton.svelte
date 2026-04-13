<script lang="ts">
	import { runEmberSync } from '$lib/sync/embers';

	let syncing = $state(false);
	let syncMessage = $state('');

	async function handleSync() {
		syncing = true;
		syncMessage = '';

		try {
			await runEmberSync();
			syncMessage = 'Sync complete';
		} catch (e) {
			syncMessage = e instanceof Error ? e.message : 'Sync failed';
		} finally {
			syncing = false;
		}
	}
</script>

<button type="button" onclick={handleSync} disabled={syncing}
class="dark:border-stone-800 rounded-2xl"
>
	{syncing ? 'Syncing...' : 'Sync embers'}
</button>

{#if syncMessage}
	<p>{syncMessage}</p>
{/if}
