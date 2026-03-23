<script lang="ts">
    import { page } from '$app/state';
    let { tab_name } = $props();
</script>


<a 
    class="btn flex-1 border-b-2 transition-all {page.url.pathname.startsWith('/' + tab_name) ? 'text-orange-800 border-emerald-800 dark:text-orange-500 dark:border-orange-500 font-bold' : 'border-transparent'}" 
    href="/{tab_name}"
>
    {tab_name}
</a>
