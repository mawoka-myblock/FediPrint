<svelte:options runes={true} />

<script lang="ts">
	let {
		disabled = false,
		flex = false,
		href = undefined,
		target = '_self',
		type = 'button'
	} = $props();
</script>

{#if href}
	<a
		{href}
		{target}
		{disabled}
		class:opacity-50={disabled}
		class:cursor-not-allowed={disabled}
		class:pointer-events-none={disabled}
		class="text-black hover:bg-opacity-80 w-full px-4 py-2 leading-5 transition-all duration-200 transform bg-[#B07156] rounded text-center outline-none"
		on:click
		class:flex
		class:justify-center={flex}
	>
		<slot />
	</a>
{:else}
	<button
		{disabled}
		{type}
		class="text-black hover:opacity-80 w-full px-4 py-2 leading-5 transition-all duration-200 transform bg-[#B07156] rounded text-center focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 outline-none"
		on:click
		class:flex
		class:justify-center={flex}
	>
		<slot />
	</button>
{/if}
