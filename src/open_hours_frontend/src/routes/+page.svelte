<script>
	import { backend } from '$lib/canisters';

	let greeting = '';

	function onSubmit(event) {
		const name = event.target.name.value;
		backend.greet(name).then((response) => {
			greeting = response;
		});
		return false;
	}
</script>

<main>
	<form action="#" on:submit|preventDefault={onSubmit}>
		<label for="name">Enter your name: &nbsp;</label>
		<input id="name" alt="Name" type="text" />
		<button type="submit">Click Me!</button>
	</form>
	<section id="greeting">{greeting}</section>
</main>

<style lang="postcss">
	main {
		display: grid;
		place-items: center;
		height: 100vh;
	}

	form {
		display: grid;
		grid-template-columns: auto auto;
		align-items: center;
		gap: 1em;
	}

	label {
		text-align: right;
	}

	input {
		padding: 0.5em;
	}


	button {
		@apply font-bold py-2 px-4 rounded;
		background-color: #333;
		color: white;
	}

	#greeting {
		margin-top: 1em;
	}

</style>


