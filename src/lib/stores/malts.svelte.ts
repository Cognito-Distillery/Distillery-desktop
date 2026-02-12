import { invoke } from '@tauri-apps/api/core';
import { MaltStatus, type Malt, type MaltType } from '$lib/types';
import { getLocale } from '$lib/i18n/index.svelte';
import { friendlyError } from '$lib/utils/error';
import { showToast } from '$lib/stores/toast.svelte';

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
		showToast(friendlyError(e));
	} finally {
		loading = false;
	}
}

export async function setMaltStatus(id: string, status: MaltStatus) {
	try {
		await invoke('set_malt_status', { id, status });
	} catch (e) {
		showToast(friendlyError(e));
		throw e;
	}
}

export async function addMalt(data: {
	type: MaltType;
	summary: string;
	context: string;
	memo: string;
}) {
	try {
		await invoke<Malt>('add_malt', {
			maltType: data.type,
			summary: data.summary,
			context: data.context,
			memo: data.memo
		});
	} catch (e) {
		showToast(friendlyError(e));
		throw e;
	}
}

export async function deleteMalt(id: string) {
	try {
		await invoke('delete_malt', { id });
	} catch (e) {
		showToast(friendlyError(e));
		throw e;
	}
}

export async function updateMalt(
	id: string,
	data: Partial<Pick<Malt, 'type' | 'summary' | 'context' | 'memo'>>
) {
	try {
		await invoke<Malt>('update_malt', {
			id,
			maltType: data.type,
			summary: data.summary,
			context: data.context,
			memo: data.memo
		});
	} catch (e) {
		showToast(friendlyError(e));
		throw e;
	}
}

export async function loadQueuedMalts() {
	loading = true;
	try {
		malts = await invoke<Malt[]>('get_queued_malts', { lang: getLocale() });
	} catch (e) {
		showToast(friendlyError(e));
	} finally {
		loading = false;
	}
}

export async function drawBackMalt(id: string, serverId: string): Promise<void> {
	try {
		await invoke('malt_draw_back', { id, serverId, lang: getLocale() });
	} catch (e) {
		showToast(friendlyError(e));
		throw e;
	}
}

export async function queueMalt(id: string): Promise<void> {
	try {
		await invoke('malt_queue', { id, lang: getLocale() });
	} catch (e) {
		showToast(friendlyError(e));
		throw e;
	}
}

export async function queueMaltsBatch(): Promise<number> {
	try {
		return await invoke<number>('malts_queue_batch', { lang: getLocale() });
	} catch (e) {
		showToast(friendlyError(e));
		throw e;
	}
}
