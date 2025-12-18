<script lang="ts">
	import { page } from '$app/stores';
	import { api, type Snippet } from '$lib/api';
	import { onMount } from 'svelte';

	$: id = $page.params.id;

	let snippet: Snippet | null = null;
	let loading = true;
	let error = '';
	let password = '';
	let savedPassword = ''; // Store password after successful unlock
	let showPasswordPrompt = false;
	let copied = false;

	async function loadSnippet(pwd?: string) {
		loading = true;
		error = '';

		try {
			snippet = await api.getSnippet(id, pwd);
			showPasswordPrompt = false;
			if (pwd) {
				savedPassword = pwd; // Save password for raw link
			}
		} catch (e) {
			const message = e instanceof Error ? e.message : 'Failed to load snippet';
			if (message.includes('password')) {
				showPasswordPrompt = true;
				error = 'This snippet is password protected';
			} else {
				error = message;
			}
		} finally {
			loading = false;
		}
	}

	async function handlePasswordSubmit() {
		await loadSnippet(password);
	}

	async function copyContent() {
		if (snippet) {
			await navigator.clipboard.writeText(snippet.content);
			copied = true;
			setTimeout(() => (copied = false), 2000);
		}
	}

	function formatDate(dateStr: string) {
		return new Date(dateStr).toLocaleString();
	}

	onMount(() => {
		loadSnippet();
	});
</script>

<svelte:head>
	<title>Snippet {id} - Cratebin</title>
</svelte:head>

<div class="container">
	<header>
		<a href="/">← Back to Cratebin</a>
	</header>

	<main>
		{#if loading}
			<div class="loading">Loading snippet...</div>
		{:else if showPasswordPrompt}
			<div class="password-prompt">
				<h2>🔒 Password Required</h2>
				<p>This snippet is password protected</p>
				<form on:submit|preventDefault={handlePasswordSubmit}>
					<input
						type="password"
						bind:value={password}
						placeholder="Enter password"
						autofocus
					/>
					<button type="submit">Unlock</button>
				</form>
				{#if error}
					<div class="error">{error}</div>
				{/if}
			</div>
		{:else if error}
			<div class="error-box">
				<h2>Error</h2>
				<p>{error}</p>
				<a href="/">Create New Snippet</a>
			</div>
		{:else if snippet}
			<div class="snippet-view">
				<div class="toolbar">
					<div class="metadata">
						<span>Created: {formatDate(snippet.created_at)}</span>
						{#if snippet.expires_at}
							<span>Expires: {formatDate(snippet.expires_at)}</span>
						{/if}
						<span>{snippet.size} bytes</span>
					</div>
					<div class="actions">
						<button on:click={copyContent}>
							{copied ? '✓ Copied' : 'Copy'}
						</button>
						<a href={`/${id}/raw${savedPassword ? `?password=${encodeURIComponent(savedPassword)}` : ''}`}>
							Raw
						</a>
					</div>
				</div>

				<pre class="content">{snippet.content}</pre>
			</div>
		{/if}
	</main>
</div>

<style>
	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	header {
		margin-bottom: 1.5rem;
	}

	header a {
		color: #58a6ff;
		text-decoration: none;
		font-size: 0.95rem;
	}

	header a:hover {
		text-decoration: underline;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: #8b949e;
	}

	.password-prompt {
		max-width: 450px;
		margin: 0 auto;
		background: #161b22;
		border: 1px solid #30363d;
		border-radius: 6px;
		padding: 2rem;
		text-align: center;
	}

	.password-prompt h2 {
		margin: 0 0 0.5rem 0;
		color: #c9d1d9;
		font-size: 1.5rem;
		font-weight: 600;
	}

	.password-prompt p {
		color: #8b949e;
		margin-bottom: 1.5rem;
	}

	.password-prompt form {
		display: flex;
		gap: 0.5rem;
	}

	.password-prompt input {
		flex: 1;
		padding: 0.625rem;
		background: #0d1117;
		border: 1px solid #30363d;
		border-radius: 6px;
		color: #c9d1d9;
		font-size: 1rem;
	}

	.password-prompt input:focus {
		outline: none;
		border-color: #58a6ff;
	}

	.password-prompt button {
		padding: 0.625rem 1.25rem;
		background: #238636;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
	}

	.password-prompt button:hover {
		background: #2ea043;
	}

	.error-box {
		max-width: 500px;
		margin: 0 auto;
		text-align: center;
		background: #161b22;
		border: 1px solid #da3633;
		border-radius: 6px;
		padding: 2rem;
	}

	.error-box h2 {
		color: #f85149;
		margin: 0 0 0.5rem 0;
		font-size: 1.5rem;
	}

	.error-box p {
		color: #8b949e;
		margin-bottom: 1.5rem;
	}

	.error-box a {
		display: inline-block;
		padding: 0.625rem 1.25rem;
		background: #238636;
		color: white;
		text-decoration: none;
		border-radius: 6px;
		font-weight: 500;
	}

	.error {
		margin-top: 1rem;
		padding: 0.75rem;
		background: #3d1319;
		border: 1px solid #da3633;
		border-radius: 6px;
		color: #f85149;
	}

	.snippet-view {
		background: #161b22;
		border: 1px solid #30363d;
		border-radius: 6px;
		overflow: hidden;
	}

	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: #0d1117;
		border-bottom: 1px solid #30363d;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.metadata {
		display: flex;
		gap: 1.5rem;
		font-size: 0.85rem;
		color: #8b949e;
		flex-wrap: wrap;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.actions button,
	.actions a {
		padding: 0.5rem 1rem;
		background: #21262d;
		color: #c9d1d9;
		border: 1px solid #30363d;
		border-radius: 6px;
		cursor: pointer;
		text-decoration: none;
		font-size: 0.9rem;
		font-weight: 500;
	}

	.actions button:hover,
	.actions a:hover {
		background: #30363d;
		border-color: #8b949e;
	}

	.content {
		margin: 0;
		padding: 1.5rem;
		background: #0d1117;
		color: #c9d1d9;
		font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
		font-size: 0.9rem;
		line-height: 1.5;
		overflow-x: auto;
		white-space: pre-wrap;
		word-wrap: break-word;
	}

	@media (max-width: 600px) {
		.toolbar {
			flex-direction: column;
			align-items: flex-start;
		}

		.metadata {
			flex-direction: column;
			gap: 0.5rem;
		}

		.actions {
			width: 100%;
		}

		.actions button,
		.actions a {
			flex: 1;
			text-align: center;
		}
	}
</style>
