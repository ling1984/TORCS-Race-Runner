import { writable } from 'svelte/store';

export const running = writable(false);
export const onStartDriver = writable<(event: Event) => Promise<void>>(() => {});
