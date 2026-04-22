<script lang='ts'>
    type Props = {
        first?: boolean
    };
    let { first = false }:Props  = $props();
</script>

<div class="flex items-center justify-between 
    {first ? 'border-t-0' : 'border-t-1'}
    border-orange-200/80 px-6 py-4 dark:border-stone-800 dark:hover:bg-[#151a28] hover:bg-orange-100/80">
    <slot />
</div>
