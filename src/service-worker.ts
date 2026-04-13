import { build, files, prerendered, version } from '$service-worker';

const CACHE = `campfire-${version}`;
const APP_SHELL = '/200.html';

const ASSETS = [...build, ...files, ...prerendered].filter(
	(asset) => !asset.startsWith('/.')
);

self.addEventListener('install', (event: ExtendableEvent) => {
	event.waitUntil(
		(async () => {
			const cache = await caches.open(CACHE);
			await cache.addAll(ASSETS);
			await self.skipWaiting();
		})()
	);
});

self.addEventListener('activate', (event: ExtendableEvent) => {
	event.waitUntil(
		(async () => {
			const keys = await caches.keys();

			await Promise.all(
				keys
					.filter((key) => key !== CACHE)
					.map((key) => caches.delete(key))
			);

			await self.clients.claim();
		})()
	);
});

self.addEventListener('fetch', (event: FetchEvent) => {
	const { request } = event;

	if (request.method !== 'GET') return;

	if (request.mode === 'navigate') {
		event.respondWith(
			(async () => {
				try {
					return await fetch(request);
				} catch {
					const cache = await caches.open(CACHE);
					const cached = await cache.match(APP_SHELL);

					if (cached) return cached;

					return new Response('Offline', {
						status: 503,
						statusText: 'Service Unavailable'
					});
				}
			})()
		);
		return;
	}

	event.respondWith(
		(async () => {
			const cached = await caches.match(request);
			if (cached) return cached;

			try {
				return await fetch(request);
			} catch {
				return new Response('Offline', {
					status: 503,
					statusText: 'Service Unavailable'
				});
			}
		})()
	);
});
