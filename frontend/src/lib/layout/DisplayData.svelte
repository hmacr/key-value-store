<script lang="ts">
	import Box from '$lib/components/Box.svelte';
	import Card from '$lib/components/Card.svelte';
	import type { ResponseData } from '$lib/response';
	import { onMount } from 'svelte';

	let respData: Array<ResponseData> = [];

	onMount(() => {
		fetch('http://localhost:3000/api/postgres')
			.then((res) => res.json())
			.then((data: ResponseData[]) => {
				respData = data;
			})
			.catch((err) => {
				console.log(err);
				respData = [];
			});
	});
</script>

<div class="container mx-auto p-2 my-6">
	<h2 class="text-2xl text-blue-600 text-center">Stored Data</h2>
	<Box>
		{#each respData as data}
			<Card key={data.key} value={data.value} />
		{/each}
	</Box>
</div>
