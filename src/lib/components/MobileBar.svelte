<script lang='ts'>
    import MobileBack from "$lib/components/MobileBack.svelte"; 
    type Props = { 
        isMobile: boolean;
        onBackClick: () => void;
        onButtonClick: () => void;
        backDestination: string;
        buttonText: string;
    }
    let { isMobile, onBackClick, backDestination, onButtonClick, buttonText }: Props = $props();
</script>

<div class="w-full  px-4 py-1 flex items-center justify-between border-b border-orange-200 dark:border-stone-800">
    <div>
    {#if isMobile}
        <MobileBack {backDestination} {onBackClick}/>
    {/if}
    </div>

    <button
        type="button"
        onclick={onButtonClick}
        class="my-1 rounded-xl bg-emerald-800 px-4 py-[6px] text-sm font-medium text-white transition hover:opacity-90 dark:bg-orange-500"
    >
        {buttonText}
    </button>
</div>
