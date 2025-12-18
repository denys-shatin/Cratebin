<script lang="ts">
	import { api, type CreateSnippetData } from '$lib/api';
	import { goto } from '$app/navigation';

	let content = '';
	let visibility: 'public' | 'unlisted' | 'private' = 'public';
	let ttl = '24h';
	let password = '';
	let error = '';
	let loading = false;

	async function handleSubmit() {
		error = '';
		
		if (!content.trim()) {
			error = 'Content cannot be empty';
			return;
		}

		loading = true;

		try {
			const data: CreateSnippetData = {
				content,
				visibility,
				ttl: ttl === 'never' ? undefined : ttl,
				password: visibility === 'private' && password ? password : undefined
			};

			const response = await api.createSnippet(data);
			goto(`/success?id=${response.id}&token=${response.delete_token}`);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to create snippet';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Cratebin - Create Snippet</title>
</svelte:head>

<div class="container">
	<header>
		<h1>📦 Cratebin</h1>
		<p>Share code snippets, logs, and text</p>
	</header>

	<main>
		<form on:submit|preventDefault={handleSubmit}>
			<div class="form-group">
				<label for="content">Content</label>
				<textarea
					id="content"
					bind:value={content}
					placeholder="Paste your code, logs, or text here..."
					rows="18"
					required
				></textarea>
			</div>

			<div class="options">
				<div class="form-group">
					<label for="visibility">Visibility</label>
					<select id="visibility" bind:value={visibility}>
						<option value="public">Public</option>
						<option value="unlisted">Unlisted</option>
						<option value="private">Private</option>
					</select>
				</div>

				<div class="form-group">
					<label for="ttl">Expires</label>
					<select id="ttl" bind:value={ttl}>
						<option value="1h">1 hour</option>
						<option value="24h">24 hours</option>
						<option value="7d">7 days</option>
						<option value="never">Never</option>
					</select>
				</div>

				{#if visibility === 'private'}
					<div class="form-group">
						<label for="password">Password (optional)</label>
						<input
							type="password"
							id="password"
							bind:value={password}
							placeholder="Enter password"
						/>
					</div>
				{/if}
			</div>

			{#if error}
				<div class="error">{error}</div>
			{/if}

			<button type="submit" disabled={loading}>
				{loading ? 'Creating...' : 'Create Snippet'}
			</button>
		</form>
	</main>
</div>

<style>
	.container {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	header {
		text-align: center;
		margin-bottom: 2rem;
	}

	h1 {
		font-size: 2.5rem;
		margin: 0;
		color: #58a6ff;
		font-weight: 600;
	}

	header p {
		color: #8b949e;
		margin: 0.5rem 0 0 0;
		font-size: 1rem;
	}

	main {
		background: #161b22;
		border: 1px solid #30363d;
		border-radius: 6px;
		padding: 2rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		color: #c9d1d9;
		font-size: 0.95rem;
		font-weight: 500;
	}

	textarea {
		width: 100%;
		padding: 0.75rem;
		background: #0d1117;
		border: 1px solid #30363d;
		border-radius: 6px;
		color: #c9d1d9;
		font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
		font-size: 0.9rem;
		resize: vertical;
		box-sizing: border-box;
	}

	textarea:focus {
		outline: none;
		border-color: #58a6ff;
	}

	.options {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 1rem;
		margin-bottom: 1.25rem;
	}

	select,
	input[type='password'] {
		width: 100%;
		padding: 0.625rem;
		background: #0d1117;
		border: 1px solid #30363d;
		border-radius: 6px;
		color: #c9d1d9;
		font-size: 0.9rem;
		box-sizing: border-box;
	}

	select:focus,
	input:focus {
		outline: none;
		border-color: #58a6ff;
	}

	button {
		width: 100%;
		padding: 0.75rem;
		background: #238636;
		color: #ffffff;
		border: none;
		border-radius: 6px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
	}

	button:hover:not(:disabled) {
		background: #2ea043;
	}

	button:disabled {
		background: #21262d;
		color: #8b949e;
		cursor: not-allowed;
	}

	.error {
		padding: 0.75rem;
		background: #3d1319;
		border: 1px solid #da3633;
		border-radius: 6px;
		color: #f85149;
		margin-bottom: 1rem;
	}

	@media (max-width: 600px) {
		h1 {
			font-size: 2rem;
		}

		main {
			padding: 1.5rem;
		}

		.options {
			grid-template-columns: 1fr;
		}
	}
</style>
