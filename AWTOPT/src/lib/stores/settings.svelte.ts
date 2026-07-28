import * as api from '../api';
import type { SettingsDto, SettingsUpdateDto } from '../types/dtos';

class SettingsStore {
	settings: SettingsDto | null = $state(null);
	loading: boolean = $state(false);
	error: string | null = $state(null);

	async load() {
		this.loading = true;
		try {
			this.settings = await api.getSettings();
			this.error = null;
		} catch (err) {
			this.error = String(err);
		} finally {
			this.loading = false;
		}
	}

	async update(patch: SettingsUpdateDto) {
		this.loading = true;
		try {
			this.settings = await api.updateSettings(patch);
			this.error = null;
		} catch (err) {
			this.error = String(err);
		} finally {
			this.loading = false;
		}
	}
}

export const settingsStore = new SettingsStore();
