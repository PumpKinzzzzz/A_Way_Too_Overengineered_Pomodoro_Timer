<script lang="ts">
	import type { SessionStatsDto } from '../types/dtos';
	import { formatElapsed } from '../utils/format';

	let { stats }: { stats: SessionStatsDto } = $props();
</script>

<div class="card">
	<div class="card-header">Session Stats</div>
	<div class="card-body">
		<p><strong>Completed cycles:</strong> {stats.completed_cycles}</p>
		<p><strong>Time elapsed:</strong> {formatElapsed(stats.time_elapsed)}</p>
	</div>
</div>
