import { writable } from 'svelte/store';
import type { ResponseData } from './response';

export const storeData = writable<Array<ResponseData>>([]);
