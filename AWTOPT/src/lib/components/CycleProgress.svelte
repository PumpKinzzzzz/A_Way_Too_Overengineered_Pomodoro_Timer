<script lang="ts">
	import type { SequenceType } from '../types/dtos';
	import { phaseLabel } from '../utils/format';

	let {
		sequenceList,
		currentCycle
	}: {
		sequenceList: SequenceType[];
		currentCycle: number;
	} = $props();
</script>

<div class="cycle-progress">
	<div class="cycle-text">
		Cycle {Math.min(currentCycle + 1, sequenceList.length)} of {sequenceList.length}
	</div>
	<div class="segments">
		{#each sequenceList as phase, index (index)}
			<span
				class="segment"
				class:done={index < currentCycle}
				class:active={index === currentCycle}
				title={phaseLabel(phase)}
			></span>
		{/each}
	</div>
</div>

<style>
	.cycle-progress {
		text-align: center;
		margin: 1rem 0;
	}

	.cycle-text {
		font-family: var(--font-body);
		font-size: 0.9rem;
		color: var(--mossy-gray-500);
		margin-bottom: 0.5rem;
	}

	.segments {
		display: flex;
		justify-content: center;
		gap: 0.4rem;
	}

	.segment {
		display: inline-block;
		width: 0.6rem;
		height: 0.6rem;
		border-radius: 50%;
		background-color: var(--mossy-gray-200);
		transition: background-color var(--transition-fast);
	}

	.segment.done {
		background-color: var(--mossy-green-600);
	}

	.segment.active {
		background-color: var(--mossy-green);
		box-shadow: 0 0 0 3px rgba(135, 145, 71, 0.25);
	}
</style>
