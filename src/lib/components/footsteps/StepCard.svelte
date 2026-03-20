<script lang='ts'>
    import { fly } from 'svelte/transition';
    import type { Footstep } from '$lib/db/types';
    type Props = {
        selectedStep: Footstep,
        closeDetails: () => void,
        saveSelectedStep: () => void,
        deleteSelectedStep: () => void
    };
    let { selectedStep, closeDetails, saveSelectedStep, deleteSelectedStep }: Props = $props();

    function handleBlur () {
        saveSelectedStep();
    }
</script>


<div class="hidden md:block absolute top-4 right-4 bottom-4 w-[calc(56.5%-1rem)]"
        in:fly={{ x: 24, duration: 180 }}
        out:fly={{ x: 24, duration: 140 }}
>
    <div class="flex h-full flex-col rounded-2xl border border-orange-200 bg-orange-50/60 shadow-sm dark:border-stone-800 dark:bg-[#151a28]">
        <div class="flex items-center justify-between gap-3 border-b border-orange-200 px-4 py-3 dark:border-stone-800">
            <button
                type="button"
                onclick={closeDetails}
                class="rounded-xl p-2 text-zinc-600 hover:bg-black/5 dark:text-zinc-300 dark:hover:bg-white/5"
            >
                ✕
            </button>

            <input
                type="text"
                bind:value={selectedStep.name}
                onblur={handleBlur}
                placeholder="Untitled step"
                class="min-w-0 flex-1 bg-transparent pb-3 text-2xl font-semibold text-center text-zinc-900 placeholder:text-zinc-400 focus:outline-none dark:text-zinc-100 dark:placeholder:text-zinc-500"
            />


            <button
                type="button"
                onclick={saveSelectedStep}
                class="rounded-xl bg-emerald-800 px-4 py-2 text-sm font-medium text-white hover:opacity-90 dark:bg-orange-500"
            >
                Save
            </button>
        </div>

        <div class="flex-1 p-4">
            <textarea
                bind:value={selectedStep.content}
                onblur={handleBlur}
                placeholder="Write notes, links, thoughts..."
                class="h-full w-full resize-none bg-transparent text-base leading-7 text-zinc-800 placeholder:text-zinc-400 focus:outline-none dark:text-zinc-200 dark:placeholder:text-zinc-500"
            ></textarea>
        </div>
        <button onclick={deleteSelectedStep} class="rounded-b-2xl rounded-t-none border-t border-orange-200 dark:border-stone-800 px-4 py-2 text-sm font-medium text-red-600 hover:bg-red-500/10 dark:text-red-400 dark:hover:bg-red-500/10">delete</button>
    </div>
</div>
