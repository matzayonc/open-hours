<script lang="ts">
	import { backend } from '$lib/canisters';

	const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	const hours = Array.from({ length: 32 }, (_, i) =>
		i % 2 == 0 ? i / 2 + 8 + ':00' : Math.floor(i / 2) + 8 + ':30'
	);

	let marking = $state(0);
	let chosen: string[] = $state([]);

	function startMarking(day: string, hour: string) {
		console.log('start marking');

		const targetMarked = chosen.findIndex((v) => v == day + hour) != -1;
		marking = targetMarked ? -1 : 1;

		document.addEventListener('mouseup', () => {
			console.log('end marking');
			marking = 0;
		});

		mark(day, hour);
	}

	function mark(day: string, hour: string) {
		if (marking == 1) {
			chosen = [...new Set(chosen), day + hour];
		} else if (marking == -1) {
			chosen = chosen.filter((v) => v != day + hour);
		}
	}
</script>

<header class="flex justify-between items-center">
	<h1><b> Open Hours </b> - kiedy piwo?</h1>
	<button class="hover:bg-cyan-400 rounded-md bg-cyan-800 p-2">Publish</button>
</header>

<div class="select-none grid grid-cols-6 gaps-1">
	{#each days as day}
		<div>
			<div class="text-center text-md">
				{day}
			</div>
			{#each hours as hour}
				{#if chosen.findIndex((v) => v == day + hour) != -1}
					<div
						onmousedown={() => startMarking(day, hour)}
						onmouseenter={() => mark(day, hour)}
						class="time marked"
					>
						{hour}
					</div>
				{:else}
					<div
						onmousedown={() => startMarking(day, hour)}
						onmouseenter={() => mark(day, hour)}
						class="time"
					>
						{hour}
					</div>
				{/if}
			{/each}
		</div>
	{/each}
</div>

<style lang="postcss">
	header {
		@apply bg-gray-800 text-white p-4;
	}

	.time {
		@apply select-none transition duration-200 hover:bg-gray-200 text-sm rounded-lg bg-cyan-200 m-1 border border-cyan-400 p-1 text-center;

		&.marked {
			@apply bg-cyan-400;
		}
	}
</style>
