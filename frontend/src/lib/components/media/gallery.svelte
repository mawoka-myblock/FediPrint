<svelte:options runes={true} />

<script lang="ts">
	let { image_ids = [] }: { image_ids: string[] } = $props();

	const images: {
		[key: string]: {
			blurhash?: string;
			alt_text?: string;
		};
	} = {};

	const fetch_metadata = async (id: string) => {
		const res = await fetch(`/api/v1/storage/download/${id}`, { method: 'HEAD' });
		const headers = res.headers;
		const blurhash = headers.get('X-Blurhash') ?? undefined;
		const alt_text = headers.get('X-Alt-Text') ?? undefined;
		images[id] = {
			blurhash: blurhash,
			alt_text: alt_text
		};
	};
	$effect(() => {
		for (const id of image_ids) {
			fetch_metadata(id);
		}
	});
</script>

<div class="w-[75vh]">
	<div class="h-0 pb-full">
		{#each image_ids as image, i}
			<div class="relative w-full h-0 pb-full flex bg-red flex">
				<img
					class="absolute inset-0 object-contain max-h-full max-w-full m-auto"
					src="/api/v1/storage/download/{image}"
					alt={images[image]?.alt_text}
				/>
			</div>
		{/each}
	</div>
</div>
