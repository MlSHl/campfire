<script lang="ts">
	import { login } from '$lib/api/auth';

	type Props = {
		open: boolean;
		onClose: () => void;
	};

	let { open, onClose }: Props = $props();

	let email = $state('');
	let password = $state('');
	let loading = $state(false);
	let error = $state('');

	async function submitLogin() {
		error = '';
		loading = true;

		try {
			const result = await login(email, password);

			if (result.status !== 'Success') {
				error = result.message || 'Login failed';
				return;
			}

			onClose();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Login failed';
		} finally {
			loading = false;
		}
	}
</script>

{#if open}
	<div class="fixed inset-0 z-30 flex items-center justify-center bg-black/50 p-4">
		<div class="w-full max-w-sm rounded-2xl border border-stone-200 bg-white p-6 shadow-xl dark:border-stone-800 dark:bg-[#151a28] dark:text-zinc-200">
			<h2 class="mb-2 text-lg font-semibold">Log in</h2>
			<p class="mb-4 text-sm text-zinc-600 dark:text-zinc-400">
				Log in to enable sync.
			</p>

			<form
				class="space-y-4"
				onsubmit={(e) => {
					e.preventDefault();
					submitLogin();
				}}
			>
				<div class="space-y-1">
					<label for="email" class="text-sm">Email</label>
					<input
						id="email"
						type="email"
						bind:value={email}
						class="w-full rounded-xl border border-stone-300 px-3 py-2 outline-none focus:border-orange-400 dark:border-stone-700 dark:bg-stone-900"
						required
					/>
				</div>

				<div class="space-y-1">
					<label for="password" class="text-sm">Password</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						class="w-full rounded-xl border border-stone-300 px-3 py-2 outline-none focus:border-orange-400 dark:border-stone-700 dark:bg-stone-900"
						required
					/>
				</div>

				{#if error}
					<p class="text-sm text-red-600 dark:text-red-400">{error}</p>
				{/if}

				<div class="flex justify-end gap-2 pt-2">
					<button
						type="button"
						onclick={onClose}
						class="rounded-xl border border-stone-300 px-4 py-2 dark:border-stone-700"
						disabled={loading}
					>
						Cancel
					</button>

					<button
						type="submit"
						class="rounded-xl bg-orange-500 px-4 py-2 text-white disabled:opacity-60"
						disabled={loading}
					>
						{loading ? 'Logging in...' : 'Log in'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
