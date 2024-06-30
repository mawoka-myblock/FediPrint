<svelte:options runes={true} />

<script lang="ts">
	import type { FullModelWithRelationsIds } from '$lib/helpers/typings.js';
	import BrownButton from '$lib/components/button/brown.svelte';
	import Gallery from '$lib/components/media/gallery.svelte';

	const { data } = $props();
	const model: FullModelWithRelationsIds = data.model;
	const own: boolean = data.own;

	const toggle_visibility = async () => {
		const res = await fetch('/api/v1/model/visibility', {
			method: 'PATCH',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ model_id: model.id, public: !model.published })
		});
		if (res.ok) {
			window.location.reload();
			return;
		}
		alert('Something went wrong!');
	};
</script>

<div class="w-screen">
	{#if own}
		<div class="grid grid-cols-3">
			<span></span>
			<div class="flex flex-col">
				<div class="mx-auto">
					<BrownButton flex={true} on:click={toggle_visibility}
						>{#if model.published}Unpublish{:else}Publish{/if}</BrownButton
					>
				</div>
			</div>
		</div>
	{/if}
	<div class="flex flex-col w-full">
		<h1 class="text-4xl mx-a">{model.title}</h1>
	</div>
	<div class="grid grid-cols-2">
		{#if model.images}
			<Gallery image_ids={model.images} />
		{/if}
		{#if model.description}
			<p>{model.description}</p>
		{/if}
	</div>
	<section class="flex flex-col">
		<h2>Files</h2>
		{#if model.cost != 0}
			<p>This model is paid.</p>
			<a href="/api/v1/payments/stripe/pay/{model.id}">But it now for {(model.cost / 100).toFixed(2)} ct!</a>
		{/if}
	</section>
</div>
