<script lang="ts">
	import '../app.css';
</script>

<slot />

<footer>
	<div class="footer-content">
		<div class="footer-links">
			<a href="https://github.com/denys-shatin/Cratebin" target="_blank" rel="noopener noreferrer" class="github-link">
				<svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor">
					<path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
				</svg>
				GitHub
			</a>
		</div>
		<div class="footer-text">
			<p>Free pastebin alternative • Code snippet sharing • Self-hosted • Open source</p>
		</div>
	</div>
</footer>

<style>
	footer {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		background: #0d1117;
		border-top: 1px solid #30363d;
		padding: 1rem;
		z-index: 1000;
	}

	.footer-content {
		max-width: 1200px;
		margin: 0 auto;
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
	}

	.footer-links {
		display: flex;
		gap: 1rem;
	}

	.github-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #21262d;
		color: #c9d1d9;
		text-decoration: none;
		border-radius: 6px;
		border: 1px solid #30363d;
		font-size: 0.9rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.github-link:hover {
		background: #30363d;
		border-color: #8b949e;
	}

	.footer-text {
		color: #8b949e;
		font-size: 0.85rem;
	}

	.footer-text p {
		margin: 0;
	}

	@media (max-width: 768px) {
		.footer-content {
			flex-direction: column;
			text-align: center;
		}

		.footer-text {
			font-size: 0.75rem;
		}
	}
</style>
