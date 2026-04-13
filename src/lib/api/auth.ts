import { apiFetch } from "$lib/api/client";

type LoginResponse = {
    status: string,
    message: string
};

export function login(email: string, password: string) {
    return apiFetch<LoginResponse>('/api/session', {
        method: 'POST',
        body: JSON.stringify({email, password})
    });
}

