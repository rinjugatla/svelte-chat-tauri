<script lang="ts">
	import AnimationChat from '$lib/Components/AnimationChat.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';

	let unlisten: any;
	let recieve_chats: {id: string, message: string, time: string}[] = [];
	onMount(async () => {
		unlisten = await listen('recieve_chat', (event: any) => {
			recieve_chats = event.payload;
		});
	});

	function generateRandom(min: number, max: number): number {
		const random = Math.floor( Math.random() * (max + 1 - min) ) + min;
		return random;
	}

	function generateRandomColor(): string {
		const r = convertDexToHex(generateRandom(0, 255));
		const g = convertDexToHex(generateRandom(0, 255));
		const b = convertDexToHex(generateRandom(0, 255));
		const rgb = `#${r}${g}${b}`;
		return rgb;
	}

	function convertDexToHex(dex: number): string {
		const hex = ('00' + dex.toString(16).toUpperCase()).substr(-2)
		return hex;
	}

	onDestroy(() => {
		if (unlisten) {
			unlisten();
		}
	});
</script>

<div>
	{#each recieve_chats as chat (chat.id)}
		<AnimationChat top={generateRandom(5, 95)} 
		animationSpeed={generateRandom(1, 10)} 
		color={generateRandomColor()} 
		fontSize={generateRandom(10, 20)} 
		message={chat.message} />
	{/each}	
</div>

<style>
	:global(:root) {
		background-color: hsla(0, 0%, 100%, 0.01);
    }
</style>