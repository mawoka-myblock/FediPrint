<svelte:options runes={true} />

<script lang="ts">
	import '@fontsource/marck-script';

	let data = $state({
		email: '',
		pw1: '',
		pw2: '',
		username: '',
		display_name: ''
	});
	let loading = $state(false);
	const username_regex = /^\w{4,30}$/;
	const display_name_regex = /^.{4,30}$/;
	let valid = $derived.by(() => {
		if (data.email.length < 8)
			return {
				valid: false,
				hint: 'Shorter than 8 characters',
				email: false,
				pw: true,
				display: true,
				username: true
			};
		if (data.pw1.length < 8)
			return {
				valid: false,
				hint: 'Do not match',
				pw: false,
				email: true,
				display: true,
				username: true
			};
		if (data.pw1 !== data.pw2)
			return {
				valid: false,
				hint: 'Must be longer than 8 characters',
				pw: false,
				email: true,
				display: true,
				username: true
			};
		if (!username_regex.exec(data.username))
			return {
				valid: false,
				hint: 'Username must only contain normal letters and numbers and must be between 4 and 30 letters.',
				pw: true,
				email: true,
				display: true,
				username: false
			};
		if (!display_name_regex.exec(data.display_name))
			return {
				valid: false,
				hint: 'Display Name must be between 4 and 30 letters.',
				pw: true,
				email: true,
				display: false,
				username: true
			};
		return { valid: true, hint: undefined, pw: true, email: true, display: true, username: true };
	});

	const submit = async () => {
		if (!valid.valid) {
			return;
		}
		loading = true;
		const res = await fetch('/api/v1/auth/create', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				password: data.pw1,
				email: data.email,
				display_name: data.display_name,
				username: data.username
			})
		});
		if (res.ok) {
			window.location.replace('/home');
		} else if (res.status === 409) {
			alert('Username or email already exists.');
			data = {
				email: '',
				pw1: '',
				pw2: '',
				username: '',
				display_name: ''
			};
		}
		loading = false;
	};
</script>

<svelte:head>
	<title>Register to FediPrint</title>
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
						autocomplete="new-password"
						required
						class="block transition w-full rounded-md border-2 p-2 text-gray-900 shadow-sm"
						class:border-red-800={!valid.pw}
						bind:value={data.pw1}
					/>
				</div>
				<label for="pw2" class="block">Reperat Password</label>
				<div class="mt-2">
					<input
						type="password"
						name="pw2"
						id="pw2"
						autocomplete="new-password"
						required
						class="block transition w-full rounded-md border-2 p-2 text-gray-900 shadow-sm"
						class:border-red-800={!valid.pw}
						bind:value={data.pw2}
					/>
				</div>
				<label for="username" class="block">Username</label>
				<div class="mt-2">
					<input
						type="text"
						name="username"
						id="username"
						autocomplete="username"
						required
						class="block transition w-full rounded-md border-2 p-2 text-gray-900 shadow-sm"
						class:border-red-800={!valid.username}
						bind:value={data.username}
					/>
				</div>
				<label for="displayname" class="block">Preffered (Display)name</label>
				<div class="mt-2">
					<input
						type="text"
						name="displayname"
						id="displayname"
						autocomplete="nickname"
						required
						class="block transition w-full rounded-md border-2 p-2 text-gray-900 shadow-sm"
						class:border-red-800={!valid.display}
						bind:value={data.display_name}
					/>
				</div>
				{valid.hint}
				<div class="flex mt-2">
					<button class="mx-auto p-2 rounded bg-c-lgreen" disabled={!valid.valid && !loading}
						>{#if loading}Loading...{:else}Submit!{/if}</button
					>
				</div>
			</div>
		</form>
	</section>
</div>
