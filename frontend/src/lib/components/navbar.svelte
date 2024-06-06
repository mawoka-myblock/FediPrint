<script lang="ts">
	import '@fontsource/marck-script/index.css';
	import { user } from '$lib/stores';
	import { browser } from '$app/environment';
	import { beforeNavigate } from '$app/navigation';
	import { draw, slide } from 'svelte/transition';

	let menuIsClosed = true;
	const toggleMenu = () => {
		menuIsClosed = !menuIsClosed;
	};

	beforeNavigate(() => {
		menuIsClosed = true; // Closes menu to let the user see the page beneath
	});

	let darkMode = false;
	if (browser) {
		darkMode =
			localStorage.theme === 'dark' ||
			(!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches);
	}

	const switchDarkMode = () => {
		!darkMode ? localStorage.setItem('theme', 'dark') : localStorage.setItem('theme', 'light');
		window.location.reload();
	};
</script>

<nav class="w-screen px-4 lg:px-10 py-2 sticky backdrop-blur-2xl bg-white/70 shadow-md z-30 top-0">
	<!-- Desktop navbar -->
	<div class="hidden lg:flex lg:items-center lg:flex-row lg:justify-between">
		<div class="lg:flex lg:items-center lg:flex-row gap-1">
			<a
				href="/"
				class="font-black tracking-tight text-xl lg:text-2xl text-black marck-script link-hover px-3 lg:px-5"
				>FediPrint</a
			>
			<!-- <a class="btn-nav" href="/explore">Explore</a> -->
			<a class="btn-nav" href="/search">Search</a>
			{#if $user}
				<a class="btn-nav border-2 rounded" href="/models/create/select">Create</a>

				<a class="btn-nav" href="/models/own">Own Models</a>
			{:else}
				<a class="btn-nav" href="/docs">Docs </a>
				<a
					target="_blank"
					class="btn-nav flex items-center gap-1"
					href="https://github.com/mawoka-myblock/FediPrint"
					>GitHub
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="17"
						height="17"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" /><polyline
							points="15 3 21 3 21 9"
						/><line x1="10" x2="21" y1="14" y2="3" /></svg
					>
				</a>
			{/if}
		</div>
		<div class="lg:flex lg:items-center lg:flex-row gap-1">
			{#if $user}
				<a class="btn-nav" href="/api/v1/users/logout">Logout</a>
			{:else}
				{#if !import.meta.env.VITE_REGISTRATION_DISABLED}
					<a class="btn-nav" href="/auth/register">Register</a>
				{/if}

				<a class="btn-nav" href="/auth/login?returnTo=">Login</a>
			{/if}

			<div class="fit-content flex items-center justify-center gap-2">
				<div class="lg:flex items-center justify-center">
					{#if darkMode}
						<button
							on:click={() => {
								switchDarkMode();
							}}
							aria-label="Activate light mode"
							class="bg-transparent"
						>
							<!-- Heroicons: sun -->
							<svg
								class="w-6 h-6 text-black"
								fill="none"
								aria-label="Sun-Icon"
								stroke="currentColor"
								viewBox="0 0 24 24"
								xmlns="http://www.w3.org/2000/svg"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									stroke="currentColor"
									d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
								/>
							</svg>
						</button>
					{:else}
						<button
							on:click={() => {
								switchDarkMode();
							}}
							aria-label="Activate darkmode"
							class="bg-transparent"
						>
							<!-- Heroicons: moon -->
							<svg
								aria-label="Moon-Icon"
								class="w-6 h-6"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
								xmlns="http://www.w3.org/2000/svg"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
								/>
							</svg>
						</button>
					{/if}
				</div>
			</div>
		</div>
	</div>

	<!-- Mobile navbar -->
	<div class="lg:hidden">
		<!-- Navbar header -->
		<div class="flex items-center justify-between">
			<a
				href="/"
				class="font-black tracking-tight text-xl lg:text-2xl text-black marck-script link-hover px-3 lg:px-5"
				>ClassQuiz</a
			>
			<a class="btn-nav flex" href="/models/create">Create</a>

			<!-- Dark/Light mode toggle + Open/Close menu -->
			<div class="flex items-center">
				{#if darkMode}
					<!-- Sun icon -->
					<button
						class="px-3"
						on:click={() => {
							switchDarkMode();
						}}
						aria-label="Activate light mode"
					>
						<!-- Heroicons: sun -->
						<svg
							class="w-6 h-6 text-black"
							fill="none"
							aria-label="Sun-Icon"
							stroke="currentColor"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								stroke="currentColor"
								d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
							/>
						</svg>
					</button>
				{:else}
					<!-- Moon icon -->
					<button
						class="px-3"
						on:click={() => {
							switchDarkMode();
						}}
						aria-label="Activate darkmode"
					>
						<!-- Heroicons: moon -->
						<svg
							aria-label="Moon-Icon"
							class="w-6 h-6"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
							/>
						</svg>
					</button>
				{/if}

				{#if menuIsClosed}
					<button class="px-3" id="open-menu" on:click={toggleMenu} aria-label="Open navbar">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							stroke="#000000"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M3 6h18M3 12h18M3 18h18" />
						</svg>
					</button>
				{:else}
					<button class="px-3" id="close-menu" on:click={toggleMenu} aria-label="Close navbar">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							stroke="#000000"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							><path in:draw={{ duration: 300 }} d="M18 6 6 18" /><path
								in:draw={{ duration: 300 }}
								d="m6 6 12 12"
							/></svg
						>
					</button>
				{/if}
			</div>
		</div>

		<!-- Navbar content -->
		{#if !menuIsClosed}
			<div class="flex flex-col" transition:slide={{ duration: 400 }}>
				<a class="btn-nav" href="/explore">Explore</a>
				<a class="btn-nav" href="/search">Search</a>
				{#if $user}
					<a class="btn-nav" href="/dashboard">Dashboard</a>
				{:else}
					<a class="btn-nav" href="/docs">Docs</a>
					<a
						target="_blank"
						class="btn-nav flex items-center gap-1"
						href="https://github.com/mawoka-myblock/FediPrint"
						>GitHub
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="17"
							height="17"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="lucide lucide-external-link"
							><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" /><polyline
								points="15 3 21 3 21 9"
							/><line x1="10" x2="21" y1="14" y2="3" /></svg
						>
					</a>
				{/if}

				<hr class="my-1 border" />
				{#if $user}
					<a class="btn-nav" href="/api/v1/users/logout">Logout</a>
				{:else}
					{#if !import.meta.env.VITE_REGISTRATION_DISABLED}
						<a class="btn-nav" href="/auth/register">Register</a>
					{/if}

					<a class="btn-nav" href="/auth/login?returnTo=">Login</a>
				{/if}
			</div>
		{/if}
	</div>
</nav>

<style>
	.btn-nav {
		@apply text-lg font-medium px-3 text-gray-600 hover:text-green-600 py-1.5 transition-all duration-300;
	}
</style>
