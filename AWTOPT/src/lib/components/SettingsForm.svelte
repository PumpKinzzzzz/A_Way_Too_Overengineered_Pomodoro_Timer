<script lang="ts">
	import type { SettingsDto, SettingsUpdateDto } from '../types/dtos';
	import { phaseLabel } from '../utils/format';

	let {
		settings,
		onSave
	}: {
		settings: SettingsDto;
		onSave: (patch: SettingsUpdateDto) => void;
	} = $props();

	let workDuration = $state(settings.work_duration);
	let shortBreakDuration = $state(settings.short_break_duration);
	let longBreakDuration = $state(settings.long_break_duration);
	let autoStartBreaks = $state(settings.auto_start_breaks);

	function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		const patch: SettingsUpdateDto = {};
		if (workDuration !== settings.work_duration) patch.work_duration = workDuration;
		if (shortBreakDuration !== settings.short_break_duration) {
			patch.short_break_duration = shortBreakDuration;
		}
		if (longBreakDuration !== settings.long_break_duration) {
			patch.long_break_duration = longBreakDuration;
		}
		if (autoStartBreaks !== settings.auto_start_breaks) patch.auto_start_breaks = autoStartBreaks;
		onSave(patch);
	}
</script>

<form onsubmit={handleSubmit}>
	<div class="form-group">
		<label for="work-duration">Work duration (minutes)</label>
		<input id="work-duration" type="number" min="1" bind:value={workDuration} />
	</div>
	<div class="form-group">
		<label for="short-break-duration">Short break duration (minutes)</label>
		<input id="short-break-duration" type="number" min="1" bind:value={shortBreakDuration} />
	</div>
	<div class="form-group">
		<label for="long-break-duration">Long break duration (minutes)</label>
		<input id="long-break-duration" type="number" min="1" bind:value={longBreakDuration} />
	</div>
	<div class="form-group">
		<input id="auto-start-breaks" type="checkbox" bind:checked={autoStartBreaks} />
		<label for="auto-start-breaks">Auto-start breaks</label>
		<span class="form-help">
			When off, the timer pauses at the start of each break and waits for you to press Start.
		</span>
	</div>
	<div class="form-group">
		<span class="sequence-label">Cycle sequence</span>
		<p class="sequence-list">
			{#each settings.sequence_list as phase, index (index)}
				<span class="sequence-step">{phaseLabel(phase)}</span>
				{#if index < settings.sequence_list.length - 1}
					<span class="sequence-arrow" aria-hidden="true">&rarr;</span>
				{/if}
			{/each}
		</p>
		<span class="form-help">Editing the sequence order isn't supported yet.</span>
	</div>
	<button type="submit" class="button button-primary button-block">Save</button>
</form>

<style>
	.sequence-label {
		display: block;
		margin-bottom: 0.4rem;
		font-weight: var(--font-weight-bold);
		font-family: var(--font-heading);
	}

	.sequence-list {
		margin: 0 0 0.3rem 0;
		font-family: var(--font-body);
	}

	.sequence-arrow {
		color: var(--mossy-gray-400);
		margin: 0 0.3rem;
	}
</style>
