<script>
	import '../../app.postcss';

	import Header from '$lib/Common/Header.svelte';
</script>

<main class="bg-white dark:bg-gray-400 h-screen">
	<Header />
	<div class="pt-16 pb-4 z-20 bg-white dark:bg-gray-400">
		<slot />
	</div>
</main>