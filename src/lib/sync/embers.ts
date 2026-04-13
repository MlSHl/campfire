import { syncEmbers } from '$lib/api/embers';
import { getAllEmbers, replaceAllEmbers } from '$lib/db/embers';
import { apiToLocalEmber, localToApiEmber } from './emberMappers';

export async function runEmberSync() {
	const localEmbers = await getAllEmbers();
	const apiPayload = localEmbers.map(localToApiEmber);

    const response = await syncEmbers(apiPayload);
    console.log('sync response:', response);

    if (response.status !== 'Ok') {
        throw new Error(`Ember sync failed: ${response.status}`);
    }

	const canonicalLocalEmbers = response.embers.map(apiToLocalEmber);
	await replaceAllEmbers(canonicalLocalEmbers);

	return response;
}
