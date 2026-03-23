<script lang='ts'>
    type Props = {
        addEmber: () => void;
    }

    let { addEmber }:Props = $props();
</script>
<div onclick={addEmber} class="rounded-xl dark:text-zinc-100 bg-white/50 px-3 py-2 text-center dark:bg-white/[0.03]">
    +
</div>
