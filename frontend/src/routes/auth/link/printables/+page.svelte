<svelte:options runes={true} />

<script lang="ts">
	import { page } from '$app/stores';
	const { data } = $props();
	let printables_username = $state('Mawoka');
	let link_copy_success = $state(false);
	$effect(() => {
		printables_username = printables_username.trim().replace(/^@/, '');
	});

	const copy_to_clipboard = () => {
		const to_copy = `${$page.url.origin}/links/printables/${data.user.data?.profile_id}`;
		navigator.clipboard.writeText(to_copy);
		link_copy_success = true;
		setTimeout(() => {
			link_copy_success = false;
		}, 1000);
	};
</script>

<div class="w-screen h-screen flex bg-c-lgreen">
	<div class="m-auto w-2/3 h-fit shadow-2xl rounded flex flex-col p-2 bg-white gap-6">
		<h1 class="mx-auto text-3xl">Link your Printables!</h1>
		<div class="flex flex-col rounded-lg border-2 border-c-dgreen p-2">
			<h2 class="text-xl text-center">Enter your Printables Handle</h2>
			<label for="printables_username">Your Printables handle (not username)</label>
			<span class="rounded-lg bg-c-blue p-2 flex"
				>@<input
					bind:value={printables_username}
					type="text"
					autocomplete="off"
					class="w-full bg-transparent inline-block pl-0.5 outline-none focus:outline-none"
					id="printables_username"
				/></span
			>
			<p>
				For that, go to your Printables-Profile page and get the handle, not your display name. The
				handle is the text starting with the "@".
			</p>
		</div>
		<div
			class="flex flex-col rounded-lg border-2 border-c-dgreen p-2 transition gap-2"
			class:blur-sm={printables_username.length < 5}
		>
			<h2 class="text-xl text-center">Add the link to your profile</h2>
			First, copy the link below:
			<button
				class="flex rounded p-2 bg-c-blue justify-center"
				aria-roledescription="Copy link to clipboard"
				on:click={copy_to_clipboard}
			>
				<span class="w-5"></span>
				<span class="font-mono select-all text-nowrap ml-auto bg-transparent"
					>{$page.url.origin}/links/printables/{data.user.data?.profile_id}</span
				>{#if link_copy_success}
					<svg
						class="h-5 w-5 ml-auto my-auto flex"
						data-slot="icon"
						aria-hidden="true"
						fill="none"
						stroke-width="1.5"
						stroke="currentColor"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M11.35 3.836c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m8.9-4.414c.376.023.75.05 1.124.08 1.131.094 1.976 1.057 1.976 2.192V16.5A2.25 2.25 0 0 1 18 18.75h-2.25m-7.5-10.5H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V18.75m-7.5-10.5h6.375c.621 0 1.125.504 1.125 1.125v9.375m-8.25-3 1.5 1.5 3-3.75"
							stroke-linecap="round"
							stroke-linejoin="round"
						></path>
					</svg>
				{:else}<svg
						class="h-5 w-5 inline ml-auto my-auto flex"
						data-slot="icon"
						aria-hidden="true"
						fill="none"
						stroke-width="1.5"
						stroke="currentColor"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25ZM6.75 12h.008v.008H6.75V12Zm0 3h.008v.008H6.75V15Zm0 3h.008v.008H6.75V18Z"
							stroke-linecap="round"
							stroke-linejoin="round"
						></path>
					</svg>{/if}
			</button>
			<p class="">Now open your</p>
			<a
				href="https://www.printables.com/@{printables_username}#profile"
				target="_blank"
				class="mx-auto rounded-lg bg-c-blue p-2 w-2/3 text-center"
				>Profile Settings ⧉
			</a>
			<p>
				And add the link you've copied as a "<i
					>+ Add another social link</i
				>". Just paste the URL into there and hit "<i>Add</i>"
			</p>
			<p class="bg-c-brown w-fit rounded-lg p-2">Now don't forget to save!</p>
			<p>Now, make sure that there's a globe under the "<i>Message</i>"-button on your profile.</p>
		</div>
	</div>
</div>
