<script lang="ts">
	import { quintInOut } from 'svelte/easing';
	import { type TransitionConfig } from 'svelte/transition';

	export let value: number = 0;
	export let delay: number = 0;
	export let maxValue: number = 0;

	const formatDigit = (digit: number): string => (digit < 10 ? `0${digit}` : digit.toString());

	$: formattedValue = formatDigit(value);
	$: nextValue = formatDigit(value === maxValue ? 0 : value + 1);

	function flip(node: Element): TransitionConfig {
		return {
			delay,
			duration: 1100,
			css: (t: number) => {
				const easing = quintInOut(t);
				return `
					transform: rotateX(-${easing * 180}deg);
				`;
			},
		};
	}
</script>

<div class="container">
	<div class="card card-next">
		<div class="card-inner">
			<div class="card-front">
				<div class="card-digit">{nextValue}</div>
			</div>
		</div>
	</div>

	{#key formattedValue}
		<div class="card card-top">
			<div class="card-inner" in:flip>
				<div class="card-front">
					<div class="card-digit">{formattedValue}</div>
				</div>
				<div class="card-back">
					<div class="card-digit">{nextValue}</div>
				</div>
			</div>
		</div>
	{/key}

	<div class="card card-bottom">
		<div class="card-inner">
			<div class="card-front">
				<div class="card-digit">{formattedValue}</div>
			</div>
		</div>
	</div>
</div>

<style>
	.container {
		position: relative;
		width: 100%;
		height: 100%;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		font-family: monospace;
	}

	.card {
		width: 100%;
		height: 50%;
		perspective: 80cqh;
		position: relative;
	}

	.card-inner {
		position: relative;
		width: 100%;
		height: 100%;
		transform-style: preserve-3d;
		transform-origin: center bottom;
	}

	.card-front,
	.card-back {
		position: absolute;
		width: 100%;
		height: 100%;
		backface-visibility: hidden;
		container-type: size;
		overflow: hidden;
		background-color: oklch(var(--b3));
		color: white;
	}

	.card-top .card-front,
	.card-next .card-front {
		border-radius: 15% 15% 0 0 / 30%;
	}

	.card-top .card-back,
	.card-bottom .card-front {
		border-radius: 0 0 15% 15% / 30%;
	}

	.card-back {
		transform: rotateX(180deg);
	}

	.card-digit {
		font-size: 65cqw;
		font-weight: 900;
		position: absolute;
		left: 50%;
		transform: translateX(-50%);
		line-height: 65cqw;
		user-select: none;
		cursor: default;
	}

	.card-top {
		z-index: 1;
	}

	.card-bottom {
		z-index: 0;
	}

	.card-top .card-front .card-digit,
	.card-next .card-front .card-digit {
		top: 35%;
	}

	.card-next {
		position: absolute;
		top: 0;
		left: 0;
	}

	.card-top .card-back .card-digit {
		top: -65%;
	}

	.card-bottom .card-front .card-digit {
		top: -65%;
	}
</style>
