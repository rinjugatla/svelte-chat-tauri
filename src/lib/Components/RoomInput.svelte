<script lang="ts" type="module">
    import { invoke } from '@tauri-apps/api';
	import { UserId } from '$lib/store';
	import { onDestroy } from 'svelte';
    import { ButtonGroup, InputAddon, Input, Button } from 'flowbite-svelte';
	import { postRoom } from '$lib/api';

	let uid: string;
	const unsubscribe = UserId.subscribe((id: string) => {
		uid = id;
	});
	onDestroy(() => {
		unsubscribe();
	});

    // 入室
	let meet_id = '';
	let posting = false;
	const enterRoom = async () => {
		try{
			posting = true;
			const roomIdPattern = "[a-z]{3}-[a-z]{4}-[a-z]{3}";
			const matchResult = meet_id.match(roomIdPattern);
			if (!matchResult){ 
				alert('部屋IDの形式に誤りがあります。');
				return; 
			}

			const validRommId = matchResult[0];
			const room_firebase_id = await postRoom(validRommId);
			if (room_firebase_id) {
				invoke("create_child_window", {meet: {id: validRommId}});
				document.location.href = `/room?roomId=${room_firebase_id}`;
			} else {
				alert('ルームの作成に失敗しました。');
			}
		}finally{
			posting = false;
		}
	};

	const submitByKey = async (event: KeyboardEvent) => {
		const needSubmit = event.key == 'Enter';
		if(!needSubmit){ return; }
		
		await enterRoom();
	};
</script>

<div>
    {#if uid}
        <form class="p-5">
			<ButtonGroup class="w-full">
				<InputAddon class="break-keep">部屋名</InputAddon>
				<Input id="room" type="text" bind:value={meet_id} on:keydown={submitByKey} placeholder="abc-defg-hij" required/>
				<Button class="break-keep" color="primary" disabled={posting || meet_id.length == 0} on:click={enterRoom}>入室</Button>
			</ButtonGroup>
        </form>
    {/if}
</div>