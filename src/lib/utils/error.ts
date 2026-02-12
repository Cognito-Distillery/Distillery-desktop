import { t } from '$lib/i18n/index.svelte';
import { forceLogout } from '$lib/stores/auth.svelte';

export function friendlyError(e: unknown): string {
	const msg = String(e);

	if (msg.includes('error sending request') || msg.includes('error trying to connect')) {
		return t('error.network');
	}
	if (msg.includes('LOGIN_REQUIRED') || msg.includes('로그인이 필요합니다')) {
		forceLogout();
		return t('error.loginRequired');
	}
	if (msg.includes('NOT_FOUND') || msg.includes('찾을 수 없습니다')) {
		return t('error.notFound');
	}
	if (msg.includes('401') || msg.includes('verify_otp failed')) {
		return t('error.invalidOtp');
	}
	if (msg.includes('403')) {
		forceLogout();
		return t('error.loginRequired');
	}
	if (msg.includes('404')) {
		return t('error.notFound');
	}

	return t('error.unknown');
}
