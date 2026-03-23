import { build, files, prerendered, version } from '$service-worker';

const CACHE = `campfire-${version}`;
const ASSETS = [...build, ...files, ...prerendered];
const APP_SHELL = new Request('/', { credentials: 'same-origin' });

self.addEventListener('install', (event: ExtendableEvent) => {
	event.waitUntil(
		(async () => {
			const cache = await caches.open(CACHE);

			await cache.addAll(ASSETS);

			// Explicitly cache the app shell for offline navigations
			const response = await fetch(APP_SHELL);
			await cache.put(APP_SHELL, response.clone());

			await self.skipWaiting();
		})()
	);
});

self.addEventListener('activate', (event: ExtendableEvent) => {
	event.waitUntil(
		(async () => {
			const keys = await caches.keys();
			await Promise.all(
				keys.filter((key) => key !== CACHE).map((key) => caches.delete(key))
			);
			await self.clients.claim();
		})()
	);
});

self.addEventListener('fetch', (event: FetchEvent) => {
	const { request } = event;

	if (request.method !== 'GET') return;

	// For page navigations, serve the cached app shell
	if (request.mode === 'navigate') {
		event.respondWith(
			(async () => {
				const cache = await caches.open(CACHE);
				const cached = await cache.match(APP_SHELL);
				if (cached) return cached;

				return fetch(request);
			})()
		);
		return;
	}

	// For normal assets, use cache first
	event.respondWith(
		(async () => {
			const cached = await caches.match(request);
			if (cached) return cached;

			return fetch(request);
		})()
	);
});
