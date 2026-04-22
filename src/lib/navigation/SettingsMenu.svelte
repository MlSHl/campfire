<script lang="ts">
    import { SlidersHorizontal, LogIn } from 'lucide-svelte';

    import LoginModal from '$lib/auth/LoginModal.svelte';
    import ThemeToggle from '$lib/ThemeToggle.svelte'; 
    import SyncTestButton from '$lib/components/SyncTestButton.svelte';
    import SettingsTab from '$lib/navigation/SettingsTab.svelte';

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

		<div class="fixed z-20 right-0 top-[3.65rem] rounded-bl-2xl border border-orange-200 bg-orange-50 shadow-sm dark:border-stone-800 dark:bg-[#0f1220] dark:text-zinc-300">
			<SettingsTab first={true}>
				<span>Sync: local only</span>
			</SettingsTab>

			<SettingsTab>
				Theme
				<ThemeToggle />
			</SettingsTab>

            <SettingsTab>
                <button type="button" onclick={openLogin} class="inline-flex items-center gap-2">
                    <LogIn size={16} />
                    <span>Log in</span>
                </button>
            </SettingsTab>
            <SettingsTab>
                <SyncTestButton />
            </SettingsTab>
		</div>
    {/if}
</div>

<LoginModal open={showLogin} onClose={closeLogin}/>

