<script lang="ts">
	import { addData, fetchAllData } from '$lib/client';
	import { storeData } from '$lib/store';

	function handleSubmit(event: SubmitEvent) {
		const formData = new FormData(event.target as HTMLFormElement);

		const key = formData.get('key') as FormDataEntryValue as string;
		const value = formData.get('value') as FormDataEntryValue as string;

		addData(key, value)
			.then((_) => fetchAllData())
			.then((data) => ($storeData = data));
	}
</script>

<div class="container w-[50%] mx-auto p-4 text-center border rounded-lg border-blue-500">
	<h2 class="text-2xl text-blue-600">Add Data</h2>
	<form on:submit|preventDefault={handleSubmit}>
		<label>
			<span class="inline-block text-pink-500 p-4">Key:</span>
			<input type="text" name="key" class="rounded-md p-1" />
		</label>
		<label>
			<span class="inline-block text-pink-500 p-4 ml-6">Value:</span>
			<input type="text" name="value" class="rounded-md p-1" />
		</label>
		<button
			class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 ml-6 rounded focus:outline-none focus:shadow-outline"
			type="submit"
		>
			Add
		</button>
	</form>
</div>
