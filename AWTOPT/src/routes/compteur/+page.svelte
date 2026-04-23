<script lang="ts">
	let count = $state(0);

	function increment() {
		count += 1;
	}
</script>

<main>
	<h1>Page Compteur</h1>

	<p>Le compteur est à : {count}</p>

	<button onclick={increment}> +1 </button>

	<nav>
		<a href="/">Retour à l'accueil</a>
	</nav>
</main>

<style>
	main {
		text-align: center;
		padding: 2em;
	}

	button {
		font-size: 1.5em;
		padding: 0.5em 1em;
		margin: 1em 0;
		cursor: pointer;
	}

	nav {
		margin-top: 2em;
	}
</style>
