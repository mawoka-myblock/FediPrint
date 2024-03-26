<svelte:options runes={true} />

<script lang="ts">
	let search_query = $state('');
	let search_results = $derived.by(async () => {
		const res = await fetch('');
	});
</script>

<div class="flex h-screen">
	<div class="mx-auto h-full flex flex-col h-full w-1/3">
		<div class="h-full mx-auto">
			<h1>Search through the models</h1>
		</div>
		<div class="h-fit w-full">
			<input
				class="p-2 text-center focus:border-c-dgreen border-2 border-c-brown focus:outline-none transition outline-none rounded-lg shadow-xl w-full text-2xl"
				type="text"
				bind:value={search_query}
			/>
		</div>
		<div class="h-full flex">
			<ul class="mx-auto">
				<li>PLACEHOLDER!</li>
			</ul>
		</div>
	</div>
</div>
