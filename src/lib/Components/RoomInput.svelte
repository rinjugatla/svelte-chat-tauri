<script>
    import { invoke } from '@tauri-apps/api';
	import { UserId } from '$lib/store';
	import { onDestroy } from 'svelte';
    import { ButtonGroup, InputAddon, Input, Button } from 'flowbite-svelte';
	import { postRoom } from '$lib/api';

    /**
	 * @type {string}
	 */
	let uid;
	const unsubscribe = UserId.subscribe((id) => {
		uid = id;
	});
	onDestroy(() => {
		unsubscribe();
	});

    // 入室
	let meet_id = '';
	let posting = false;
	const submit = async () => {
		posting = true;
		const room_firebase_id = await postRoom(meet_id);
		if (room_firebase_id) {
			invoke("create_meet_window", {meet: {id: meet_id}});
			document.location.href = `/rooms/${room_firebase_id}`;
		} else {
			alert('ルームの作成に失敗しました。');
		}

		posting = false;
	};

	const submitByKey = async (/** @type {KeyboardEvent} */ event) => {
		const needSubmit = event.key == 'Enter';
		if(!needSubmit){ return; }
		
		await submit();
	};
</script>

<div>
    {#if uid}
        <form class="p-5">
			<ButtonGroup class="w-full">
				<InputAddon class="break-keep">部屋名</InputAddon>
				<Input id="room" type="text" bind:value={meet_id} on:keydown={submitByKey}/>
				<Button class="break-keep" color="primary" disabled={posting || meet_id.length == 0} on:click={submit}>入室</Button>
			</ButtonGroup>
        </form>
    {/if}
</div>