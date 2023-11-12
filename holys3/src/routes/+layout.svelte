<script>
	import { Toaster } from 'svelte-french-toast';
	import '../app.css';
	import { onMount } from 'svelte';
	import AppShell from '$lib/ui/AppShell.svelte';
	let loading = true;
	onMount(() => {
		loading = false;
	});
</script>

{#if !loading}
	<AppShell>
		<slot />
		<Toaster />
	</AppShell>
	<!-- {:else}
	<div class="w-full h-screen bg-black text-white flex items-center justify-center">
		Loading....
	</div> -->
{/if}
