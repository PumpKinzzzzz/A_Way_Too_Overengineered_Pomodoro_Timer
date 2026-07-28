import * as api from '../api';
import { isRunning, type TimerStatusDto } from '../types/dtos';

const idleStatus: TimerStatusDto = { state: 'Idle', time_remaining: 0, current_cycle: 0 };

class TimerStore {
	status: TimerStatusDto = $state(idleStatus);
	error: string | null = $state(null);
	#intervalId: ReturnType<typeof setInterval> | null = null;

	async refresh() {
		await this.#run(() => api.getTimerStatus());
	}

	async start() {
		await this.#run(() => api.startTimer());
	}

	async pause() {
		await this.#run(() => api.pauseTimer());
	}

	async resume() {
		await this.#run(() => api.resumeTimer());
	}

	async reset() {
		await this.#run(() => api.resetTimer());
	}

	async #tickOnce() {
		await this.#run(() => api.tick());
	}

	async #run(action: () => Promise<TimerStatusDto>) {
		try {
			this.status = await action();
			this.error = null;
		} catch (err) {
			this.error = String(err);
		}
		this.#syncInterval();
	}

	// The single place deciding whether the tick loop runs: the interval is
	// running iff the timer is Running, enforced after every status update.
	#syncInterval() {
		const running = isRunning(this.status.state);
		if (running && this.#intervalId === null) {
			this.#intervalId = setInterval(() => {
				void this.#tickOnce();
			}, 1000);
		} else if (!running && this.#intervalId !== null) {
			clearInterval(this.#intervalId);
			this.#intervalId = null;
		}
	}
}

export const timerStore = new TimerStore();
