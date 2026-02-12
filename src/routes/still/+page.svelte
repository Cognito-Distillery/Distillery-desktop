<script lang="ts">
	import MaltList from '$lib/components/MaltList.svelte';
	import DistillTimer from '$lib/components/DistillTimer.svelte';
	import { getMalts, queueMaltsBatch, loadMalts } from '$lib/stores/malts.svelte';
	import { MaltStatus } from '$lib/types';
	import { t } from '$lib/i18n/index.svelte';

	let queueing = $state(false);

	async function handleQueueAll() {
		queueing = true;
		try {
			await queueMaltsBatch();
			await loadMalts(MaltStatus.ON_STILL);
		} catch {
			// toast already shown by store
		} finally {
			queueing = false;
		}
	}
</script>

<div class="max-w-2xl mx-auto flex flex-col gap-8">
	<div class="flex flex-col gap-4">
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-2xl font-semibold tracking-tight">{t('still.title')}</h1>
				<p class="text-sm text-base-content/40 mt-1">{t('still.subtitle')}</p>
			</div>
			{#if getMalts().length > 0}
				<button
					class="btn btn-primary btn-sm gap-2"
					onclick={handleQueueAll}
					disabled={queueing}
				>
					{#if queueing}
						<span class="loading loading-spinner loading-sm"></span>
					{:else}
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
							<polyline points="17 8 12 3 7 8"/>
							<line x1="12" y1="3" x2="12" y2="15"/>
						</svg>
						{t('still.queueAll')}
					{/if}
				</button>
			{/if}
		</div>
		<DistillTimer />
	</div>
	<MaltList mode="still" />
</div>
