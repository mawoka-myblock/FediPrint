<svelte:options runes={true} />

<script lang="ts">
	import BrownButton from '$lib/components/button/brown.svelte';
	import type { FullModelWithRelationsIds } from '$lib/helpers/typings';

	const { model }: { model: FullModelWithRelationsIds } = $props();
</script>

<div class="grid grid-cols-2 p-4 w-full bg-c-dgreen shadow-2xl rounded-lg h-[30vh]">
	<div class="w-full h-full relative flex justify-center">
		<img
			src="/api/v1/storage/download/{model.images[0]}"
			class="object-cover max-w-full max-h-full absolute rounded-lg mx-auto"
		/>
	</div>
	<div class="flex flex-col text-white">
		<h2 class="mx-auto text-4xl">{model.title}</h2>
		<p class="mx-auto">{model.summary}</p>
		<div class="flex mx-auto">
			<BrownButton href="/model/{model.id}?own=true">View</BrownButton>
		</div>
	</div>
</div>
