<script>
	import '../../app.postcss';

	import Header from '$lib/Common/Header.svelte';
</script>

<main class="bg-white dark:bg-gray-400">
	<Header />
	<div class="pt-16 z-20">
		<slot />
	</div>
</main>

<style>
	main {
		height: 100vh;
	}
</style>