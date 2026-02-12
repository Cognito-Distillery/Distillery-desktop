<script lang="ts">
	import { getSidebarPosition, setSidebarPosition } from '$lib/stores/settings.svelte';
	import type { SidebarPosition } from '$lib/stores/settings.svelte';
	import { getEmail, logout } from '$lib/stores/auth.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { getLocale, setLocale } from '$lib/i18n/index.svelte';
	import type { Locale } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';

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
</div>
