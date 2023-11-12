<script lang="ts">
	export let shadow_position: 'NW' | 'SW' | any;
	export let wtclass: string = '';
	// let shadow_x: any = +8 ? shadow_position[0] == 'N' : shadow_position[0] == 'S' ? -8 : 0;
	// let shadow_y: any = +8 ? shadow_position[1] == 'E' : shadow_position[1] == 'W' ? -8 : 0;
	export let type: 'button' | 'reset' | 'submit' | null = 'button';
</script>

<div class={wtclass}>
	<button
		{type}
		on:click
		on:keypress
		class="the-fucking-button bg-white shadow-sm p-4 border-2 border-black font-semibold {shadow_position[0] ==
		'S'
			? 'after:translate-y-[8px]'
			: shadow_position[0] == 'S'
			? 'after:translate-y-[-8px]'
			: ''} 
		
		{shadow_position[1] == 'E'
			? 'after:translate-x-[8px]'
			: shadow_position[1] == 'W'
			? 'after:translate-x-[-8px]'
			: ''}"
	>
		<slot />
	</button>
</div>

<style>
	/* CSS */
	.the-fucking-button {
		align-items: center;
		/* background-color: #fee6e3; */
		border: 2px solid #111;
		border-radius: 8px;
		box-sizing: border-box;
		color: #111;
		cursor: pointer;
		display: flex;
		font-family: Inter, sans-serif;
		font-size: 16px;
		height: 48px;
		justify-content: center;
		line-height: 24px;
		max-width: 100%;
		padding: 0 25px;
		position: relative;
		text-align: center;
		text-decoration: none;
		user-select: none;
		-webkit-user-select: none;
		touch-action: manipulation;
	}

	.the-fucking-button:after {
		background-color: #111;
		border-radius: 8px;
		content: '';
		display: block;
		height: 48px;
		left: 0;
		width: 100%;
		position: absolute;
		top: -2px;
		/* transform: translate({shadow_x}px, 8px); */
		transition: transform 0.2s ease-out;
		z-index: -1;
	}

	.the-fucking-button:hover:after {
		transform: translate(0, 0);
	}

	.the-fucking-button:active {
		background-color: #ffdeda;
		outline: 0;
	}

	.the-fucking-button:hover {
		outline: 0;
	}

	@media (min-width: 768px) {
		.the-fucking-button {
			padding: 0 40px;
		}
	}
</style>
