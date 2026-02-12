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
		<div class="text-center">
			<h1 class="text-2xl font-bold tracking-tight">Distillery</h1>
			<p class="text-sm text-base-content/40 mt-1">{t('login.title')}</p>
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
