import { invoke } from '@tauri-apps/api/core';
import { MaltStatus, type Malt, type MaltType } from '$lib/types';
import { getLocale } from '$lib/i18n/index.svelte';

let malts = $state<Malt[]>([]);
let searchQuery = $state('');
let loading = $state(false);

export function getMalts() {
	return malts;
}

export function isLoading() {
	return loading;
}

export function getSearchQuery() {
	return searchQuery;
}

export function setSearchQuery(q: string) {
	searchQuery = q;
}

export async function loadMalts(status: string, query?: string) {
	loading = true;
	try {
		const q = query?.trim() || searchQuery.trim() || undefined;
		malts = await invoke<Malt[]>('get_malts_by_status', { status, query: q });
	} catch (e) {
		console.error('loadMalts failed:', e);
	} finally {
		loading = false;
	}
}

export async function setMaltStatus(id: string, status: MaltStatus) {
	await invoke('set_malt_status', { id, status });
}

export async function addMalt(data: {
	type: MaltType;
	summary: string;
	context: string;
	memo: string;
}) {
	await invoke<Malt>('add_malt', {
		maltType: data.type,
		summary: data.summary,
		context: data.context,
		memo: data.memo
	});
}

export async function deleteMalt(id: string) {
	await invoke('delete_malt', { id });
}

export async function updateMalt(
	id: string,
	data: Partial<Pick<Malt, 'type' | 'summary' | 'context' | 'memo'>>
) {
	await invoke<Malt>('update_malt', {
		id,
		maltType: data.type,
		summary: data.summary,
		context: data.context,
		memo: data.memo
	});
}

export async function loadQueuedMalts() {
	loading = true;
	try {
		malts = await invoke<Malt[]>('get_queued_malts', { lang: getLocale() });
	} catch (e) {
		console.error('loadQueuedMalts failed:', e);
	} finally {
		loading = false;
	}
}

export async function drawBackMalt(id: string, serverId: string): Promise<void> {
	await invoke('malt_draw_back', { id, serverId, lang: getLocale() });
}

export async function queueMalt(id: string): Promise<void> {
	await invoke('malt_queue', { id, lang: getLocale() });
}

export async function queueMaltsBatch(): Promise<number> {
	return await invoke<number>('malts_queue_batch', { lang: getLocale() });
}
