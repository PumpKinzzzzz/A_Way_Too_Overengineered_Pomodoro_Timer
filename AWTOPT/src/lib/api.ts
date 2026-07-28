import { invoke } from '@tauri-apps/api/core';
import type { TimerStatusDto, SettingsDto, SettingsUpdateDto, SessionStatsDto } from './types/dtos';

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

export async function skipToNext(): Promise<TimerStatusDto> {
	return await invoke<TimerStatusDto>('skip_to_next');
}
