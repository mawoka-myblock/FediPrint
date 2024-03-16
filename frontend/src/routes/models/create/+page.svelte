<svelte:options runes={true} />

<script lang="ts">
	import 'filepond/dist/filepond.css';
	import 'filepond-plugin-image-preview/dist/filepond-plugin-image-preview.css';
	import FilePond, { registerPlugin, supported } from 'svelte-filepond';
	import FilePondPluginImageExifOrientation from 'filepond-plugin-image-exif-orientation';
	import FilePondPluginImagePreview from 'filepond-plugin-image-preview';
	import type { FilePondFile } from 'filepond';
	import '@fontsource/marck-script';
	import { name_to_license, Licenses } from '$lib/helpers/licenses';
	import { createTagsInput, melt } from '@melt-ui/svelte';

	const {
		elements: { root, input, tag, deleteTrigger, edit },
		states: { tags }
	} = createTagsInput({
		defaultTags: [],
		unique: true,
		add(tag) {
			const edited = tag.replaceAll('#', '');
			return { id: edited, value: edited };
		},
		addOnPaste: true
	});

	registerPlugin(FilePondPluginImageExifOrientation, FilePondPluginImagePreview);
	let pond = $state();
	const handleInit = () => {
		console.log('FilePond has initialised');
	};
	const handleAddFile = async (err, fileItem: FilePondFile) => {
		console.log('A file has been added', fileItem);
	};
	let name = 'pond';

	let data: {
		images: string[];
		files: string[];
		title: string;
		summary: string;
		description: string;
		tags: string[];
		license: Licenses;
	} = $state({
		images: [],
		files: [],
		title: '',
		summary: '',
		description: '',
		tags: [],
		license: Licenses.CcAttr
	});
	tags.subscribe((d) => {
		data.tags = []
		for (const t of d) {
			data.tags.push(t.value)
		}
	});
	let form: HTMLFormElement | undefined = $state();
	let valid = $derived.by(() => {
		data.title;
		data.description;
		data.license;
		data.summary;
		data.tags;
		try {
			return form?.checkValidity();
		} catch (e) {
			console.log(e);
			return false;
		}
	});
</script>

<div class="flex flex-col">
	<div class="flex flex-col w-1/3 mx-auto">
		<h1 class="mx-auto marck-script text-4xl my-4">Create your model!</h1>
		<div class="">
			<FilePond
				bind:this={pond}
				{name}
				server="/api"
				allowMultiple={true}
				oninit={handleInit}
				onaddfile={handleAddFile}
				credits={{}}
			/>
		</div>
		<form action="" bind:this={form}>
			<div class="p-4 rounded-lg shadow-2xl my-4">
				<label for="title" class="block">Title</label>
				<div class="mt-2">
					<input
						type="text"
						name="title"
						id="title"
						required
						class="block transition-all w-full rounded-md border-2 p-2 text-gray-900 shadow-sm outline-none focus:border-c-dgreen"
						bind:value={data.title}
					/>
				</div>
			</div>
			<div class="p-4 rounded-lg shadow-2xl my-4">
				<label for="summary" class="block">Summary</label>
				<div class="mt-2">
					<textarea
						name="summary"
						rows="2"
						id="summary"
						required
						autocomplete="on"
						spellcheck="true"
						maxlength="250"
						minlength="20"
						class="block transition-all w-full rounded-md border-2 p-2 text-gray-900 shadow-sm outline-none focus:border-c-dgreen resize-none overscroll-none"
						bind:value={data.summary}
					/>
				</div>
			</div>
			<div class="flex flex-col items-start justify-center gap-2 w-full">
				<div
					use:melt={$root}
					class="flex min-w-[280px] flex-row flex-wrap gap-2.5 rounded-md bg-white px-3 py-2 text-magnum-700
				  focus-within:ring focus-within:ring-magnum-400 w-full"
				>
					{#each $tags as t}
						<div
							use:melt={$tag(t)}
							class="flex items-center overflow-hidden rounded-md bg-magnum-200 text-magnum-900 [word-break:break-word]
					data-[disabled]:bg-magnum-300 data-[selected]:bg-magnum-400 data-[disabled]:hover:cursor-default
					  data-[disabled]:focus:!outline-none data-[disabled]:focus:!ring-0"
						>
							<span class="flex items-center border-r border-white/10 px-1.5">#{t.value}</span>
							<button
								use:melt={$deleteTrigger(t)}
								type="button"
								class="flex h-full items-center px-1 enabled:hover:bg-magnum-300 transition-all"
							>
								<!-- <X class="size-3" /> -->
								<svg
									class="h-3 w-3"
									data-slot="icon"
									aria-hidden="true"
									fill="none"
									stroke-width="3"
									stroke="currentColor"
									viewBox="0 0 24 24"
									xmlns="http://www.w3.org/2000/svg"
								>
									<path d="M6 18 18 6M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"
									></path>
								</svg>
							</button>
						</div>
						<div
							use:melt={$edit(t)}
							class="flex items-center overflow-hidden rounded-md px-1.5 [word-break:break-word] data-[invalid-edit]:focus:!ring-red-500 before:content-['#']"
						/>
					{/each}

					<input
						use:melt={$input}
						type="text"
						placeholder="Enter tags..."
						class="min-w-[4.5rem] shrink grow basis-0 border-0 text-black outline-none focus:!ring-0 data-[invalid]:text-red-500"
					/>
				</div>
			</div>

			<div class="p-4 rounded-lg shadow-2xl my-4">
				<label for="description" class="block">Description</label>
				<div class="mt-2">
					<textarea
						name="description"
						rows="20"
						id="description"
						required
						autocomplete="on"
						spellcheck="true"
						maxlength="5000"
						minlength="20"
						class="block transition-all w-full rounded-md border-2 p-2 text-gray-900 shadow-sm outline-none focus:border-c-dgreen resize-none overscroll-y-auto"
						style="field-sizing: content"
						bind:value={data.description}
					/>
				</div>
			</div>
			<div class="p-4 rounded-lg shadow-2xl my-4">
				<label for="license" class="block">License</label>
				<div class="mt-2 w-full">
					<select
						id="license"
						bind:value={data.license}
						class="p-2 rounded-lg transition-all focus:border-c-dgreen border-2 w-full"
					>
						{#each name_to_license as license}
							<option value={license.value}>{license.name}</option>
						{/each}
					</select>
				</div>
			</div>
			<div class="p-4 rounded-lg shadow-2xl my-4 flex">
				<button
					type="submit"
					class="w-full p-2 transition-all rounded-lg bg-c-brown hover:border-c-dgreen border-2 border-white disabled:opacity-50"
					disabled={!valid}>Submit</button
				>
			</div>
		</form>
	</div>
</div>
