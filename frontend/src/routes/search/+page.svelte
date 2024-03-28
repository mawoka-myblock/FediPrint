<svelte:options runes={true} />

<script lang="ts">
	import { browser } from '$app/environment';
	import { invalidateAll, pushState, goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import type { SearchResult } from './proxy+page.js';
	// import { page as svelte_page } from '$app/stores';

	let { data } = $props();

	let search_query = $state(data.query);
	let search_results: SearchResult | null = $state(data.results);
	let page = $state(data.page);
	const search = async () => {
		const params = new URLSearchParams(window.location.search);
		params.set('q', search_query);
		params.set('p', page.toString());
		const res = await fetch(`/api/v1/search/model?q=${search_query}&page=${page}`);
		search_results = await res.json();
		pushState('?' + params.toString(), {});
		// window.location.search = url.toString()
	};
</script>

<div class="flex h-screen">
	<div class="mx-auto h-full flex flex-col h-full w-1/3">
		<div class="h-fit w-full">
			<input
				class="p-2 text-center focus:border-c-dgreen border-2 border-c-brown focus:outline-none transition outline-none rounded-lg shadow-xl w-full text-2xl"
				type="text"
				name="q"
				id="q"
				bind:value={search_query}
				on:input={search}
			/>
		</div>
		<div class="h-full flex">
			<div class="mx-auto grid grid-cols-2 gap-4 w-full">
				{#if search_results}
					{#each search_results.hits as m}
						<div class="rounded-lg border-2 border-c-grey aspect-square">
							<h3>{m.result.title}</h3>
						</div>
					{/each}
				{:else}{/if}
			</div>
		</div>
	</div>
</div>
