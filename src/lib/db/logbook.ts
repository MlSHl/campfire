import { getDb } from './porter';
import type { Logbook } from './types';

export async function getAllLogbooks(): Promise<Logbook[]> {
	const db = await getDb();
	return db.getAll('logbook');
}

export async function getLogbook(id: string): Promise<Logbook | undefined> {
	const db = await getDb();
	return db.get('logbook', id);
}

export async function createLogbook(
	data: Omit<Logbook, 'id' | 'createdAt' | 'updatedAt'>
): Promise<Logbook> {
	const db = await getDb();
	const now = new Date().toISOString();

	const logbook: Logbook = {
		id: crypto.randomUUID(),
		createdAt: now,
		updatedAt: now,
		...data
	};

	await db.add('logbook', logbook);
	return logbook;
}

export async function updateLogbook(logbook: Logbook): Promise<void> {
	const db = await getDb();

	await db.put('logbook', {
		...logbook,
		updatedAt: new Date().toISOString()
	});
}

export async function deleteLogbook(id: string): Promise<void> {
	const db = await getDb();
	await db.delete('logbook', id);
}
