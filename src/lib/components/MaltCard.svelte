<script lang="ts">
	import type { Malt, MaltType } from '$lib/types';
	import { MaltStatus } from '$lib/types';
	import type { ViewMode } from '$lib/stores/settings.svelte';
	import { deleteMalt, setMaltStatus, updateMalt, queueMalt, drawBackMalt, loadMalts, loadQueuedMalts } from '$lib/stores/malts.svelte';
	import { t } from '$lib/i18n/index.svelte';
	import { getLocale } from '$lib/i18n/index.svelte';
	import type { MessageKey } from '$lib/i18n/index.svelte';

	let {
		malt,
		view = 'list',
		mode = 'malts',
		currentStatus = ''
	}: {
		malt: Malt;
		view?: ViewMode;
		mode?: 'malts' | 'still' | 'drawback';
		currentStatus?: string;
	} = $props();

	const maltTypes: { type: MaltType; titleKey: MessageKey; btnClass: string }[] = [
		{ type: '결정', titleKey: 'type.결정', btnClass: 'btn-warning' },
		{ type: '문제', titleKey: 'type.문제', btnClass: 'btn-error' },
		{ type: '인사이트', titleKey: 'type.인사이트', btnClass: 'btn-success' },
		{ type: '질문', titleKey: 'type.질문', btnClass: 'btn-info' }
	];

	async function handleSetStatus(id: string, status: MaltStatus) {
		try {
			await setMaltStatus(id, status);
			await loadMalts(currentStatus);
		} catch {
			// toast already shown by store
		}
	}

	async function handleDelete(id: string) {
		try {
			await deleteMalt(id);
			await loadMalts(currentStatus);
		} catch {
			// toast already shown by store
		}
	}

	const typeBadgeClass: Record<MaltType, string> = {
		'결정': 'badge-warning',
		'문제': 'badge-error',
		'인사이트': 'badge-success',
		'질문': 'badge-info'
	};

	const typeBorderClass: Record<MaltType, string> = {
		'결정': 'border-l-warning',
		'문제': 'border-l-error',
		'인사이트': 'border-l-success',
		'질문': 'border-l-info'
	};

	const typeBgClass: Record<MaltType, string> = {
		'결정': 'border-warning/30',
		'문제': 'border-error/30',
		'인사이트': 'border-success/30',
		'질문': 'border-info/30'
	};

	const localeTag: Record<string, string> = { ko: 'ko-KR', en: 'en-US' };

	function formatDate(ts: number): string {
		return new Date(ts).toLocaleDateString(localeTag[getLocale()] ?? 'ko-KR', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function typeLabel(type: MaltType): string {
		return t(`type.${type}` as MessageKey);
	}

	let showConfirm = $state(false);

	// Edit mode
	let editing = $state(false);
	let editType = $state<MaltType>('결정');
	let editSummary = $state('');
	let editContext = $state('');
	let editMemo = $state('');
	let saving = $state(false);

	function startEdit() {
		editType = malt.type;
		editSummary = malt.summary;
		editContext = malt.context;
		editMemo = malt.memo;
		editing = true;
	}

	function cancelEdit() {
		editing = false;
	}

	async function saveEdit() {
		if (!editSummary.trim()) return;
		saving = true;
		try {
			await updateMalt(malt.id, {
				type: editType,
				summary: editSummary.trim(),
				context: editContext.trim(),
				memo: editMemo.trim()
			});
			editing = false;
			await loadMalts(currentStatus);
		} catch {
			// toast already shown by store
		} finally {
			saving = false;
		}
	}

	let canSave = $derived(editSummary.trim().length > 0);

	// Queue
	let queueing = $state(false);

	async function handleQueue() {
		queueing = true;
		try {
			await queueMalt(malt.id);
			await loadMalts(currentStatus);
		} catch {
			// toast already shown by store
		} finally {
			queueing = false;
		}
	}

	// Draw Back
	let drawingBack = $state(false);

	async function handleDrawBack() {
		if (!malt.serverId) return;
		drawingBack = true;
		try {
			await drawBackMalt(malt.id, malt.serverId);
			await loadQueuedMalts();
		} catch {
			// toast already shown by store
		} finally {
			drawingBack = false;
		}
	}
</script>

{#if editing}
	<article class="p-4 border border-primary/40 rounded-lg bg-white/[0.04] flex flex-col gap-3">
		<div class="flex gap-2">
			{#each maltTypes as mt}
				<button
					type="button"
					class="btn btn-outline btn-xs flex-1 {mt.btnClass}"
					class:btn-active={editType === mt.type}
					onclick={() => (editType = mt.type)}
				>
					{t(mt.titleKey)}
				</button>
			{/each}
		</div>

		<input
			type="text"
			class="input input-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30"
			placeholder={t('form.summary')}
			bind:value={editSummary}
		/>

		<textarea
			class="textarea textarea-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary text-sm placeholder:text-base-content/30"
			placeholder={t('form.context')}
			bind:value={editContext}
			rows="2"
		></textarea>

		<textarea
			class="textarea textarea-sm w-full bg-white/[0.12] border-white/[0.18] focus:border-primary text-sm placeholder:text-base-content/30"
			placeholder={t('form.memo')}
			bind:value={editMemo}
			rows="2"
		></textarea>

		<div class="flex justify-end gap-2">
			<button class="btn btn-ghost btn-xs" onclick={cancelEdit} disabled={saving}>{t('common.cancel')}</button>
			<button class="btn btn-primary btn-xs" onclick={saveEdit} disabled={!canSave || saving}>
				{#if saving}
					<span class="loading loading-spinner loading-xs"></span>
				{:else}
					{t('common.save')}
				{/if}
			</button>
		</div>
	</article>
{:else if view === 'card'}
	<article class="p-4 border {typeBgClass[malt.type]} rounded-lg hover:bg-white/[0.08] transition-colors group flex flex-col gap-2">
		<div class="flex items-center justify-between">
			<span class="badge badge-sm badge-outline {typeBadgeClass[malt.type]}">{typeLabel(malt.type)}</span>
			<div class="flex items-center gap-1">
				{#if mode === 'malts'}
					<button
						class="btn btn-ghost btn-xs text-base-content/25 hover:text-base-content/60"
						title={t('common.edit')}
						onclick={startEdit}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
							<path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
						</svg>
					</button>
					<button
						class="btn btn-ghost btn-xs text-base-content/25 hover:text-primary/70"
						title={t('card.putOnStill')}
						onclick={() => handleSetStatus(malt.id, MaltStatus.ON_STILL)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="18" cy="5" r="3"/>
							<circle cx="6" cy="12" r="3"/>
							<circle cx="18" cy="19" r="3"/>
							<line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
							<line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
						</svg>
					</button>
				{:else if mode === 'still'}
					<button
						class="btn btn-ghost btn-xs text-base-content/30 hover:text-warning"
						title={t('card.takeOffStill')}
						onclick={() => handleSetStatus(malt.id, MaltStatus.MALT_HOUSE)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4"/>
							<polyline points="16 17 21 12 16 7"/>
							<line x1="21" y1="12" x2="9" y2="12"/>
						</svg>
					</button>
				{/if}
				{#if mode !== 'drawback'}
					{#if showConfirm}
						<button class="btn btn-ghost btn-xs text-error" onclick={() => handleDelete(malt.id)}>{t('common.delete')}</button>
						<button class="btn btn-ghost btn-xs text-base-content/40" onclick={() => (showConfirm = false)}>{t('common.cancel')}</button>
					{:else}
						<button class="btn btn-ghost btn-xs text-base-content/30 hover:text-error" title={t('common.delete')} onclick={() => (showConfirm = true)}>
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<polyline points="3 6 5 6 21 6"/>
								<path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
							</svg>
						</button>
					{/if}
				{/if}
			</div>
		</div>

		<p class="text-[15px] font-medium leading-relaxed">{malt.summary}</p>

		{#if malt.context}
			<p class="text-sm text-base-content/50 leading-relaxed line-clamp-2">{malt.context}</p>
		{/if}

		{#if malt.memo}
			<p class="text-sm text-base-content/30 italic leading-relaxed line-clamp-1">{malt.memo}</p>
		{/if}

		<div class="flex items-center justify-between mt-auto pt-1">
			<span class="text-xs text-base-content/25">{formatDate(malt.createdAt)}</span>
			{#if mode === 'still'}
				<button
					class="btn btn-primary btn-xs gap-1"
					onclick={handleQueue}
				disabled={queueing}
				>
					{#if queueing}
						<span class="loading loading-spinner loading-xs"></span>
					{:else}
						<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
							<polyline points="17 8 12 3 7 8"/>
							<line x1="12" y1="3" x2="12" y2="15"/>
						</svg>
						{t('card.queue')}
					{/if}
				</button>
			{:else if mode === 'drawback'}
				<button
					class="btn btn-warning btn-xs gap-1"
					onclick={handleDrawBack}
					disabled={drawingBack}
				>
					{#if drawingBack}
						<span class="loading loading-spinner loading-xs"></span>
					{:else}
						<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<polyline points="1 4 1 10 7 10"/>
							<path d="M3.51 15a9 9 0 102.13-9.36L1 10"/>
						</svg>
						{t('card.drawBack')}
					{/if}
				</button>
			{/if}
		</div>
	</article>
{:else}
	<article class="p-4 border border-base-300 rounded-lg border-l-[3px] {typeBorderClass[malt.type]} hover:bg-white/[0.08] transition-colors group">
		<div class="flex items-center justify-between mb-2">
			<span class="badge badge-sm badge-outline {typeBadgeClass[malt.type]}">{typeLabel(malt.type)}</span>
			<div class="flex items-center gap-2">
				{#if mode === 'malts'}
					<button
						class="btn btn-ghost btn-xs text-base-content/30 hover:text-base-content/60"
						title={t('common.edit')}
						onclick={startEdit}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
							<path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
						</svg>
					</button>
					<button
						class="btn btn-ghost btn-xs text-base-content/30 hover:text-primary/70"
						title={t('card.putOnStill')}
						onclick={() => handleSetStatus(malt.id, MaltStatus.ON_STILL)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="18" cy="5" r="3"/>
							<circle cx="6" cy="12" r="3"/>
							<circle cx="18" cy="19" r="3"/>
							<line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
							<line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
						</svg>
					</button>
				{:else if mode === 'still'}
					<button
						class="btn btn-ghost btn-xs text-base-content/30 hover:text-warning"
						title={t('card.takeOffStill')}
						onclick={() => handleSetStatus(malt.id, MaltStatus.MALT_HOUSE)}
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4"/>
							<polyline points="16 17 21 12 16 7"/>
							<line x1="21" y1="12" x2="9" y2="12"/>
						</svg>
					</button>
				{/if}
				{#if mode !== 'drawback'}
					{#if showConfirm}
						<button class="btn btn-ghost btn-xs text-error" onclick={() => handleDelete(malt.id)}>{t('common.delete')}</button>
						<button class="btn btn-ghost btn-xs text-base-content/40" onclick={() => (showConfirm = false)}>{t('common.cancel')}</button>
					{:else}
						<button class="btn btn-ghost btn-xs text-base-content/30 hover:text-error" title={t('common.delete')} onclick={() => (showConfirm = true)}>
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<polyline points="3 6 5 6 21 6"/>
								<path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
							</svg>
						</button>
					{/if}
				{/if}
				<span class="text-xs text-base-content/30">{formatDate(malt.createdAt)}</span>
			</div>
		</div>

		<p class="text-[15px] font-medium leading-relaxed">{malt.summary}</p>

		{#if malt.context}
			<p class="mt-1.5 text-sm text-base-content/50 leading-relaxed">{malt.context}</p>
		{/if}

		{#if malt.memo}
			<p class="mt-1.5 text-sm text-base-content/30 italic leading-relaxed">{malt.memo}</p>
		{/if}

		{#if mode === 'still'}
			<div class="flex justify-end mt-2">
				<button
					class="btn btn-primary btn-xs gap-1"
					onclick={handleQueue}
				disabled={queueing}
				>
					{#if queueing}
						<span class="loading loading-spinner loading-xs"></span>
					{:else}
						<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
							<polyline points="17 8 12 3 7 8"/>
							<line x1="12" y1="3" x2="12" y2="15"/>
						</svg>
						{t('card.queue')}
					{/if}
				</button>
			</div>
		{:else if mode === 'drawback'}
			<div class="flex justify-end mt-2">
				<button
					class="btn btn-warning btn-xs gap-1"
					onclick={handleDrawBack}
					disabled={drawingBack}
				>
					{#if drawingBack}
						<span class="loading loading-spinner loading-xs"></span>
					{:else}
						<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<polyline points="1 4 1 10 7 10"/>
							<path d="M3.51 15a9 9 0 102.13-9.36L1 10"/>
						</svg>
						{t('card.drawBack')}
					{/if}
				</button>
			</div>
		{/if}
	</article>
{/if}
