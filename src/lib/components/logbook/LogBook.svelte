<script lang='ts'>
    import type { Log } from "$lib/db/types";
    import LogEntry from "$lib/components/logbook/LogEntry.svelte";

    type Props = {
        logs: Log[];
        selectedLog: Log | undefined;
        addLog: () => void;
        selectLog: (log: Log) => void;
        isMobile: boolean;
    };
    
    let { logs, selectedLog, addLog, selectLog, isMobile }: Props = $props();
</script>

<button class="w-full  px-4 py-3 text-left transition dark:text-white hover:bg-zinc-800/60 border-b border-r border-orange-200 dark:border-stone-800" onclick={addLog}>
    + new log
</button>
{#if logs.length === 0}
    <div class="text-stone-500 dark:text-slate-500 px-4 py-5">no logs yet</div>
{:else}
    {#each logs as log (log.id)}
        <LogEntry {log} {selectedLog} {selectLog} {isMobile}/>
    {/each}
{/if}
