import { invoke } from '@tauri-apps/api/core';
import { getLocale } from '$lib/i18n/index.svelte';
import { friendlyError } from '$lib/utils/error';

let authenticated = $state(false);
let loading = $state(true);
let email = $state('');
let error = $state('');
let otpSent = $state(false);
let otpLoading = $state(false);
let verifyLoading = $state(false);

export function isAuthenticated() {
	return authenticated;
}

export function isAuthLoading() {
	return loading;
}

export function getEmail() {
	return email;
}

export function getAuthError() {
	return error;
}

export function isOtpSent() {
	return otpSent;
}

export function isOtpLoading() {
	return otpLoading;
}

export function isVerifyLoading() {
	return verifyLoading;
}

export async function checkAuth() {
	loading = true;
	try {
		const result = await invoke<string | null>('auth_check', { lang: getLocale() });
		if (result) {
			email = result;
			authenticated = true;
		} else {
			authenticated = false;
			email = '';
		}
	} catch {
		authenticated = false;
		email = '';
	} finally {
		loading = false;
	}
}

export async function sendOtp(userEmail: string) {
	otpLoading = true;
	error = '';
	try {
		await invoke('auth_send_otp', { email: userEmail, lang: getLocale() });
		email = userEmail;
		otpSent = true;
	} catch (e) {
		error = friendlyError(e);
	} finally {
		otpLoading = false;
	}
}

export async function verifyOtp(otp: string) {
	verifyLoading = true;
	error = '';
	try {
		await invoke('auth_verify_otp', { email, otp, lang: getLocale() });
		authenticated = true;
	} catch (e) {
		error = friendlyError(e);
	} finally {
		verifyLoading = false;
	}
}

export async function logout() {
	try {
		await invoke('auth_logout');
	} catch {
		// ignore — clear local state regardless
	}
	authenticated = false;
	email = '';
	resetLoginForm();
}

export function forceLogout() {
	authenticated = false;
	email = '';
	resetLoginForm();
}

export function resetLoginForm() {
	otpSent = false;
	error = '';
	otpLoading = false;
	verifyLoading = false;
}
