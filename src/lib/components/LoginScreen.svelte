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
			<svg width="96" height="96" viewBox="0 0 24 24" fill="none" class="mb-4">
				<path d="M13 16A5 5 0 1 1 3 16C3 13.5 4.5 11.5 6 10.2V6A2 2 0 0 1 10 6V10.2C11.5 11.5 13 13.5 13 16Z" fill="currentColor" class="text-primary"/>
				<path d="M9 5C12.5 5 18 6.5 18 11V12" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" class="text-primary"/>
				<path d="M18 14.5L16.2 17.2C15.6 18.1 16.3 19.5 17.5 19.5H18.5C19.7 19.5 20.4 18.1 19.8 17.2L18 14.5Z" fill="currentColor" class="text-primary"/>
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
