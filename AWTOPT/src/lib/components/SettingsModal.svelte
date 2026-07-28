<script lang="ts">
	import SettingsForm from './SettingsForm.svelte';
	import ErrorBanner from './ErrorBanner.svelte';
	import { settingsStore } from '../stores/settings.svelte';
	import type { SettingsUpdateDto } from '../types/dtos';

	let {
		open,
		onClose
	}: {
		open: boolean;
		onClose: () => void;
	} = $props();

	async function handleSave(patch: SettingsUpdateDto) {
		await settingsStore.update(patch);
		if (!settingsStore.error) {
			onClose();
		}
	}
</script>

<div class="modal-overlay" class:active={open}>
	<div class="modal">
		<div class="modal-header">
			<h3>Settings</h3>
			<button class="modal-close" onclick={onClose} aria-label="Close">&times;</button>
		</div>
		<div class="modal-body">
			<ErrorBanner message={settingsStore.error} onDismiss={() => (settingsStore.error = null)} />
			{#if settingsStore.settings}
				{#key open}
					<SettingsForm settings={settingsStore.settings} onSave={handleSave} />
				{/key}
			{/if}
		</div>
	</div>
</div>
