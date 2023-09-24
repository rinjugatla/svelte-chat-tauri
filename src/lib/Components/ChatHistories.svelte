<script>
	import { invoke } from '@tauri-apps/api';
	import { onDestroy, onMount } from 'svelte';
	import { onSnapshotChats } from '$lib/api';
	import { SnapshotChats } from '$lib/store';
	import ChatHistory from './ChatHistory.svelte';

	export let roomId = '';
	/**
	 * @type {import('svelte/store').Unsubscriber}
	 */
	let unsubscribeStore;
	/**
	 * @type {import('firebase/firestore').Unsubscribe}
	 */
	let unsubscribeFirestore;
	/**
	 * @type {Array.<{id: string, message: string, time: string}>}
	 */
	let chats = [];
	$: { 
		console.log(chats);
		invoke("send_chat_to_screen", {payload: chats}); 
	}
	onMount(async () => {
		try {
			unsubscribeFirestore = onSnapshotChats(roomId);
			unsubscribeStore = SnapshotChats.subscribe((value) => {
				chats = value;
			});
		} catch (e) {
			alert('faild fetchMessages');
		}
	});

	onDestroy(() => {
		unsubscribeFirestore;
		unsubscribeStore;
	});
</script>

<div class="m-5 pb-10">
	{#each chats as chat}
		<ChatHistory {chat} />
	{/each}
</div>
