<script lang='ts'>
    // top position
    export let top: number;
    // second
    export let animationSpeed: number;
    // hex ex: #11ff33
    export let color: string;
	export let fontSize: number;
	export let message: string;

	const cssTop = `${top}%`;
	const cssAnimation = `slide ${animationSpeed}s linear 0s 1 normal forwards`;
	const cssColor = color;
	const cssFontSize = `${fontSize}px`;
</script>

<div class="niconico" style="--my-top: {cssTop}; --my-animation: {cssAnimation}; --my-color: {cssColor}; --my-font-size: {cssFontSize};">
	{message}
</div>

<style>
    .niconico {
		z-index: 2147483647;
		position: absolute;
		white-space: nowrap;
		-webkit-text-stroke: 0.5px white;
		animation: var(--my-animation);
		font-size: var(--my-font-size);
		color: var(--my-color);
		top: var(--my-top);
	}
	@keyframes -global-slide {
		0% { right: 0; }
		100% { right: 100%; display: none; }
	}
</style>