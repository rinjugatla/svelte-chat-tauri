<script lang="ts">
    import type {PageData} from './$types';

    import { onMount } from 'svelte';
	import { existRoomById } from '$lib/api';

    import ChatHistories from '$lib/Components/ChatHistories.svelte';
	import ChatInput from '$lib/Components/ChatInput.svelte';

    export let data: PageData;
    // 部屋名
    const roomId = data.roomId;
    let isValidRoom = false;

    onMount(async () => {
        const exists = await existRoomById(roomId);
        isValidRoom = exists;
        if (exists){ return; }

        alert('部屋が存在しません。部屋を登録してください。');
        document.location.href = `/`;
	});
</script>

{#if isValidRoom}
    <ChatHistories {roomId} />
    <ChatInput {roomId} />
{/if}
