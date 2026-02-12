<script lang="ts">
	import { t } from '$lib/i18n/index.svelte';
	import { getLocale } from '$lib/i18n/index.svelte';

	const CYCLE_MS = 12 * 60 * 60 * 1000;
	const localeTag: Record<string, string> = { ko: 'ko-KR', en: 'en-US' };

	let now = $state(Date.now());

	$effect(() => {
		now = Date.now();
		const id = setInterval(() => { now = Date.now(); }, 30_000);
		return () => clearInterval(id);
	});

	let distill = $derived.by(() => {
		const d = new Date(now);
		const h = d.getHours();
		const today = new Date(d);

		let next: Date;
		let prev: Date;

		if (h < 12) {
			prev = new Date(today.setHours(0, 0, 0, 0));
			next = new Date(new Date(d).setHours(12, 0, 0, 0));
		} else {
			prev = new Date(today.setHours(12, 0, 0, 0));
			next = new Date(new Date(d).setHours(24, 0, 0, 0));
		}

		const remaining = next.getTime() - now;
		const progress = (now - prev.getTime()) / CYCLE_MS;
		const rh = Math.floor(remaining / 3_600_000);
		const rm = Math.floor((remaining % 3_600_000) / 60_000);

		const tag = localeTag[getLocale()] ?? 'ko-KR';
		const nextLabel = next.toLocaleDateString(tag, {
			month: 'short',
			day: 'numeric'
		}) + ' ' + next.toLocaleTimeString(tag, {
			hour: '2-digit',
			minute: '2-digit',
			hour12: false
		});

		return { progress: Math.min(progress, 1), rh, rm, nextLabel };
	});
</script>

<div class="flex items-center gap-3">
	<span class="text-[11px] text-base-content/30 shrink-0">{t('timer.label')}</span>
	<div class="flex-1 h-1 rounded-full bg-base-content/[0.08] overflow-hidden">
		<div
			class="h-full rounded-full bg-primary transition-[width] duration-500"
			style="width: {distill.progress * 100}%"
		></div>
	</div>
	<span class="text-[11px] text-base-content/40 font-medium tabular-nums shrink-0">
		{distill.rh}{t('timer.hours')} {distill.rm}{t('timer.minutes')} ({distill.nextLabel})
	</span>
</div>
