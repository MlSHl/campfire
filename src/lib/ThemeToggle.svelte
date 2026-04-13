<script lang="ts">
    import { Sun, Moon } from 'lucide-svelte';
    let isDark = $state(
        typeof document !== 'undefined' &&
        document.documentElement.classList.contains('dark')
    );

    function toggleTheme() {
        isDark = !isDark;
        localStorage.setItem("theme", isDark ? "dark" : "light");
        document.documentElement.classList.toggle("dark", isDark);
    }
</script>

<button
	type="button"
	onclick={toggleTheme}
	aria-label="Toggle theme"
	class="relative inline-flex h-8 w-14 items-center rounded-full bg-emerald-800 transition-colors duration-300 ease-in-out dark:bg-orange-500"
>
    <span class="absolute left-2 text-white/60 dark:text-white/30">
        <Sun size={14} />
    </span>

    <span class="absolute right-2 text-white/30 dark:text-white/60">
        <Moon size={14} />
    </span>

	<span
		class="absolute left-1 h-6 w-6 translate-x-0 rounded-full bg-white shadow-md transition-transform duration-300 ease-in-out dark:translate-x-6"
	></span>
</button>
