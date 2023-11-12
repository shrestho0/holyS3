<script lang="ts">
	import DirectoryInListing from '$lib/ui/DirectoryInListing.svelte';
	import FileInListing from '$lib/ui/FileInListing.svelte';
	import { readDir, BaseDirectory } from '@tauri-apps/api/fs';
	import { invoke } from '@tauri-apps/api/tauri';
	import {
		ArrowLeftCircle,
		ArrowRightCircle,
		Cog,
		FileIcon,
		FolderIcon,
		LucideSettings,
		Settings,
		Settings2
	} from 'lucide-svelte';
	import { onMount } from 'svelte';
	import toast from 'svelte-french-toast';

	const conf = {
		show_conf_section: false,
		show_hidden_files: true
	};

	let root = '';
	let nav_val = '/';
	let child_level = '';

	let parent_list: any[] = [];
	let child_list: any = [];

	let debugX: any = { entries: {} };
	let child_entries: any[] = [];
	let browsing = {
		parent: ''
	};
	onMount(async () => {
		await handleNavigateClick();
	});
	let level_stack = [];

	async function getFilesStuff(dir: string) {
		// toast(`req: ${dir}`);

		const isValid = await invoke('check_file_or_directory', {
			path: dir
		});
		if (isValid == -1) {
			return [false, 'File/Directory does not exists'];
		}
		// debugX.loc = nav_val;
		// debugX.isValid = isValid;
		nav_val = dir;
		const entries = await readDir(dir, { recursive: false });
		return [true, entries];
		// debugX.ENTRIES = entries;
		// parent_list = [...entries];
		// processEntries(entries);
	}

	// function processEntries(entries: any) {
	// 	for (const entry of entries) {
	// 		console.log(`Entry: ${entry.path}`);
	// 		if (entry.children) {
	// 			processEntries(entry.children);
	// 		}
	// 	}
	// }

	async function handleNavigatePop() {
		if (nav_val == '/') return;
		let x = nav_val.split('/');
		x.pop();

		if (x?.length > 0) {
			nav_val = x.join('/');
			handleNavigateClick();
		}
	}
	async function handleNavigateClick() {
		child_list = [];
		if (!nav_val) {
			nav_val = '/';
		}
		const [valid, dataOrMsg]: any[] = await getFilesStuff(root + nav_val);
		if (valid) {
			parent_list = dataOrMsg;
		} else {
			toast('not valid');
		}
	}
	async function handleParentClick(parent: string) {
		const [valid, dataOrMsg]: any[] = await getFilesStuff(parent);

		if (valid) {
			child_list = dataOrMsg;
			child_level = parent;
			browsing.parent = parent;
			// parent_list = child_list;
			// root = parent;
		} else {
			toast('not valid');
		}
		// debugX.valid = valid;
		// debugX.data = dataOrMsg;
	}
	async function handleChildClick(child: string) {
		parent_list = child_list;
		child_list = [];
		const [valid, dataOrMsg]: any[] = await getFilesStuff(child);

		if (valid) {
			child_list = dataOrMsg;
			browsing.parent;
		} else {
			toast('not valid');
		}
	}
</script>

<!-- <pre> -->
<!-- </pre> -->
<button
	class="flex items-center gap-2"
	on:click={() => {
		conf.show_conf_section = !conf.show_conf_section;
	}}
>
	{#if conf.show_conf_section}
		<Settings />
	{:else}<Cog />{/if} Settings</button
>
<section
	class="conf transition ease-in-out delay-150 border my-1"
	class:hidden={conf.show_conf_section}
>
	<div class="flex gap-2 items-center p-4">
		<label class=" text-lg" for="show_hidden_files">show_hidden_files</label>
		<input
			class="checkbox"
			type="checkbox"
			name="show_hidden_files"
			bind:checked={conf.show_hidden_files}
		/>
	</div>
</section>
<div
	class="location flex items-center gap-2 w-full bg-gray-100 font-normal font-sans p-2 rounded-xl"
>
	<button on:click={handleNavigatePop} class="btn btn-square btn-sm">
		<ArrowLeftCircle />
	</button>
	<input
		class=" bg-transparent border-black border-b-2 w-full focus:outline-none"
		type="text"
		on:keydown={(e) => {
			e.code === 'Enter' ? handleNavigateClick() : null;
		}}
		bind:value={nav_val}
	/>
	<button on:click={handleNavigateClick} class="btn btn-square btn-sm">
		<ArrowRightCircle />
	</button>
</div>
<div class="p-3 my-3 bg-gray-100 rounded-lg grid grid-cols-2">
	<section class="parent_level bg-purple-200 h-[60vh] overflow-y-scroll">
		Parent Level Listing
		{#each parent_list as dir}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- <div
				on:click={() => handleParentClick(dir.path)}
				class={browsing.parent == dir.path ? 'bg-red-200' : ''}
			/> -->
			{#if dir.children == undefined}
				<FileInListing fileName={dir.name} />
			{:else}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<!-- <div
					class={browsing.parent == dir.path ? 'bg-red-200' : ''}
					on:click={() => handleParentClick(dir.path)}
				/> -->
				<DirectoryInListing
					on:click={() => handleParentClick(dir.path)}
					active={browsing.parent == dir.path}
					dirName={dir.name}
				/>
			{/if}
		{/each}
		<div />
	</section>
	<section class="parent_level bg-purple-300 h-[60vh] overflow-y-scroll">
		Child Level Listing
		{#each child_list as dir}
			{#if dir.children == undefined}
				<FileInListing fileName={dir.name} />
			{:else}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->

				<DirectoryInListing
					active={false}
					dirName={dir.name}
					on:click={() => handleChildClick(dir.path)}
				/>
			{/if}
		{/each}
	</section>
</div>
