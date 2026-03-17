<script lang="ts">
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
    onclick={toggleTheme}
    aria-label="Toggle theme"
    class="relative inline-flex h-8 w-14 items-center rounded-full transition-colors duration-300 ease-in-out bg-emerald-800 dark:bg-orange-500">
    <span
        class="absolute left-1 h-6 w-6 rounded-full bg-white shadow-md transition-transform duration-300 ease-in-out translate-x-0 dark:translate-x-6"
    ></span>
</button>
