<script>
	import LocalFiles from '$lib/components/LocalFiles.svelte';
	import { ArrowRightCircle, UnfoldHorizontal } from 'lucide-svelte';

	let left_perc = 50;
	let right_perc = 50;

	let mouse_up_or_down = '';
	let touch_status = '';

	let mouse_down = false;
	/**
	 * @type {number}
	 */
	let temp_mouse_x;
	/**
	 * @type {any}
	 */
	let new_mouse_x;

	$: outerWidth = 0;
	$: outerHeight = 0;
	let debug_data = {};
	let debug_hide = true;
</script>

<svelte:window bind:outerHeight bind:outerWidth />
<!-- <div class="layouts bg-black m-4 rounded-lg">sdsd</div> -->

<section>
	<button
		class="show_hide"
		on:click={() => {
			debug_hide = !debug_hide;
		}}>{debug_hide ? 'show' : 'hide'} Debug</button
	>false

	<div class="inputs flex flex-col w-72 items-center {debug_hide ? 'hidden' : ''}">
		<div class=" flex items-center justify-center gap-4">
			Left Perc:&nbsp; {left_perc}
			<input
				on:input={() => {
					right_perc = 100 - left_perc;
				}}
				class=""
				type="range"
				bind:value={left_perc}
			/>
		</div>
		<div class=" flex items-center justify-center gap-4">
			Right Perc: {right_perc}
			<input
				on:input={() => {
					left_perc = 100 - right_perc;
				}}
				class=" "
				type="range"
				bind:value={right_perc}
			/>
		</div>
	</div>
	debug data:
	{JSON.stringify(debug_data, null, 2)}
</section>

<div class="flex m-4 gap-3">
	<section class="s3_files relative border p-8 resize-x" style="width: {left_perc}%;">
		<div class="mockup-code">
			<div class="p-2 break-all tracking-tighter">
				jinish ta transaction type er hobe, koyeta instruction niye pore shob eksathe execute korbe.
				ekta selected list thakbe, upload selected, selected s3 location ee upload hoye jabe.
			</div>
		</div>
	</section>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		draggable={true}
		on:dragend={(e) => {
			// debug_data.clientX = e.clientX;
			// debug_data.pageWidth = outerWidth;
			// dragged percentage
			// debug_data.percentage = Math.round((e.clientX / outerWidth) * 100) - 2;
			let val = (e.clientX / outerWidth) * 100;
			left_perc = val + (val >= 50 ? 0 : -2);
			right_perc = 98 - left_perc;
			// debug_data.drag_end = true;
		}}
		class="resizer w-[2%] hover:bg-blue-200 flex flex-col items-center justify-center"
	>
		<UnfoldHorizontal />
	</div>

	<section class="local_files border p-8 h-full" style="width: {right_perc}%;">
		<div class="title text-lg">Local file browser</div>

		<LocalFiles />
	</section>
</div>

<!-- <br />Temp: {temp_mouse_x}
<br />New : {new_mouse_x}
<br />Mouse Moves: {mouse_up_or_down}
<br />Touch Status: {touch_status} -->

<style>
</style>
