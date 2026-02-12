export type MaltType = '결정' | '문제' | '인사이트' | '질문';

export enum MaltStatus {
	MALT_HOUSE = 'MALT_HOUSE',
	ON_STILL = 'ON_STILL',
	DISTILLED_READY = 'DISTILLED_READY',
	DISTILLED = 'DISTILLED',
	CASKED = 'CASKED'
}

export interface Malt {
	id: string;
	type: MaltType;
	summary: string;
	context: string;
	memo: string;
	status: MaltStatus;
	serverId: string | null;
	syncedAt: number | null;
	createdAt: number;
	updatedAt: number;
}
