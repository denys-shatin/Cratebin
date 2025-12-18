<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>Error - Cratebin</title>
</svelte:head>

<div class="container">
	<div class="error-box">
		<h1>{$page.status}</h1>
		<h2>{$page.error?.message || 'An error occurred'}</h2>
		<a href="/" class="btn">Go Home</a>
	</div>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial,
			sans-serif;
		background: #1a1a1a;
		color: #e0e0e0;
	}

	.container {
		max-width: 600px;
		margin: 0 auto;
		padding: 4rem 1rem;
	}

	.error-box {
		text-align: center;
		background: #2a2a2a;
		border: 1px solid #444;
		border-radius: 8px;
		padding: 3rem 2rem;
	}

	h1 {
		font-size: 4rem;
		margin: 0;
		color: #ff6b6b;
	}

	h2 {
		font-size: 1.5rem;
		margin: 1rem 0 2rem 0;
		color: #aaa;
		font-weight: normal;
	}

	.btn {
		display: inline-block;
		padding: 0.75rem 2rem;
		background: #4a9eff;
		color: white;
		text-decoration: none;
		border-radius: 4px;
		transition: background 0.2s;
	}

	.btn:hover {
		background: #3a8eef;
	}
</style>
