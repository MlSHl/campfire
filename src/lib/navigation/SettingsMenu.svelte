<script lang="ts">
    import { SlidersHorizontal, LogIn } from 'lucide-svelte';

    import LoginModal from '$lib/auth/LoginModal.svelte';
    import ThemeToggle from '$lib/ThemeToggle.svelte'; 

    let open = $state(false);
    let showLogin = $state(false);

    function toggleMenu() {
        open = !open;
    }

    function closeMenu() {
        open = false;
    }

    function closeLogin() {
        showLogin = false;
    }

    function openLogin() {
        open = false;
        showLogin = true;
    }
</script>

<div class="relative">
    <button 
        type="button" 
        onclick={toggleMenu}
        class="dark:text-orange-100 pt-2"
    >
        <SlidersHorizontal size={25}/>
    </button>


    {#if open}
        <!-- A bit hacky, but supposedly a common pattern for this -->
        <button
            type="button"
            class="fixed inset-0 z-10"
            onclick={closeMenu}
            aria-label="Close settings menu"
        ></button>

		<div class="fixed z-20 right-0 top-[3.65rem] rounded-bl-2xl border border-orange-200 bg-orange-50 shadow-sm dark:border-stone-800 dark:bg-[#151a28] dark:text-zinc-300 p-4 ">
			<div>
				<span>Sync: local only</span>
			</div>

			<div class="flex items-center justify-between">
				<span>Theme</span>
				<ThemeToggle />
			</div>

			<button type="button" onclick={openLogin}>
				<LogIn size={16} />
				Log in
			</button>
		</div>
    {/if}
</div>

<LoginModal open={showLogin} onClose={closeLogin}/>

