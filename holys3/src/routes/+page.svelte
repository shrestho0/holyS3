<script lang="ts">
	import { ArrowRight, MoveRight, MoveRightIcon } from 'lucide-svelte';
	import FuckingButton from '$lib/ui/FuckingButton.svelte';
</script>

<div class="flex flex-col gap-4 px-8 mx-8">
	<div class="bg-[#c2fbd7] p-4 my-3 shadow-sm w-full flex flex-col rounded-xl">
		<div class="title text-2xl font-bold tracking-wide">Have you fucked yourself?</div>
		<div class="subtitle">The fucking way to fuck yourself is here</div>
		<div class="learn_more flex">Fuck more <ArrowRight /></div>
	</div>

	<div class="buttons">
		<div class=" w-full flex justify-around items-center pb-16">
			<a href="/manage_s3">
				<FuckingButton shadow_position="SW" wtclass="rotate-[3deg]">Browse S3</FuckingButton>
			</a>
			<a href="/test">
				<FuckingButton shadow_position="SE" wtclass="rotate-[-3deg]">TEST</FuckingButton>
			</a>
			<a href="/add_a_bucket">
				<FuckingButton shadow_position="S_" wtclass="rotate-[1deg]">Add A Bucket</FuckingButton>
			</a>

			<a href="/test">
				<FuckingButton shadow_position="SE" wtclass="rotate-[-3deg]">Saved Buckets</FuckingButton>
			</a>
		</div>
	</div>
</div>
