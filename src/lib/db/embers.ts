import { getDb } from './porter';
import type { Ember } from './types';

export async function getAllEmbers(): Promise<Ember[]> {
	const db = await getDb();
	let embers = await db.getAll('embers');
    return embers.sort(
        (a, b) => new Date(a.createdAt ?? 0).getTime() - new Date(b.createdAt ?? 0).getTime()
    );
}

export async function getEmber(id: string): Promise<Ember | undefined> {
	const db = await getDb();
	return db.get('embers', id);
}

export async function createEmber(
	data: Omit<Ember, 'id' | 'createdAt' | 'updatedAt'>
): Promise<Ember> {
	const db = await getDb();
	const now = new Date().toISOString();

	const ember: Ember = {
		id: crypto.randomUUID(),
		createdAt: now,
		updatedAt: now,
		...data
	};

	await db.add('embers', ember);
	return ember;
}

export async function updateEmber(ember: Ember): Promise<void> {
	const db = await getDb();

	await db.put('embers', {
		...ember,
		updatedAt: new Date().toISOString()
	});
}

export async function deleteEmber(id: string): Promise<void> {
	const db = await getDb();
	await db.delete('embers', id);
}
