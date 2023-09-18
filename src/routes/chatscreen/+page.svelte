<script lang="ts">
	import { emit, listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';

	let unlisten: any;
	let recieve_chats: {message: string, time: string}[] = [];
	onMount(async () => {
		unlisten = await listen('recieve_chat', (event: any) => {
			recieve_chats = event.payload;
		});
	});

	onDestroy(() => {
		if (unlisten) {
			unlisten();
		}
	});
</script>

<div>
	{#each recieve_chats as chat}
		 <div>
			<p>{chat.message}</p> 
			<p>{chat.time}</p>
		 </div>
	{/each}
</div>