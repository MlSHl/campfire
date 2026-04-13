import { apiFetch } from '$lib/api/client';

export type ApiEmber = {
	id: string;
	name: string;
	hours_today: number;
	hours_this_week: number;
	hours_total: number;
	created_at: string;
	updated_at: string;
	deleted_at: string | null;
};

export type EmberSyncRequest = {
	embers: ApiEmber[];
	since: string | null;
};

export type EmberSyncResponse = {
	status: string;
	embers: ApiEmber[];
	server_time: string;
};

export function syncEmbers(embers: ApiEmber[]) {
	return apiFetch<EmberSyncResponse>('/api/embers', {
		method: 'POST',
		body: JSON.stringify({
			embers,
			since: null
		} satisfies EmberSyncRequest)
	});
}
