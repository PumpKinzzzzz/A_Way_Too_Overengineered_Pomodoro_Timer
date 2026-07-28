import type { SequenceType } from '../types/dtos';

function pad(n: number): string {
	return n.toString().padStart(2, '0');
}

/** "MM:SS" for a countdown display. */
export function formatCountdown(totalSeconds: number): string {
	const minutes = Math.floor(totalSeconds / 60);
	const seconds = totalSeconds % 60;
	return `${pad(minutes)}:${pad(seconds)}`;
}

/** "H:MM:SS" for cumulative elapsed time, which can exceed an hour. */
export function formatElapsed(totalSeconds: number): string {
	const hours = Math.floor(totalSeconds / 3600);
	const minutes = Math.floor((totalSeconds % 3600) / 60);
	const seconds = totalSeconds % 60;
	return `${hours}:${pad(minutes)}:${pad(seconds)}`;
}

export function minutesToSeconds(minutes: number): number {
	return minutes * 60;
}

export function secondsToMinutes(seconds: number): number {
	return Math.round(seconds / 60);
}

export function phaseLabel(phase: SequenceType): string {
	switch (phase) {
		case 'Work':
			return 'Work';
		case 'ShortBreak':
			return 'Short Break';
		case 'LongBreak':
			return 'Long Break';
	}
}
