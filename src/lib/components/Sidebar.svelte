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
				<svg width="24" height="24" viewBox="0 0 200 200" fill="none">
					<defs>
						<linearGradient id="copH" x1="50" y1="150" x2="150" y2="50" gradientUnits="userSpaceOnUse">
							<stop offset="0%" stop-color="#92400E"/><stop offset="50%" stop-color="#F59E0B"/><stop offset="100%" stop-color="#FEF3C7"/>
						</linearGradient>
						<linearGradient id="liqH" x1="100" y1="100" x2="100" y2="180" gradientUnits="userSpaceOnUse">
							<stop offset="0%" stop-color="#FBBF24" stop-opacity="0.1"/><stop offset="100%" stop-color="#F59E0B" stop-opacity="0.6"/>
						</linearGradient>
					</defs>
					<path d="M60 140 C60 110, 80 100, 90 95 V 70" stroke="url(#copH)" stroke-width="4" stroke-linecap="round" fill="none"/>
					<path d="M110 70 V 95 C120 100, 140 110, 140 140 C140 165, 122 185, 100 185 C78 185, 60 165, 60 140 Z" stroke="url(#copH)" stroke-width="4" stroke-linecap="round" fill="url(#liqH)"/>
					<path d="M90 70 C90 60, 90 50, 100 40 C110 30, 130 30, 145 45 L 155 55" stroke="url(#copH)" stroke-width="4" stroke-linecap="round" fill="none"/>
					<path d="M155 65 C155 65, 150 72, 150 76 A 5 5 0 0 0 160 76 C160 72, 155 65, 155 65 Z" fill="#FCD34D"/>
					<path d="M75 130 Q 80 110, 95 105" stroke="#FEF3C7" stroke-width="2" stroke-opacity="0.4" stroke-linecap="round"/>
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
				<svg width="28" height="28" viewBox="0 0 200 200" fill="none">
					<defs>
						<linearGradient id="copV" x1="50" y1="150" x2="150" y2="50" gradientUnits="userSpaceOnUse">
							<stop offset="0%" stop-color="#92400E"/><stop offset="50%" stop-color="#F59E0B"/><stop offset="100%" stop-color="#FEF3C7"/>
						</linearGradient>
						<linearGradient id="liqV" x1="100" y1="100" x2="100" y2="180" gradientUnits="userSpaceOnUse">
							<stop offset="0%" stop-color="#FBBF24" stop-opacity="0.1"/><stop offset="100%" stop-color="#F59E0B" stop-opacity="0.6"/>
						</linearGradient>
					</defs>
					<path d="M60 140 C60 110, 80 100, 90 95 V 70" stroke="url(#copV)" stroke-width="4" stroke-linecap="round" fill="none"/>
					<path d="M110 70 V 95 C120 100, 140 110, 140 140 C140 165, 122 185, 100 185 C78 185, 60 165, 60 140 Z" stroke="url(#copV)" stroke-width="4" stroke-linecap="round" fill="url(#liqV)"/>
					<path d="M90 70 C90 60, 90 50, 100 40 C110 30, 130 30, 145 45 L 155 55" stroke="url(#copV)" stroke-width="4" stroke-linecap="round" fill="none"/>
					<path d="M155 65 C155 65, 150 72, 150 76 A 5 5 0 0 0 160 76 C160 72, 155 65, 155 65 Z" fill="#FCD34D"/>
					<path d="M75 130 Q 80 110, 95 105" stroke="#FEF3C7" stroke-width="2" stroke-opacity="0.4" stroke-linecap="round"/>
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
