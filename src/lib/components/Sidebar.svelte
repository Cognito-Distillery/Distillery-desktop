<script lang="ts">
	import { page } from '$app/state';
	import { isHorizontal } from '$lib/stores/settings.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import { invoke } from '@tauri-apps/api/core';
	import { friendlyError } from '$lib/utils/error';
	import { showToast } from '$lib/stores/toast.svelte';

	async function goToWeb() {
		try {
			const url = await invoke<string>('get_web_url');
			await openUrl(url);
		} catch (e) {
			showToast(friendlyError(e));
		}
	}

	const links = [
		{ href: '/', titleKey: 'nav.malting' as const, icon: 'plus' },
		{ href: '/malts', titleKey: 'nav.maltHouse' as const, icon: 'doc' },
		{ href: '/still', titleKey: 'nav.still' as const, icon: 'share' },
		{ href: '/drawback', titleKey: 'nav.drawback' as const, icon: 'undo' }
	];

	function isActive(href: string) {
		return page.url.pathname === href;
	}

	const activeClass = 'bg-primary/15 text-primary';
	const inactiveClass = 'text-base-content/40 hover:text-base-content/70 hover:bg-white/[0.05]';
</script>

{#if isHorizontal()}
	<nav class="h-12 border-b border-base-300 flex items-center justify-between px-5 shrink-0">
		<div class="flex items-center gap-1">
			<a href="/" class="mr-3" title="Distillery">
				<svg width="24" height="24" viewBox="0 0 24 24" fill="none">
					<path d="M13 16A5 5 0 1 1 3 16C3 13.5 4.5 11.5 6 10.2V6A2 2 0 0 1 10 6V10.2C11.5 11.5 13 13.5 13 16Z" fill="currentColor" class="text-primary"/>
					<path d="M9 5C12.5 5 18 6.5 18 11V12" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" class="text-primary"/>
					<path d="M18 14.5L16.2 17.2C15.6 18.1 16.3 19.5 17.5 19.5H18.5C19.7 19.5 20.4 18.1 19.8 17.2L18 14.5Z" fill="currentColor" class="text-primary"/>
				</svg>
			</a>

			{#each links as link}
				<a
					href={link.href}
					class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive(link.href) ? activeClass : inactiveClass}"
					title={t(link.titleKey)}
				>
					{#if link.icon === 'plus'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
						</svg>
					{:else if link.icon === 'doc'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/>
						</svg>
					{:else if link.icon === 'share'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
						</svg>
					{:else if link.icon === 'undo'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 102.13-9.36L1 10"/>
						</svg>
					{/if}
				</a>
			{/each}
		</div>

		<div class="flex items-center gap-1">
			<button
				onclick={goToWeb}
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {inactiveClass}"
				title={t('nav.goToWeb')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4-10 15.3 15.3 0 014-10z"/>
				</svg>
			</button>
			<a
				href="/help"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/help') ? activeClass : inactiveClass}"
				title={t('nav.help')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 015.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/>
				</svg>
			</a>
			<a
				href="/settings"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/settings') ? activeClass : inactiveClass}"
				title={t('nav.settings')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="3"/>
					<path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
				</svg>
			</a>
		</div>
	</nav>
{:else}
	<nav class="w-14 h-screen border-r border-base-300 flex flex-col items-center justify-between py-5 shrink-0">
		<div class="flex flex-col items-center gap-1">
			<a href="/" class="mb-4" title="Distillery">
				<svg width="28" height="28" viewBox="0 0 24 24" fill="none">
					<path d="M13 16A5 5 0 1 1 3 16C3 13.5 4.5 11.5 6 10.2V6A2 2 0 0 1 10 6V10.2C11.5 11.5 13 13.5 13 16Z" fill="currentColor" class="text-primary"/>
					<path d="M9 5C12.5 5 18 6.5 18 11V12" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" class="text-primary"/>
					<path d="M18 14.5L16.2 17.2C15.6 18.1 16.3 19.5 17.5 19.5H18.5C19.7 19.5 20.4 18.1 19.8 17.2L18 14.5Z" fill="currentColor" class="text-primary"/>
				</svg>
			</a>

			{#each links as link}
				<a
					href={link.href}
					class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive(link.href) ? activeClass : inactiveClass}"
					title={t(link.titleKey)}
				>
					{#if link.icon === 'plus'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
						</svg>
					{:else if link.icon === 'doc'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/>
						</svg>
					{:else if link.icon === 'share'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
						</svg>
					{:else if link.icon === 'undo'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 102.13-9.36L1 10"/>
						</svg>
					{/if}
				</a>
			{/each}
		</div>

		<div class="flex flex-col items-center gap-1">
			<button
				onclick={goToWeb}
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {inactiveClass}"
				title={t('nav.goToWeb')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4-10 15.3 15.3 0 014-10z"/>
				</svg>
			</button>
			<a
				href="/help"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/help') ? activeClass : inactiveClass}"
				title={t('nav.help')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 015.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/>
				</svg>
			</a>
			<a
				href="/settings"
				class="w-9 h-9 flex items-center justify-center rounded-lg transition-colors {isActive('/settings') ? activeClass : inactiveClass}"
				title={t('nav.settings')}
			>
				<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="3"/>
					<path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
				</svg>
			</a>
		</div>
	</nav>
{/if}
