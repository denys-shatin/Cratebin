import { api } from '$lib/api';
import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params, url }) => {
	const password = url.searchParams.get('password') || undefined;

	try {
		const content = await api.getSnippetRaw(params.id, password);
		
		return new Response(content, {
			headers: {
				'Content-Type': 'text/plain; charset=utf-8'
			}
		});
	} catch (e) {
		throw error(404, 'Snippet not found');
	}
};
