<script lang="ts">
	import type { TimerStatusDto, SettingsDto } from '../types/dtos';
	import { isRunning, describePaused } from '../types/dtos';
	import { formatCountdown, phaseLabel } from '../utils/format';

	let {
		status,
		settings
	}: {
		status: TimerStatusDto;
		settings: SettingsDto | null;
	} = $props();

	let awaitingPhase = $derived(describePaused(status, settings));

	let label = $derived.by(() => {
		if (status.state === 'Idle') return 'Idle';
		if (isRunning(status.state)) return phaseLabel(status.state.Running.sequence);
		if (status.state === 'Completed') return 'Completed';
		if (status.state === 'Paused') {
			return awaitingPhase ? `Ready to start ${phaseLabel(awaitingPhase)}` : 'Paused';
		}
		return '';
	});
</script>

<div class="timer-display">
	<div class="countdown">{formatCountdown(status.time_remaining)}</div>
	<div class="phase-label">{label}</div>
</div>

<style>
	.timer-display {
		text-align: center;
		padding: 2rem 0;
	}

	.countdown {
		font-family: var(--font-heading);
		font-weight: var(--font-weight-black);
		font-size: 4rem;
		color: var(--mossy-dark);
		line-height: 1;
	}

	.phase-label {
		margin-top: 0.5rem;
		font-family: var(--font-heading);
		font-weight: var(--font-weight-bold);
		font-size: 1.1rem;
		color: var(--mossy-green-700);
	}
</style>
