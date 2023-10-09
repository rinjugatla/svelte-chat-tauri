<script lang="ts">
    import { onMount } from 'svelte';
	import { existRoomById } from '$lib/api';

    import ChatHistories from '$lib/Components/ChatHistories.svelte';
	import ChatInput from '$lib/Components/ChatInput.svelte';

    const urlParams = new URLSearchParams(window.location.search);
    // 部屋名
    const roomId = urlParams.has("roomId") ? urlParams.get("roomId")! : "";
    let isValidRoom = false;

    onMount(async () => {
        if(roomId == null) { document.location.href = '/chat'; }

        const exists = await existRoomById(roomId);
        isValidRoom = exists;
        if (exists){ return; }

        alert('部屋が存在しません。部屋を登録してください。');
        document.location.href = `/chat`;
	});
</script>

{#if isValidRoom}
<ChatHistories {roomId} />
<ChatInput {roomId} />
{/if}