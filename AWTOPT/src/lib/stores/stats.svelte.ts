import * as api from '../api';
import type { SessionStatsDto } from '../types/dtos';

class StatsStore {
	stats: SessionStatsDto | null = $state(null);
	loading: boolean = $state(false);
	error: string | null = $state(null);

	async refresh() {
		this.loading = true;
		try {
			this.stats = await api.getSessionStats();
			this.error = null;
		} catch (err) {
			this.error = String(err);
		} finally {
			this.loading = false;
		}
	}
}

export const statsStore = new StatsStore();
