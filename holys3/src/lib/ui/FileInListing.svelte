<script lang="ts">
	import { FileIcon } from 'lucide-svelte';
	export let fileName: String = '';
</script>

<div class="flex gap-2 items-center">
	<FileIcon size="20" />
	{fileName}
</div>
