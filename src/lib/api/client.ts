export async function apiFetch<T> (
    path: string,
    init: RequestInit = {}
): Promise<T> {
    const response = await fetch(path, {
        credentials:'same-origin',
        ...init,
        headers: {
            'Content-Type': 'application/json',
            ...(init.headers ?? {})
        }
    });

    let data: unknown = null;

    const contentType = response.headers.get('content-type') ?? '';
	if (contentType.includes('application/json')) {
		data = await response.json();
	} else {
		data = await response.text();
	}

	if (!response.ok) {
		const message =
			typeof data === 'object' && data && 'message' in data
				? String((data as { message?: unknown }).message)
				: `Request failed with status ${response.status}`;

		throw new Error(message);
	}

	return data as T;
}
