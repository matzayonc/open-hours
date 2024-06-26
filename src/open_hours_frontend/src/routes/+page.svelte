<script lang="ts">
	import { anonymousBackend, connect } from '$lib/canisters';
	import { Principal } from '@dfinity/principal';
	import { onMount } from 'svelte';

	let backend: any = $state(undefined);
	let stats: any = $state(undefined);

	onMount(async () => {
		// if (!connectedBackend) connectedBackend = await connect();
		if (!anonymousBackend) throw new Error('No backend connection');
		stats = await anonymousBackend.open_counts();
	});

	function occupancy(day: string, hour: string) {
		if (!stats) return 0;
		if (stats[0] === 0n) return 0;
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
	let openAt: number | undefined = $state(undefined);
	let openDetails: string[] = $state([]);

	function startMarking(day: string, hour: string) {
		const targetMarked = chosen.findIndex((v) => v == encodeSlot(day, hour)) != -1;
		marking = targetMarked ? -1 : 1;

		document.addEventListener('mouseup', () => {
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

	async function startConnecting() {
		if (!backend) backend = await connect();
		console.log('publishing', backend);
		if (!backend) return console.error('No backend connection');
	}

	async function publish() {
		const res = await backend.set_open_hours(chosen);
		console.log(res);

		stats = await anonymousBackend.open_counts();
	}

	async function slotClick(ev: MouseEvent, day: string, hour: string) {
		if (ev.button == 0) startMarking(day, hour);
		else if (ev.button == 2) {
			const slot = encodeSlot(day, hour);
			const res = await anonymousBackend.open_at([slot]);
			openDetails = res.map((r: any) => Principal.fromUint8Array(r[1]).toText());
			openAt = slot;
		}
		ev.preventDefault();
	}

	$effect(() => {
		if (!backend) return;
		backend.open_for([]).then((savedAlready: Uint8Array[]) => {
			const parsed = savedAlready.map((v) => Number(v));
			console.log('saved already', parsed);
			chosen = [...chosen, ...parsed];
		});
	});
</script>

<header class="flex justify-between items-center">
	<h1><b> Open Hours </b> - kiedy piwo?</h1>
	{#if backend == undefined}
		<button onclick={startConnecting} class="hover:bg-cyan-400 rounded-md bg-cyan-800 p-2">
			Connect
		</button>
	{:else}
		<button onclick={publish} class="hover:bg-cyan-400 rounded-md bg-cyan-800 p-2">
			Publish
		</button>
	{/if}
</header>

<div class="select-none grid grid-cols-6 gaps-1">
	{#each days as day}
		<div>
			<div class="text-center text-md">
				{day}
			</div>
			{#each hours as hour}
				<div
					onmousedown={(e) => slotClick(e, day, hour)}
					onmouseenter={() => mark(day, hour)}
					class="time {chosen.findIndex((v) => v == encodeSlot(day, hour)) != -1 ? 'marked' : ''}"
				>
					{hour}
					{occupancy(day, hour)}%
					{#if openAt == encodeSlot(day, hour)}
						<ol>
							{#each openDetails as open}
								<li>
									{open}
								</li>
							{/each}
						</ol>
					{/if}
				</div>
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
