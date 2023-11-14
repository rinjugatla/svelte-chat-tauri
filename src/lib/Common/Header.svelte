<script lang="ts" type="module">
	import { DarkMode } from 'flowbite-svelte';
	import { Navbar, NavBrand, Button } from 'flowbite-svelte';
	import { signInWithGoogle, signOutWithGoogle } from '$lib/firebase';
	import { UserId } from '$lib/store';
	import { onDestroy } from 'svelte';

	let uid: string;
	const unsubscribe = UserId.subscribe((id) => {
		uid = id;
	});
	onDestroy(() => {
		unsubscribe();
	});

	const logoutProcess = () => {
		signOutWithGoogle();
		document.location.href = `/chat`;
	}
</script>

<div class="fixed top-0 w-full z-10">
	<Navbar>
		<div>
			<NavBrand class="flex" href="/chat">
				<svg
					class="w-6 h-6 text-gray-800 dark:text-white"
					aria-hidden="true"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 12 20"
				>
					<path
						stroke="currentColor"
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M9.041 11.862A5 5 0 0 1 11 15.831V19M1 1v3.169a5 5 0 0 0 1.891 3.916M11 1v3.169a5 5 0 0 1-2.428 4.288l-5.144 3.086A5 5 0 0 0 1 15.831V19M1 3h10M1.399 6h9.252M2 14h8.652M1 17h10"
					/>
				</svg>
				<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white"
					>svelte-chat</span
				>
			</NavBrand>
		</div>
		<div class="flex justify-center items-center">
			<div class="mr-2">
				{#if uid}
					<Button 
						size="sm" 
						color="alternative" 
						on:click={logoutProcess}>Logout</Button>
				{:else}
					<Button 
						size="sm" 
						on:click={signInWithGoogle}>Login</Button>
				{/if}
			</div>
			<DarkMode size="sm" />
		</div>
	</Navbar>
</div>
