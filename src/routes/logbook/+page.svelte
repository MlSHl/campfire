<script lang="ts">
    import type { Log } from "$lib/db/types";
    import { onMount } from "svelte";
    import { getAllLogs, createLog, deleteLog, updateLastVisitedAt,  getLastOpenedLog } from "$lib/db/logbook";
    import LogEditor from "$lib/components/logbook/LogEditor.svelte";
    import LogBook from "$lib/components/logbook/LogBook.svelte";

    let logs = $state<Log[]>([]);
    let selectedLog = $state<Log | undefined>();

    onMount(async () => {
        logs = await getAllLogs();
        const lastOpened = await getLastOpenedLog();
        selectedLog = lastOpened
            ? logs.find((log) => log.id === lastOpened.id)
            : logs[0];
    });

    async function addLog() {
        const newLog = await createLog({ name: '', content: '' });
        logs = [newLog, ...logs];
        selectedLog = newLog;
        await updateLastVisitedAt(newLog.id);
    }

    async function selectLog(log: Log) {
        selectedLog = log;
        await updateLastVisitedAt(log.id);
    }

    async function deletePassedLog() {
        if (selectedLog) {
            await deleteLog(selectedLog.id);
        }

        logs = await getAllLogs();

        const lastOpened = await getLastOpenedLog();
        selectedLog = lastOpened
            ? logs.find((log) => log.id === lastOpened.id)
            : logs[0];
    }

    function updateSelectedLogField(field: 'name' | 'content', value: string) {
        if (!selectedLog) return;

        selectedLog = {
            ...selectedLog,
            [field]: value
        };

        logs = logs.map((log) =>
            log.id === selectedLog?.id
                ? { ...log, [field]: value }
                : log
        );
    }

</script>

<div class="hidden h-[calc(100vh-5rem)] md:grid md:grid-cols-[minmax(0,1fr)_minmax(0,3fr)]">
	<div>
		<LogBook {logs} {selectedLog} {addLog} {selectLog} isMobile={false}/>
	</div>

	<div>
		<LogEditor log={selectedLog} isMobile={false} {deletePassedLog} {updateSelectedLogField}/>
	</div>
</div>
<div class="md:hidden h-[calc(100vh-5rem)]">
    <LogBook {logs} {selectedLog} {addLog} {selectLog} isMobile={true}/>
</div>
