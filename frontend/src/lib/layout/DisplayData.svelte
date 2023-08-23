<script lang="ts">
	import { onMount } from 'svelte';
	import Box from '$lib/components/Box.svelte';
	import Card from '$lib/components/Card.svelte';
	import { fetchAllData } from '$lib/client';
	import { storeData } from '$lib/store';

	onMount(() => {
		fetchAllData().then((data) => ($storeData = data));
	});
</script>

<div class="container mx-auto p-2 my-6">
	<h2 class="text-2xl text-blue-600 text-center">Stored Data</h2>
	<Box>
		{#each $storeData as data}
			<Card key={data.key} value={data.value} />
		{/each}
	</Box>
</div>
