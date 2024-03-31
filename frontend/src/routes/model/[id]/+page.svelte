<script lang="ts">
	import type { FullModelWithRelationsIds } from '$lib/helpers/typings.js';

	const { data } = $props();
	const model: FullModelWithRelationsIds = data.model;
</script>

<div class="w-screen">
	<div class="flex flex-col w-full">
		<h1 class="text-4xl mx-a">{model.title}</h1>
	</div>
</div>
