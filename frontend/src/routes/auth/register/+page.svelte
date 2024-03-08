<svelte:options runes={true} />

<script lang="ts">
	import '@fontsource/marck-script';

	let data = $state({
		email: '',
		pw1: '',
		pw2: ''
	});
	let valid = $derived.by(() => {
		if (data.email.length < 8)
			return { valid: false, hint: 'Shorter than 8 characters', email: false, pw: true};
		if (data.pw1.length < 8) return { valid: false, hint: 'Do not match', pw: false, email: true};
		if (data.pw1 !== data.pw2)
			return { valid: false, hint: 'Must be longer than 8 characters', pw: false, email: true};
		return { valid: true, hint: undefined, pw: true, email: true};
	});
</script>

<svelte:head>
	<title>Register to FediPrint</title>
</svelte:head>

<div class="w-screen h-screen flex bg-red">
	<section class="m-auto w-1/3 h-fit bg-white/60 rounded shadow-xl p-2">
		<h1 class="text-center text-3xl">
			Let's get you into <span class="marck-script text-4xl">FediPrint!</span>
		</h1>
		<form>
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
						bind:value={data.pw1}
					/>
				</div>
				<label for="pw2" class="block">Reperat Password</label>
				<div class="mt-2">
					<input
						type="password"
						name="pw2"
						id="pw2"
						autocomplete="email"
						required
						class="block transition w-full rounded-md border-2 p-2 text-gray-900 shadow-sm"
                        class:border-red-800={!valid.pw}
						bind:value={data.pw2}
					/>
				</div>
				<div class="flex mt-2">
					<button class="mx-auto p-2 rounded" disabled={!valid.valid}>Submit!</button>
				</div>
			</div>
		</form>
	</section>
</div>
