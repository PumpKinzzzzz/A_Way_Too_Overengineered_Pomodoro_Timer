<script lang="ts">
	import type { TimerStatusDto, SettingsDto } from '../types/dtos';
	import { deriveControlState, describePaused } from '../types/dtos';
	import { phaseLabel } from '../utils/format';

	let {
		status,
		settings,
		onStart,
		onPause,
		onResume,
		onReset
	}: {
		status: TimerStatusDto;
		settings: SettingsDto | null;
		onStart: () => void;
		onPause: () => void;
		onResume: () => void;
		onReset: () => void;
	} = $props();

	let controls = $derived(deriveControlState(status.state));
	let awaitingPhase = $derived(describePaused(status, settings));
	let resumeLabel = $derived(awaitingPhase ? `Start ${phaseLabel(awaitingPhase)}` : 'Resume');
</script>

<div class="timer-controls">
	{#if controls.canStart}
		<button class="button button-primary" onclick={onStart}>Start</button>
	{/if}
	{#if controls.canPause}
		<button class="button button-secondary" onclick={onPause}>Pause</button>
	{/if}
	{#if controls.canResume}
		<button class="button button-primary" onclick={onResume}>{resumeLabel}</button>
	{/if}
	{#if controls.canReset}
		<button class="button button-danger" onclick={onReset}>Reset</button>
	{/if}
</div>

<style>
	.timer-controls {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		margin: 1.5rem 0;
	}
</style>
