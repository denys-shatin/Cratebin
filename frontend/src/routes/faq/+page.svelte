<svelte:head>
	<title>FAQ - Cratebin</title>
	<meta name="description" content="Frequently asked questions about Cratebin - free online pastebin and code snippet sharing service." />
</svelte:head>

<div class="container">
	<header>
		<a href="/">← Back to Cratebin</a>
		<h1>Frequently Asked Questions</h1>
	</header>

	<main>
		<details open>
			<summary>What is Cratebin?</summary>
			<p>Cratebin is a free, self-hosted pastebin alternative for sharing code snippets, logs, and text online. It offers password protection, auto-expiration, and raw text mode without requiring registration.</p>
		</details>

		<details>
			<summary>How do I share a code snippet?</summary>
			<p>Simply paste your code or text into the editor, choose visibility settings and expiration time, then click "Create Snippet". You'll get a shareable link instantly.</p>
		</details>

		<details>
			<summary>What are the visibility options?</summary>
			<p><strong>Public:</strong> Anyone can find and view your snippet.<br>
			<strong>Unlisted:</strong> Only people with the link can view it.<br>
			<strong>Private:</strong> Requires a password to access.</p>
		</details>

		<details>
			<summary>How long do snippets last?</summary>
			<p>You can set snippets to expire after 1 hour, 24 hours, 7 days, or never. Expired snippets are automatically deleted from our servers.</p>
		</details>

		<details>
			<summary>Can I delete my snippet?</summary>
			<p>Yes! When you create a snippet, you receive a delete token. Use this token to delete your snippet at any time before it expires.</p>
		</details>

		<details>
			<summary>Is Cratebin open source?</summary>
			<p>Yes! Cratebin is fully open source and available on <a href="https://github.com/denys-shatin/Cratebin" target="_blank" rel="noopener">GitHub</a>. You can self-host it on your own server.</p>
		</details>

		<details>
			<summary>Do I need to register?</summary>
			<p>No registration required! Cratebin is completely anonymous and free to use. Just paste and share.</p>
		</details>

		<details>
			<summary>What makes Cratebin secure?</summary>
			<p>Cratebin uses Argon2 password hashing, rate limiting, UUID v4 IDs, and supports password-protected snippets. All data is encrypted in transit with HTTPS.</p>
		</details>

		<details>
			<summary>Can I use Cratebin for sensitive data?</summary>
			<p>While Cratebin offers password protection and encryption, we recommend not sharing highly sensitive data like passwords, API keys, or personal information. Use private snippets with strong passwords for moderately sensitive content.</p>
		</details>

		<details>
			<summary>What's the maximum snippet size?</summary>
			<p>The maximum snippet size is 512 KB (524,288 bytes). This is enough for most code files and logs.</p>
		</details>

		<details>
			<summary>How do I self-host Cratebin?</summary>
			<p>Clone the repository from <a href="https://github.com/denys-shatin/Cratebin" target="_blank" rel="noopener">GitHub</a>, configure your environment variables, and run with Docker Compose. Full deployment instructions are available in the README.</p>
		</details>

		<details>
			<summary>What technology stack does Cratebin use?</summary>
			<p>Cratebin is built with Rust (Axum framework) for the backend, SvelteKit with TypeScript for the frontend, and PostgreSQL for the database. Everything runs in Docker containers.</p>
		</details>
	</main>
</div>

<style>
	.container {
		max-width: 800px;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	header {
		margin-bottom: 2rem;
	}

	header a {
		color: #58a6ff;
		text-decoration: none;
		font-size: 0.95rem;
		display: inline-block;
		margin-bottom: 1rem;
	}

	header a:hover {
		text-decoration: underline;
	}

	h1 {
		font-size: 2rem;
		color: #c9d1d9;
		margin: 0;
		font-weight: 600;
	}

	main {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	details {
		background: #161b22;
		border: 1px solid #30363d;
		border-radius: 6px;
		padding: 1.25rem;
		transition: border-color 0.2s;
	}

	details[open] {
		border-color: #58a6ff;
	}

	summary {
		color: #58a6ff;
		font-weight: 500;
		cursor: pointer;
		font-size: 1.05rem;
		user-select: none;
		list-style: none;
	}

	summary::-webkit-details-marker {
		display: none;
	}

	summary::before {
		content: '▶';
		display: inline-block;
		margin-right: 0.5rem;
		transition: transform 0.2s;
	}

	details[open] summary::before {
		transform: rotate(90deg);
	}

	summary:hover {
		color: #79c0ff;
	}

	details p {
		margin: 1rem 0 0 1.5rem;
		color: #8b949e;
		line-height: 1.6;
	}

	details a {
		color: #58a6ff;
		text-decoration: none;
	}

	details a:hover {
		text-decoration: underline;
	}

	@media (max-width: 600px) {
		h1 {
			font-size: 1.5rem;
		}

		details {
			padding: 1rem;
		}

		summary {
			font-size: 1rem;
		}

		details p {
			margin-left: 1rem;
		}
	}
</style>
