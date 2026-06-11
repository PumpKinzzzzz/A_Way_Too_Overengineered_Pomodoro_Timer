import { invoke } from '@tauri-apps/api/core';

// Types correspondant aux DTOs Rust

export interface TimerStatusDto {
	state: 'Idle' | 'Running' | 'Paused' | 'WorkComplete' | 'BreakComplete';
	remaining_seconds: number;
	total_seconds: number;
	current_phase: 'Work' | 'ShortBreak' | 'LongBreak';
}

export interface SettingsDto {
	work_duration_minutes: number;
	short_break_minutes: number;
	long_break_minutes: number;
	sessions_until_long_break: number;
	auto_start_breaks: boolean;
	auto_start_work: boolean;
	notifications_enabled: boolean;
}

export interface SettingsUpdateDto {
	work_duration_minutes?: number;
	short_break_minutes?: number;
	long_break_minutes?: number;
	sessions_until_long_break?: number;
	auto_start_breaks?: boolean;
	auto_start_work?: boolean;
	notifications_enabled?: boolean;
}

export interface SessionStatsDto {
	completed_pomodoros: number;
	completed_short_breaks: number;
	completed_long_breaks: number;
	total_work_seconds: number;
	total_break_seconds: number;
}

// API Functions

export async function startTimer(): Promise<TimerStatusDto> {
	return await invoke<TimerStatusDto>('start_timer');
}

export async function pauseTimer(): Promise<TimerStatusDto> {
	return await invoke<TimerStatusDto>('pause_timer');
}

export async function resumeTimer(): Promise<TimerStatusDto> {
	return await invoke<TimerStatusDto>('resume_timer');
}

export async function resetTimer(): Promise<TimerStatusDto> {
	return await invoke<TimerStatusDto>('reset_timer');
}

export async function tick(): Promise<TimerStatusDto> {
	return await invoke<TimerStatusDto>('tick');
}

export async function updateSettings(settings: SettingsUpdateDto): Promise<SettingsDto> {
	return await invoke<SettingsDto>('update_settings', { request: settings });
}

export async function getSettings(): Promise<SettingsDto> {
	return await invoke<SettingsDto>('get_settings');
}

export async function getTimerStatus(): Promise<TimerStatusDto> {
	return await invoke<TimerStatusDto>('get_timer_status');
}

export async function getSessionStats(): Promise<SessionStatsDto> {
	return await invoke<SessionStatsDto>('get_session_stats');
}
