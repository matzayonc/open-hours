<script lang="ts">
	import { anonymousBackend, connect } from '$lib/canisters';
	import { onMount } from 'svelte';

	let backend: any = $state(undefined);
	let stats: any = $state(undefined);

	onMount(async () => {
		// if (!connectedBackend) connectedBackend = await connect();
		if (!anonymousBackend) throw new Error('No backend connection');
		stats = await anonymousBackend.get_open_hours_stats();
	});

	function occupancy(day: string, hour: string) {
		if (!stats) return 0;
		const total = Number(stats[0]);

		// I choose to ignore computational complexity here
		const occupied = stats[1][days.indexOf(day)][hours.indexOf(hour)];
		return Math.round((occupied / total) * 100);
	}

	const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	const hours = Array.from({ length: 32 }, (_, i) =>
		i % 2 == 0 ? i / 2 + 8 + ':00' : Math.floor(i / 2) + 8 + ':30'
	);

	let marking = $state(0);
	let chosen: number[] = $state([]);

	function startMarking(day: string, hour: string) {
		console.log('start marking');

		const targetMarked = chosen.findIndex((v) => v == encodeSlot(day, hour)) != -1;
		marking = targetMarked ? -1 : 1;

		document.addEventListener('mouseup', () => {
			console.log('end marking');
			marking = 0;
		});

		mark(day, hour);
	}

	function mark(day: string, hour: string) {
		if (marking == 1) {
			chosen = [...new Set(chosen), encodeSlot(day, hour)];
		} else if (marking == -1) {
			chosen = chosen.filter((v) => v != encodeSlot(day, hour));
		}
	}

	function encodeSlot(day: string, hour: string) {
		return (hours.indexOf(hour) << 3) + days.indexOf(day);
	}

	async function publish() {
		if (!backend) backend = await connect();
		if (!backend) return console.error('No backend connection');

		const res = await backend.set_open_hours(chosen);
		console.log(res);

		stats = await anonymousBackend.get_open_hours_stats();
	}
</script>

<header class="flex justify-between items-center">
	<h1><b> Open Hours </b> - kiedy piwo?</h1>
	<button onclick={publish} class="hover:bg-cyan-400 rounded-md bg-cyan-800 p-2">Publish</button>
</header>

<div class="select-none grid grid-cols-6 gaps-1">
	{#each days as day}
		<div>
			<div class="text-center text-md">
				{day}
			</div>
			{#each hours as hour}
				{#if chosen.findIndex((v) => v == encodeSlot(day, hour)) != -1}
					<div
						onmousedown={() => startMarking(day, hour)}
						onmouseenter={() => mark(day, hour)}
						class="time marked"
					>
						{hour}
						{occupancy(day, hour)}%
					</div>
				{:else}
					<div
						onmousedown={() => startMarking(day, hour)}
						onmouseenter={() => mark(day, hour)}
						class="time"
					>
						{hour}
						{occupancy(day, hour)}%
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
