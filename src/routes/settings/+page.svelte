<script lang="ts">
	import { getSidebarPosition, setSidebarPosition } from '$lib/stores/settings.svelte';
	import type { SidebarPosition } from '$lib/stores/settings.svelte';
	import { getEmail, logout } from '$lib/stores/auth.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { getLocale, setLocale } from '$lib/i18n/index.svelte';
	import type { Locale } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';
	import { check } from '@tauri-apps/plugin-updater';
	import { relaunch } from '@tauri-apps/plugin-process';

	const APP_VERSION = __APP_VERSION__;

	const positions: { value: SidebarPosition; titleKey: MessageKey }[] = [
		{ value: 'top', titleKey: 'settings.pos.top' },
		{ value: 'left', titleKey: 'settings.pos.left' },
		{ value: 'right', titleKey: 'settings.pos.right' },
		{ value: 'bottom', titleKey: 'settings.pos.bottom' }
	];

	const languages: { value: Locale; label: string }[] = [
		{ value: 'ko', label: '한국어' },
		{ value: 'en', label: 'English' }
	];

	let updateStatus: 'idle' | 'checking' | 'available' | 'downloading' | 'ready' | 'upToDate' | 'error' = $state('idle');
	let updateVersion = $state('');

	async function checkForUpdates() {
		updateStatus = 'checking';
		try {
			const update = await check();
			if (update) {
				updateVersion = update.version;
				updateStatus = 'available';
			} else {
				updateStatus = 'upToDate';
			}
		} catch {
			updateStatus = 'error';
		}
	}

	async function installUpdate() {
		updateStatus = 'downloading';
		try {
			const update = await check();
			if (update) {
				await update.downloadAndInstall();
				updateStatus = 'ready';
				await relaunch();
			}
		} catch {
			updateStatus = 'error';
		}
	}
</script>

<div class="max-w-lg mx-auto flex flex-col gap-8">
	<div>
		<h1 class="text-2xl font-semibold tracking-tight">{t('settings.title')}</h1>
	</div>

	<section class="border border-base-300 rounded-lg p-5">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider mb-4">{t('settings.account')}</h2>

		<div class="flex items-center justify-between">
			<div class="flex flex-col gap-0.5">
				<span class="text-sm font-medium">{getEmail()}</span>
				<span class="text-xs text-base-content/35">{t('settings.authenticated')}</span>
			</div>
			<button class="btn btn-outline btn-sm btn-error" onclick={logout}>
				{t('settings.logout')}
			</button>
		</div>
	</section>

	<section class="border border-base-300 rounded-lg p-5 flex flex-col gap-5">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider">{t('settings.layout')}</h2>

		<div class="flex items-center justify-between">
			<div class="flex flex-col gap-0.5">
				<span class="text-sm font-medium">{t('settings.sidebarPosition')}</span>
				<span class="text-xs text-base-content/35">{t('settings.sidebarDesc')}</span>
			</div>
			<div class="join">
				{#each positions as pos}
					<button
						class="btn btn-sm join-item"
						class:btn-primary={getSidebarPosition() === pos.value}
						onclick={() => setSidebarPosition(pos.value)}
					>
						{t(pos.titleKey)}
					</button>
				{/each}
			</div>
		</div>

		<div class="flex items-center justify-between">
			<div class="flex flex-col gap-0.5">
				<span class="text-sm font-medium">{t('settings.language')}</span>
				<span class="text-xs text-base-content/35">{t('settings.languageDesc')}</span>
			</div>
			<div class="join">
				{#each languages as lang}
					<button
						class="btn btn-sm join-item"
						class:btn-primary={getLocale() === lang.value}
						onclick={() => setLocale(lang.value)}
					>
						{lang.label}
					</button>
				{/each}
			</div>
		</div>
	</section>

	<section class="border border-base-300 rounded-lg p-5">
		<h2 class="text-xs font-medium text-base-content/40 uppercase tracking-wider mb-4">{t('update.app')}</h2>

		<div class="flex items-center justify-between">
			<div class="flex flex-col gap-0.5">
				<span class="text-sm font-medium">{t('update.version')}</span>
				<span class="text-xs text-base-content/35">v{APP_VERSION}</span>
			</div>

			{#if updateStatus === 'idle'}
				<button class="btn btn-outline btn-sm" onclick={checkForUpdates}>
					{t('update.check')}
				</button>
			{:else if updateStatus === 'checking'}
				<span class="text-sm text-base-content/50">{t('update.checking')}</span>
			{:else if updateStatus === 'available'}
				<div class="flex items-center gap-3">
					<span class="text-sm text-success">{t('update.available').replace('{version}', updateVersion)}</span>
					<button class="btn btn-primary btn-sm" onclick={installUpdate}>
						{t('update.install')}
					</button>
				</div>
			{:else if updateStatus === 'downloading'}
				<span class="text-sm text-base-content/50">{t('update.downloading')}</span>
			{:else if updateStatus === 'ready'}
				<span class="text-sm text-success">{t('update.readyToInstall')}</span>
			{:else if updateStatus === 'upToDate'}
				<span class="text-sm text-base-content/50">{t('update.upToDate')}</span>
			{:else if updateStatus === 'error'}
				<div class="flex items-center gap-3">
					<span class="text-sm text-error">{t('update.failed')}</span>
					<button class="btn btn-outline btn-sm" onclick={checkForUpdates}>
						{t('update.check')}
					</button>
				</div>
			{/if}
		</div>
	</section>
</div>
