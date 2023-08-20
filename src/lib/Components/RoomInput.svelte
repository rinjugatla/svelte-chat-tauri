<script>
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
	let room = '';
	let posting = false;
	const submit = async () => {
		posting = true;
		const roomId = await postRoom(room);
		if (roomId) {
			document.location.href = `/rooms/${roomId}`;
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
				<Input id="room" type="text" bind:value={room} on:keydown={submitByKey}/>
				<Button class="break-keep" color="primary" disabled={posting || room.length == 0} on:click={submit}>入室</Button>
			</ButtonGroup>
        </form>
    {/if}
</div>