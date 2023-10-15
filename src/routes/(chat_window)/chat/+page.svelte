<script>
	import { invoke } from '@tauri-apps/api';

	import RoomInput from '$lib/Components/RoomInput.svelte';
	import { onMount } from 'svelte';
	
	onMount(() => {
		invoke("close_meet_window");
	});

</script>

<RoomInput />