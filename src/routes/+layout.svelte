<script lang="ts">
    import "./layout.css";
    import NavBar from "$lib/navigation/NavBar.svelte";

    let { children } = $props();
</script>

<div class="flex h-screen flex-col overflow-hidden">
    <NavBar/>
    <div class="flex-1 min-h-0 overflow-y-auto">
        {@render children()}
    </div>
</div>
