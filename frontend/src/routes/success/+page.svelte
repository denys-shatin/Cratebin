<script lang="ts">
	import { page } from '$app/stores';
	import { PUBLIC_APP_URL } from '$env/static/public';

	$: id = $page.url.searchParams.get('id') || '';
	$: deleteToken = $page.url.searchParams.get('token') || '';
	$: snippetUrl = `${PUBLIC_APP_URL || 'http://localhost:3000'}/${id}`;

	let copied = false;
	let tokenCopied = false;

	async function copyUrl() {
		await navigator.clipboard.writeText(snippetUrl);
		copied = true;
		setTimeout(() => (copied = false), 2000);
	}

	async function copyToken() {
		await navigator.clipboard.writeText(deleteToken);
		tokenCopied = true;
		setTimeout(() => (tokenCopied = false), 2000);
	}
</script>

<svelte:head>
	<title>Snippet Created - Cratebin</title>
</svelte:head>

<div class="container">
	<header>
		<h1>✓ Snippet Created</h1>
		<p>Your snippet is ready to share</p>
	</header>

	<main>
		<div class="success-box">
			<div class="field">
				<label>Snippet URL</label>
				<div class="input-group">
					<input type="text" value={snippetUrl} readonly />
					<button on:click={copyUrl}>
						{copied ? '✓ Copied' : 'Copy'}
					</button>
				</div>
			</div>

			<div class="field">
				<label>Delete Token</label>
				<div class="input-group">
					<input type="text" value={deleteToken} readonly />
					<button on:click={copyToken}>
						{tokenCopied ? '✓ Copied' : 'Copy'}
					</button>
				</div>
				<p class="warning">⚠️ Save this token - you'll need it to delete the snippet</p>
			</div>

			<div class="actions">
				<a href={`/${id}`} class="btn-primary">View Snippet</a>
				<a href="/" class="btn-secondary">Create Another</a>
			</div>
		</div>
	</main>
</div>

<style>
	.container {
		max-width: 700px;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	header {
		text-align: center;
		margin-bottom: 2rem;
	}

	h1 {
		font-size: 2rem;
		margin: 0 0 0.5rem 0;
		color: #3fb950;
		font-weight: 600;
	}

	header p {
		color: #8b949e;
		margin: 0;
	}

	.success-box {
		background: #161b22;
		border: 1px solid #30363d;
		border-radius: 6px;
		padding: 2rem;
	}

	.field {
		margin-bottom: 1.5rem;
	}

	.field:last-of-type {
		margin-bottom: 2rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		color: #c9d1d9;
		font-size: 0.95rem;
		font-weight: 500;
	}

	.input-group {
		display: flex;
		gap: 0.5rem;
	}

	input {
		flex: 1;
		padding: 0.625rem;
		background: #0d1117;
		border: 1px solid #30363d;
		border-radius: 6px;
		color: #c9d1d9;
		font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
		font-size: 0.9rem;
	}

	input:focus {
		outline: none;
		border-color: #58a6ff;
	}

	.input-group button {
		padding: 0.625rem 1rem;
		background: #21262d;
		color: #c9d1d9;
		border: 1px solid #30363d;
		border-radius: 6px;
		cursor: pointer;
		white-space: nowrap;
		font-weight: 500;
	}

	.input-group button:hover {
		background: #30363d;
		border-color: #8b949e;
	}

	.warning {
		margin: 0.5rem 0 0 0;
		padding: 0.75rem;
		background: #3d2817;
		border: 1px solid #9e6a03;
		border-radius: 6px;
		color: #f0883e;
		font-size: 0.85rem;
	}

	.actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-primary,
	.btn-secondary {
		flex: 1;
		padding: 0.75rem;
		text-align: center;
		text-decoration: none;
		border-radius: 6px;
		font-size: 1rem;
		font-weight: 500;
	}

	.btn-primary {
		background: #238636;
		color: white;
	}

	.btn-primary:hover {
		background: #2ea043;
	}

	.btn-secondary {
		background: #21262d;
		color: #c9d1d9;
		border: 1px solid #30363d;
	}

	.btn-secondary:hover {
		background: #30363d;
		border-color: #8b949e;
	}

	@media (max-width: 600px) {
		.actions {
			flex-direction: column;
		}
	}
</style>
