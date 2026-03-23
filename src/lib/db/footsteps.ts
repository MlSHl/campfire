import { getDb } from './porter';
import type { Footstep } from './types';

export async function getAllFootsteps(): Promise<Footstep[]> {
	const db = await getDb();
	let footsteps = await db.getAll('footsteps');
    return footsteps.sort(
        (a, b) => new Date(a.createdAt ?? 0).getTime() - new Date(b.createdAt ?? 0).getTime()
    );
}

export async function getFootstep(id: string | undefined): Promise<Footstep | undefined> {
	const db = await getDb();
    if (id) {
	    return db.get('footsteps', id);
    }
    return undefined;
}

export async function createFootstep(
	data: Omit<Footstep, 'id' | 'createdAt' | 'updatedAt'>
): Promise<Footstep> {
	const db = await getDb();
	const now = new Date().toISOString();

	const footstep: Footstep = {
		id: crypto.randomUUID(),
		createdAt: now,
		updatedAt: now,
		...data
	};

	await db.add('footsteps', footstep);
	return footstep;
}

export async function updateFootstep(footstep: Footstep): Promise<void> {
	const db = await getDb();

	await db.put('footsteps', {
		...footstep,
		updatedAt: new Date().toISOString()
	});
}

export async function deleteFootstep(id: string): Promise<void> {
	const db = await getDb();
	await db.delete('footsteps', id);
}

export async function toggleCompleteFootstep(id: string): Promise<void> {
	const db = await getDb();
	const footstep = await db.get('footsteps', id);

	if (!footstep) return;

	await db.put('footsteps', {
		...footstep,
		completed: footstep.completed ? 0 : 1,
		updatedAt: new Date().toISOString()
	});
}

export async function getCompletedCount(): Promise<number> {
	const db = await getDb();
	const tx = db.transaction('footsteps', 'readonly');
	const count = await tx.store.index('by-completed').count(1);
	await tx.done;
	return count;
}

export async function deleteCompletedSteps(): Promise<void> {
	const db = await getDb();
	const tx = db.transaction('footsteps', 'readwrite');
	const index = tx.store.index('by-completed');

	let cursor = await index.openCursor(1);

	while (cursor) {
		await cursor.delete();
		cursor = await cursor.continue();
	}

	await tx.done;
}
