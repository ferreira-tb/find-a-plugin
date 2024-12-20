import { writable } from 'svelte/store';

export { search } from './search';
export { pluginData } from './plugin';
export { bookmark } from './bookmark';

export const gridMode = writable<GridMode>('all');
