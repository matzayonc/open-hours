<script>
	import "../app.scss";
</script>

<div class="app">
	<header>
		<h1>Open Hours</h1>
	</header>

	<main>
		<slot />
	</main>

	<footer>
		<p>visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p>
	</footer>
</div>

<style lang="postcss">
	.app {
		display: grid;
		grid-template-rows: auto 1fr auto;
		min-height: 100vh;
	}

	header {
		background-color: #333;
		color: white;
		padding: 1em;
	}

	main {
		padding: 1em;
	}

	footer {
		background-color: #333;
		color: white;
		padding: 1em;
		text-align: center;
	}
</style>