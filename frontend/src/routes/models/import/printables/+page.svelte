<script lang="ts">
	import BrownButton from '$lib/components/button/brown.svelte';

	let loading = $state(false);
	let import_id: undefined | number = $state(undefined);

	const import_single = async () => {
		loading = true
		const res = await fetch('/api/v1/links/printables/import/single', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ id: import_id })
		});
		if (res.status === 412) {
			alert('Printables account not linked');
			window.location.assign('/auth/link/printables');
		}
		if (res.status === 401) {
			alert("The model isn't owned by you")
		}
		if ( res.status === 404) {
			alert("Model wasn't found")
		}
		if (res.status === 500) {
			alert("Model probably already imported")
		}
		if (res.status === 422) {
			alert("Invalid ID")
		}
		if (res.status === 200) {
			const json = await res.json()
			window.location.assign("/model/"+json.id)
		}
		loading = false
	};

	const import_all = async () => {
		loading = true
		const res = await fetch('/api/v1/links/printables/import', {
			method: 'POST'
		});
		if (res.status === 412) {
			alert('Printables account not linked');
			window.location.assign('/auth/link/printables');
		}
		if (res.status === 500) {
			alert("You've porbably already imported some models.");
		}
		if (res.status === 200) {
			alert('Success!');
			window.location.assign('/home');
		}
		loading = false
	};
</script>

<div class="w-screen h-screen flex">
	<div class="m-auto w-1/2 flex flex-col rounded-lg p-2 border-c-blue border-2 h-fit">
		<h1 class="mx-auto text-3xl">Import from Printables</h1>
		<div class="grid grid-cols-2">
			<div class="flex flex-col">
				<h2 class="mx-auto text-xl">Import All Models</h2>
				<BrownButton type="submit" disabled={loading}
					>{#if loading}Loading...{:else}Submit!{/if}</BrownButton
				>
			</div>
			<div class="flex flex-col">
				<h2 class="mx-auto text-xl">Import Single Model</h2>
				<p>Get the ID from your model. Take the following model and its URL as an example:</p>
				<span class="font-mono text-sm"
					>https://www.printables.com/model/<b>773535</b>-klackender-for-elegoo-neptune-4</span
				>
				<p>So, as you can see, the ID is the number directly after slash and before the minus.</p>
				<form class="flex flex-col p-2 gap-2" on:submit={import_single}>
					<label for="id">Model ID</label>
					<input
						type="number"
						name="id"
						id="id"
						class="w-fit min-w-12 border-2 border-c-dgreen rounded-lg p-2 focus:outline-none"
						required
						bind:value={import_id}
					/>
					<BrownButton type="submit" disabled={loading}
						>{#if loading}Loading...{:else}Submit!{/if}</BrownButton
					>
				</form>
			</div>
		</div>
	</div>
</div>
