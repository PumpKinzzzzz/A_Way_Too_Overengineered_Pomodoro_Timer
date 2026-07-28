<script lang="ts">
	let {
		activeTab,
		onTabChange,
		onOpenSettings
	}: {
		activeTab: 'timer' | 'stats';
		onTabChange: (tab: 'timer' | 'stats') => void;
		onOpenSettings: () => void;
	} = $props();
</script>

<header class="top-bar">
	<span class="brand">Mossy Pomodoro</span>
	<nav class="tabs">
		<button
			class="button {activeTab === 'timer' ? 'button-primary' : 'button-secondary'}"
			onclick={() => onTabChange('timer')}
		>
			Timer
		</button>
		<button
			class="button {activeTab === 'stats' ? 'button-primary' : 'button-secondary'}"
			onclick={() => onTabChange('stats')}
		>
			Stats
		</button>
	</nav>
	<button
		class="button button-secondary settings-button"
		onclick={onOpenSettings}
		aria-label="Settings"
	>
		⚙
	</button>
</header>

<style>
	.top-bar {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1.5rem;
		background-color: var(--mossy-green);
		color: var(--mossy-cream);
	}

	.brand {
		font-family: var(--font-heading);
		font-weight: var(--font-weight-black);
		font-size: 1.1rem;
		margin-right: auto;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
	}

	.settings-button {
		font-size: 1.1rem;
		line-height: 1;
	}
</style>
