<svelte:options runes={true} />

<script lang="ts">
	import '@fontsource/marck-script';

	let data = $state({
		email: '',
		pw: ''
	});
	let loading = $state(false);
	let valid = $derived.by(() => {
		if (data.email.length < 8)
			return { valid: false, hint: 'Shorter than 8 characters', email: false, pw: true };
		if (data.pw.length < 8)
			return { valid: false, hint: 'Must be longer than 8 characters', pw: false, email: true };
		return { valid: true, hint: undefined, pw: true, email: true };
	});

	const submit = async () => {
		if (!valid.valid) {
			return;
		}
		loading = true;
		const res = await fetch('/api/v1/auth/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: new URLSearchParams({
				email: data.email,
				password: data.pw
			})
		});
		if (res.ok) {
			const return_to = new URLSearchParams(window.location.search).get('return_to') ?? '/home';
			window.location.replace(return_to);
		} else if (res.status === 401) {
			alert('Login unsuccessful');
		}
		loading = false;
		data = {
			email: '',
			pw: ''
		};
	};
</script>

<svelte:head>
	<title>Login to FediPrint</title>
</svelte:head>

<div class="w-screen h-screen flex bg-red">
	<section class="m-auto w-1/3 h-fit bg-white/60 rounded shadow-xl p-2">
		<h1 class="text-center text-3xl">
			Let's get you into <span class="marck-script text-4xl">FediPrint!</span>
		</h1>
		<form on:submit|preventDefault={submit}>
			<div>
				<label for="email" class="block">Email address</label>
				<div class="mt-2">
					<input
						type="email"
						name="email"
						id="email"
						autocomplete="email"
						required
						class="block transition w-full rounded-md border-2 p-2 text-gray-900 shadow-sm"
						class:border-red-800={!valid.email}
						bind:value={data.email}
					/>
				</div>
				<label for="pw1" class="block">Password</label>
				<div class="mt-2">
					<input
						type="password"
						name="pw1"
						id="pw1"
						required
						class="block transition w-full rounded-md border-2 p-2 text-gray-900 shadow-sm"
						class:border-red-800={!valid.pw}
						bind:value={data.pw}
					/>
				</div>
				<div class="flex mt-2">
					<button class="mx-auto p-2 rounded bg-c-lgreen" disabled={!valid.valid && !loading}
						>{#if loading}Loading...{:else}Submit!{/if}</button
					>
				</div>
			</div>
		</form>
	</section>
</div>
