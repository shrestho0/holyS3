<script lang="ts">
	import { page } from '$app/stores';
	import { ChevronLeftCircle } from 'lucide-svelte';
</script>

<div class="select-none flex flex-col items-stretch justify-between w-full h-screen">
	{#if $page.url.pathname != '/'}
		<header class="h-10 bg-gray-300">
			<div class="home">
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div on:click|preventDefault={() => history.back()} on:keypress={() => console.log()}>
					<ChevronLeftCircle />
				</div>
			</div>
		</header>
	{/if}
	<main class="flex-1">
		<slot />
	</main>
	<!-- {#if $page.url.pathname != '/'}
		<footer>Copyright (c) mf</footer>
	{/if} -->
</div>
