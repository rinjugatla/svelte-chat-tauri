<script>
	import { Button } from 'flowbite-svelte';
	import RoomInput from '$lib/Components/RoomInput.svelte';

	import { emit  } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api';
</script>

<RoomInput />

<Button on:click={() => {invoke("send_chat_to_screen", {message: "test"})}}>simple_command</Button>