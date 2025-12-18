import { PUBLIC_API_URL } from '$env/static/public';

export interface CreateSnippetData {
	content: string;
	visibility: 'public' | 'unlisted' | 'private';
	ttl?: string;
	password?: string;
}

export interface CreateSnippetResponse {
	id: string;
	url: string;
	delete_token: string;
	expires_at?: string;
}

export interface Snippet {
	id: string;
	content: string;
	visibility: 'public' | 'unlisted' | 'private';
	expires_at?: string;
	created_at: string;
	size: number;
}

class ApiClient {
	private baseUrl: string;

	constructor(baseUrl: string) {
		this.baseUrl = baseUrl;
	}

	async createSnippet(data: CreateSnippetData): Promise<CreateSnippetResponse> {
		const response = await fetch(`${this.baseUrl}/snippets`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		});

		if (!response.ok) {
			const error = await response.json();
			throw new Error(error.error?.message || 'Failed to create snippet');
		}

		return response.json();
	}

	async getSnippet(id: string, password?: string): Promise<Snippet> {
		const url = new URL(`${this.baseUrl}/snippets/${id}`);
		if (password) {
			url.searchParams.set('password', password);
		}

		const response = await fetch(url.toString());

		if (!response.ok) {
			if (response.status === 403) {
				throw new Error('Incorrect password');
			}
			if (response.status === 404) {
				throw new Error('Snippet not found or expired');
			}
			throw new Error('Failed to fetch snippet');
		}

		return response.json();
	}

	async getSnippetRaw(id: string, password?: string): Promise<string> {
		const url = new URL(`${this.baseUrl}/snippets/${id}/raw`);
		if (password) {
			url.searchParams.set('password', password);
		}

		const response = await fetch(url.toString());

		if (!response.ok) {
			throw new Error('Failed to fetch snippet');
		}

		return response.text();
	}

	async deleteSnippet(id: string, deleteToken: string): Promise<void> {
		const response = await fetch(`${this.baseUrl}/snippets/${id}`, {
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ delete_token: deleteToken })
		});

		if (!response.ok) {
			throw new Error('Failed to delete snippet');
		}
	}
}

export const api = new ApiClient(PUBLIC_API_URL || 'http://localhost:8080');
