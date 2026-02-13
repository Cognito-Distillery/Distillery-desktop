<script lang="ts">
	import {
		sendOtp,
		verifyOtp,
		getAuthError,
		isOtpSent,
		isOtpLoading,
		isVerifyLoading,
		resetLoginForm
	} from '$lib/stores/auth.svelte';
	import { t } from '$lib/i18n/index.svelte';

	let emailInput = $state('');
	let otpInput = $state('');

	async function handleSendOtp(e: Event) {
		e.preventDefault();
		if (!emailInput.trim()) return;
		await sendOtp(emailInput.trim());
	}

	async function handleVerifyOtp(e: Event) {
		e.preventDefault();
		if (!otpInput.trim()) return;
		await verifyOtp(otpInput.trim());
	}

	function handleBack() {
		otpInput = '';
		resetLoginForm();
	}
</script>

<div class="flex h-screen items-center justify-center">
	<div class="w-full max-w-sm flex flex-col gap-6 px-6">
		<div class="text-center flex flex-col items-center">
			<svg width="96" height="96" viewBox="0 0 200 200" fill="none" class="mb-4">
				<defs>
					<linearGradient id="copLogin" x1="50" y1="150" x2="150" y2="50" gradientUnits="userSpaceOnUse">
						<stop offset="0%" stop-color="#92400E"/><stop offset="50%" stop-color="#F59E0B"/><stop offset="100%" stop-color="#FEF3C7"/>
					</linearGradient>
					<linearGradient id="liqLogin" x1="100" y1="100" x2="100" y2="180" gradientUnits="userSpaceOnUse">
						<stop offset="0%" stop-color="#FBBF24" stop-opacity="0.1"/><stop offset="100%" stop-color="#F59E0B" stop-opacity="0.6"/>
					</linearGradient>
				</defs>
				<path d="M60 140 C60 110, 80 100, 90 95 V 70" stroke="url(#copLogin)" stroke-width="4" stroke-linecap="round" fill="none"/>
				<path d="M110 70 V 95 C120 100, 140 110, 140 140 C140 165, 122 185, 100 185 C78 185, 60 165, 60 140 Z" stroke="url(#copLogin)" stroke-width="4" stroke-linecap="round" fill="url(#liqLogin)"/>
				<path d="M90 70 C90 60, 90 50, 100 40 C110 30, 130 30, 145 45 L 155 55" stroke="url(#copLogin)" stroke-width="4" stroke-linecap="round" fill="none"/>
				<path d="M155 65 C155 65, 150 72, 150 76 A 5 5 0 0 0 160 76 C160 72, 155 65, 155 65 Z" fill="#FCD34D"/>
				<path d="M75 130 Q 80 110, 95 105" stroke="#FEF3C7" stroke-width="2" stroke-opacity="0.4" stroke-linecap="round"/>
			</svg>
			<h1 class="text-2xl font-bold tracking-tight">Cognito Distillery</h1>
			<p class="text-sm text-base-content/40 mt-1">Distill your thoughts</p>
		</div>

		{#if !isOtpSent()}
			<form class="flex flex-col gap-3" onsubmit={handleSendOtp}>
				<input
					type="email"
					class="input w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30"
					placeholder={t('login.email')}
					bind:value={emailInput}
					required
				/>
				<button
					type="submit"
					class="btn btn-primary w-full"
					disabled={isOtpLoading() || !emailInput.trim()}
				>
					{#if isOtpLoading()}
						<span class="loading loading-spinner loading-sm"></span>
					{:else}
						{t('login.sendOtp')}
					{/if}
				</button>
			</form>
		{:else}
			<form class="flex flex-col gap-3" onsubmit={handleVerifyOtp}>
				<p class="text-sm text-base-content/50 text-center">
					{t('login.otpSent')}
				</p>
				<input
					type="text"
					class="input w-full bg-white/[0.12] border-white/[0.18] focus:border-primary placeholder:text-base-content/30 text-center tracking-[0.3em] text-lg"
					placeholder="000000"
					bind:value={otpInput}
					maxlength={6}
					inputmode="numeric"
					autocomplete="one-time-code"
				/>
				<button
					type="submit"
					class="btn btn-primary w-full"
					disabled={isVerifyLoading() || otpInput.trim().length < 6}
				>
					{#if isVerifyLoading()}
						<span class="loading loading-spinner loading-sm"></span>
					{:else}
						{t('login.verify')}
					{/if}
				</button>
				<button type="button" class="btn btn-ghost btn-sm" onclick={handleBack}>
					{t('login.tryOther')}
				</button>
			</form>
		{/if}

		{#if getAuthError()}
			<p class="text-sm text-error text-center">{getAuthError()}</p>
		{/if}
	</div>
</div>
