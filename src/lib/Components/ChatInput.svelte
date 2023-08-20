<script>
    import { UserId } from '$lib/store';
	import { onDestroy } from 'svelte';
    import { postMessage } from '$lib/api';
    import { ButtonGroup, InputAddon, Input, Button } from 'flowbite-svelte';

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

	export let roomId = '';

    // dbに送信するメッセージ
	let message = '';
	let posting = false;
	const submit = async () => {
		posting = true;
		const result = await postMessage(roomId, message);
		if (result) {
			// alert(result);
		} else {
			alert('チャットの送信に失敗しました。');
		}

		message = '';
		posting = false;
	};

	const submitByKey = async (/** @type {KeyboardEvent} */ event) => {
		const needSubmit = event.ctrlKey && event.key == 'Enter';
		if(!needSubmit){ return; }
		
		await submit();
	};
</script>

<div>
    {#if uid}
		<form class="m-5 fixed inset-x-0 bottom-0">
			<ButtonGroup class="w-full">
				<InputAddon class="break-keep">メッセージ</InputAddon>
				<Input id="message" type="text" bind:value={message} on:keydown={submitByKey}/>
				<Button class="break-keep" color="primary" disabled={posting || message.length == 0} on:click={submit}>送信</Button>
			</ButtonGroup>
		</form>
    {/if}
</div>