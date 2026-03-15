<script lang="ts">
    let isDark = $state(
        typeof document !== 'undefined' &&
        document.documentElement.classList.contains('dark')
    );

    $effect(() => {
        document.documentElement.classList.toggle("dark", isDark);
        localStorage.setItem("theme", isDark ? "dark" : "light");
    });
    

    function toggleTheme() {
        isDark = !isDark;
    }
</script>

<button
    onclick={toggleTheme}
    aria-label="Toggle theme"
    class="relative inline-flex h-8 w-14 items-center rounded-full transition-colors duration-300 ease-in-out"
    class:bg-emerald-800={!isDark} class:bg-orange-600={isDark} >
    <span
        class="absolute left-1 h-6 w-6 rounded-full bg-white shadow-md transition-transform duration-300 ease-in-out"
        class:translate-x-0={!isDark}
        class:translate-x-6={isDark}
    ></span>
</button>
