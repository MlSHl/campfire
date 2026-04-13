<script lang="ts">
    import SettingsMenu from "$lib/navigation/SettingsMenu.svelte";
    import NavTab from '$lib/navigation/NavTab.svelte'
</script>

<nav class="bg-orange-100/80 border-b border-orange-200 dark:bg-[#151a28] dark:border-stone-800 transition-colors duration-300">
    <div class="flex">
        <div class="flex w-11/12">
            <NavTab tab_name="logbook"/>
            <NavTab tab_name="footsteps"/>
            <NavTab tab_name="embers"/>
        </div>
        <div class="flex justify-center items-center w-1/12">
            <SettingsMenu/>
        </div>
    </div>
</nav>
