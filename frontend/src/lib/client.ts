import type { ResponseData } from './response';

export async function fetchAllData(): Promise<Array<ResponseData>> {
	return fetch('http://localhost:3000/api/postgres')
		.then((res) => res.json())
		.then((data: Array<ResponseData>) => {
			return data;
		})
		.catch((err) => {
			console.log(err);
			return Array<ResponseData>();
		});
}

export async function addData(key: string, value: string): Promise<boolean> {
	return fetch('http://localhost:3000/api/postgres', {
		method: 'POST',
		body: JSON.stringify({
			key,
			value
		}),
		headers: {
			'Content-Type': 'application/json'
		}
	})
		.then((res) => {
			console.log('adding data success;', res);
			return true;
		})
		.catch((err) => {
			console.log('adding data failed;', err);
			return false;
		});
}
