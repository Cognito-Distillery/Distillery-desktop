<script lang="ts">
	import '../app.css';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import LoginScreen from '$lib/components/LoginScreen.svelte';
	import { getSidebarPosition, isHorizontal } from '$lib/stores/settings.svelte';
	import { checkAuth, isAuthLoading, isAuthenticated } from '$lib/stores/auth.svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	let { children } = $props();

	const isFloating = getCurrentWindow().label === 'floating-memo';

	const layoutClass = $derived(() => {
		const pos = getSidebarPosition();
		if (pos === 'top') return 'flex flex-col';
		if (pos === 'bottom') return 'flex flex-col-reverse';
		if (pos === 'right') return 'flex flex-row-reverse';
		return 'flex';
	});

	$effect(() => {
		if (!isFloating) {
			checkAuth();
		}
	});
</script>

{#if isFloating}
	{@render children()}
{:else if isAuthLoading()}
	<div class="flex h-screen items-center justify-center">
		<span class="loading loading-spinner loading-lg text-primary"></span>
	</div>
{:else if !isAuthenticated()}
	<LoginScreen />
{:else}
	<div class="{layoutClass()} h-screen overflow-hidden">
		<Sidebar />

		<main class="flex-1 overflow-y-auto py-10 px-12">
			{@render children()}
		</main>
	</div>
{/if}
