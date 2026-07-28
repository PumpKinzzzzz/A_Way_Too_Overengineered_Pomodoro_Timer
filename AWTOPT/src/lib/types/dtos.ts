// DTOs mirroring AWTOPT/src-tauri/src/contracts/dtos.rs exactly (snake_case, no serde renaming).

export type SequenceType = 'Work' | 'ShortBreak' | 'LongBreak';

// TimerState is an externally-tagged Rust enum: unit variants serialize as bare
// strings, the one struct variant (Running) serializes as a nested object.
export type TimerStateDto =
	| 'Idle'
	| 'Paused'
	| 'Completed'
	| { Running: { sequence: SequenceType } };

export interface TimerStatusDto {
	state: TimerStateDto;
	time_remaining: number; // seconds
	current_cycle: number; // index into sequence_list
}

export interface SettingsDto {
	work_duration: number; // minutes
	short_break_duration: number; // minutes
	long_break_duration: number; // minutes
	auto_start_breaks: boolean;
	sequence_list: SequenceType[];
}

export interface SettingsUpdateDto {
	work_duration?: number;
	short_break_duration?: number;
	long_break_duration?: number;
	auto_start_breaks?: boolean;
	sequence_list?: SequenceType[];
}

export interface SessionStatsDto {
	time_elapsed: number; // seconds, cumulative
	completed_cycles: number;
	date: string;
}

export function isRunning(state: TimerStateDto): state is { Running: { sequence: SequenceType } } {
	return typeof state === 'object' && state !== null && 'Running' in state;
}

export function runningPhase(state: TimerStateDto): SequenceType | null {
	return isRunning(state) ? state.Running.sequence : null;
}

export interface ControlState {
	canStart: boolean;
	canPause: boolean;
	canResume: boolean;
	canReset: boolean;
}

export function deriveControlState(state: TimerStateDto): ControlState {
	if (state === 'Idle') {
		return { canStart: true, canPause: false, canResume: false, canReset: false };
	}
	if (isRunning(state)) {
		return { canStart: false, canPause: true, canResume: false, canReset: true };
	}
	if (state === 'Paused') {
		return { canStart: false, canPause: false, canResume: true, canReset: true };
	}
	// Completed
	return { canStart: false, canPause: false, canResume: false, canReset: true };
}

function durationForPhase(settings: SettingsDto, phase: SequenceType): number {
	switch (phase) {
		case 'Work':
			return settings.work_duration * 60;
		case 'ShortBreak':
			return settings.short_break_duration * 60;
		case 'LongBreak':
			return settings.long_break_duration * 60;
	}
}

// `Paused` covers both "user paused mid-phase" and "waiting for the user to
// manually start the next auto_start_breaks-blocked phase" (see timer_worker.rs).
// Heuristic: if time_remaining exactly matches the upcoming phase's full
// duration, treat it as "ready to start" rather than a plain mid-phase pause.
export function describePaused(
	status: TimerStatusDto,
	settings: SettingsDto | null
): SequenceType | null {
	if (status.state !== 'Paused' || !settings) {
		return null;
	}
	const upcoming = settings.sequence_list[status.current_cycle];
	if (upcoming && status.time_remaining === durationForPhase(settings, upcoming)) {
		return upcoming;
	}
	return null;
}
