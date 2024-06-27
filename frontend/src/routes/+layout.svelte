<script lang="ts">
	import '@unocss/reset/tailwind-compat.css';
	import '@fontsource/marck-script/index.css';
	import Navbar from '$lib/components/navbar.svelte';
</script>

<div class="h-screen flex flex-col">
	<Navbar />
	<div class="w-full h-full">
		<slot />
	</div>
</div>

<style>
	:global(.marck-script) {
		font-family: 'Marck Script', cursive;
	}
</style>
