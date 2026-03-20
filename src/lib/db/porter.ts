import { browser } from '$app/environment';
import { openDB, type DBSchema, type IDBPDatabase } from 'idb';
import type { Logbook, Footstep, Ember } from './types';

interface Backpack extends DBSchema {
	logbook: {
		key: string;
		value: Logbook;
	};
	footsteps: {
		key: string;
		value: Footstep;
        indexes: {
            'by-completed': boolean;
        }
	};
	embers: {
		key: string;
		value: Ember;
	};
}

let dbPromise: Promise<IDBPDatabase<Backpack>> | null = null;

export function getDb() {
	if (!browser) {
		throw new Error('IndexedDB is only available in the browser');
	}

	if (!dbPromise) {
		dbPromise = openDB<Backpack>('campfire-db', 1, {
			upgrade(db) {
				if (!db.objectStoreNames.contains('logbook')) {
					db.createObjectStore('logbook', { keyPath: 'id' });
				}
				if (!db.objectStoreNames.contains('footsteps')) {
					const store = db.createObjectStore('footsteps', { keyPath: 'id' });
                    store.createIndex('by-completed', 'completed');
				}
				if (!db.objectStoreNames.contains('embers')) {
					db.createObjectStore('embers', { keyPath: 'id' });
				}
			}
		});
	}

	return dbPromise;
}
