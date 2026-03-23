import { getDb } from './porter';
import type { Log } from './types';

export async function getAllLogs(): Promise<Log[]> {
	const db = await getDb();
	let logs = await db.getAll('logbook');
    return logs.sort(
        (a, b) => new Date(b.createdAt ?? 0).getTime() - new Date(a.createdAt ?? 0).getTime()
    );
}

export async function getLog(id: string): Promise<Log | undefined> {
	const db = await getDb();
	return db.get('logbook', id);
}

export async function createLog(
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

export async function updateLog(logbook: Log): Promise<void> {
	const db = await getDb();

	await db.put('logbook', {
		...logbook,
		updatedAt: new Date().toISOString()
	});
}

export async function deleteLog(id: string): Promise<void> {
	const db = await getDb();
	await db.delete('logbook', id);
}

export async function updateLastVisitedAt(id: string): Promise<void> {
    const db = await getDb();
    const logbook = await db.get('logbook', id);
    await db.put('logbook', {
        ...logbook,
        lastVisitedAt: new Date().toISOString()
    });
}

export async function getLastOpenedLog(): Promise<Log | undefined> {
    const db = await getDb();
    const logs = await db.getAll('logbook');
    const visitedLogs = logs.filter((log) => log.lastVisitedAt);

    visitedLogs.sort(
        (a, b) => new Date(b.lastVisitedAt ?? 0).getTime() - new Date(a.lastVisitedAt ?? 0).getTime()
    );

    return visitedLogs[0]; 
}
