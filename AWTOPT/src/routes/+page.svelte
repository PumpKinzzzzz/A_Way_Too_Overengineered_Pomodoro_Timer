<script lang="ts">
	import { onMount } from 'svelte';
	import '$lib/styles/vendor/mossy-grave.css';
	import TopBar from '$lib/components/TopBar.svelte';
	import TimerDisplay from '$lib/components/TimerDisplay.svelte';
	import CycleProgress from '$lib/components/CycleProgress.svelte';
	import TimerControls from '$lib/components/TimerControls.svelte';
	import StatsPanel from '$lib/components/StatsPanel.svelte';
	import SettingsModal from '$lib/components/SettingsModal.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { statsStore } from '$lib/stores/stats.svelte';

	let activeTab: 'timer' | 'stats' = $state('timer');
	let settingsOpen = $state(false);

	onMount(() => {
		void timerStore.refresh();
		void settingsStore.load();
		void statsStore.refresh();
	});

	function handleTabChange(tab: 'timer' | 'stats') {
		activeTab = tab;
		if (tab === 'stats') {
			void statsStore.refresh();
		}
	}
</script>

<TopBar {activeTab} onTabChange={handleTabChange} onOpenSettings={() => (settingsOpen = true)} />

<main>
	<ErrorBanner message={timerStore.error} onDismiss={() => (timerStore.error = null)} />

	{#if activeTab === 'timer'}
		<TimerDisplay status={timerStore.status} settings={settingsStore.settings} />
		<CycleProgress
			sequenceList={settingsStore.settings?.sequence_list ?? []}
			currentCycle={timerStore.status.current_cycle}
		/>
		<TimerControls
			status={timerStore.status}
			settings={settingsStore.settings}
			onStart={() => timerStore.start()}
			onPause={() => timerStore.pause()}
			onResume={() => timerStore.resume()}
			onReset={() => timerStore.reset()}
		/>
	{:else if statsStore.stats}
		<StatsPanel stats={statsStore.stats} />
	{/if}
</main>

<SettingsModal open={settingsOpen} onClose={() => (settingsOpen = false)} />

<style>
	main {
		max-width: 480px;
		margin: 0 auto;
		padding: 1.5rem;
	}
</style>
