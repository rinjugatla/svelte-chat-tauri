<script lang="ts">
	import { emit, listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';

	let unlisten: any;
	let recieve_chats: string[] = [];
	onMount(async () => {
		unlisten = await listen('recieve_chat', (event: {payload: string}) => {
			recieve_chats.push(event.payload);
			recieve_chats = recieve_chats
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
			{chat}
		 </div>
	{/each}
</div>