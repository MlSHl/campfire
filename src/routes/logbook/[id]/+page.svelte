<script lang='ts'>
    import LogEditor from "$lib/components/logbook/LogEditor.svelte";    
    import type { Log } from "$lib/db/types";
    import { getLog, updateLog, deleteLog } from "$lib/db/logbook";
    import { onMount } from "svelte";
    import { page } from "$app/state";
    let log = $state<Log | undefined>();

    onMount(async () => {
        if (page.params.id) {
            log = (await getLog(page.params.id)) ?? undefined; 
        }
    });

    async function deletePassedLog() {
        if (log) await deleteLog(log.id); 
    }

    async function updateSelectedLogField(field: 'name' | 'content', value: string) {
        if (!log) return;

		log = {
			...log,
			[field]: value
		};

        await updateLog(log);
    }
</script>

<LogEditor {log} isMobile={true} {deletePassedLog} {updateSelectedLogField}/>

