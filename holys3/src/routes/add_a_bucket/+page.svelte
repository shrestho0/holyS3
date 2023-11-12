<script lang="ts">
	import FuckingButton from '$lib/ui/FuckingButton.svelte';
	import { ArrowRight, Tags } from 'lucide-svelte';

	let bucket_info = {
		unique_name: crypto.randomUUID().slice(1, 6),
		full_endpoint: false,
		endpoint: 'https://',
		region: 'whatever',
		access_key: 'YOUR_ACCESS_KEY',
		secret_key: 'YOUR_SECRET_KEY',
		groups: {}, // if new group add to db as new group
		generateEndpoint() {
			if (this.full_endpoint) {
				return;
			}
			this.endpoint = 'shit';
		}
	};
</script>

<div class="flex flex-col gap-4 px-8 mx-8">
	<div class="bg-purple-200 p-4 my-3 shadow-sm w-full flex flex-col rounded-xl">
		<div class="title text-2xl font-bold tracking-wide">Have you fucked yourself?</div>
		<form on:submit|preventDefault class="form flex flex-col gap-3 max-w-md">
			<input
				class="input input-sm"
				type="text"
				bind:value={bucket_info.unique_name}
				placeholder="Unique name"
			/>

			<input
				class="checkbox"
				type="checkbox"
				bind:checked={bucket_info.full_endpoint}
				placeholder="Full endpoint"
			/>
			<input
				class="input input-sm"
				type="text"
				bind:value={bucket_info.endpoint}
				placeholder="endpoint"
			/>
			<input
				class="input input-sm"
				type="text"
				bind:value={bucket_info.region}
				placeholder="region"
			/>
			<input
				class="input input-sm"
				type="text"
				bind:value={bucket_info.access_key}
				placeholder="access key"
			/>
			<input
				class="input input-sm"
				type="text"
				bind:value={bucket_info.secret_key}
				placeholder="secret"
			/>
			<div class="flex gap-4 w-full justify-between">
				<div class="right flex gap-2">
					<FuckingButton shadow_position="SW" type="reset" wtclass="relative z-10"
						>Clear</FuckingButton
					>
					<FuckingButton
						shadow_position="SE"
						on:click={() => {
							alert('fucker');
						}}
						wtclass="relative z-10">Test Connection</FuckingButton
					>
					<FuckingButton shadow_position="SE" type="submit" wtclass="relative z-10"
						>Add Bucket/S3</FuckingButton
					>
				</div>
			</div>
			<div class="left">
				<FuckingButton shadow_position="" wtclass="relative z-10">
					<Tags /> Tags</FuckingButton
				>
			</div>
		</form>
	</div>
</div>
