<script lang="ts">
	import { FolderIcon } from 'lucide-svelte';
	export let dirName: string = '';
	export let active: boolean;
	console.log('DEBUG', dirName, active);
</script>

<div
	on:click
	on:keypress
	role="button"
	tabindex="-69"
	class="text-lg {active ? 'bg-blue-200' : ' '}"
>
	<div class="flex gap-2 items-center">
		<FolderIcon size="20" />
		{dirName}
	</div>
</div>
