import { writable } from 'svelte/store';

export const running = writable(false);
export const injectableMethod = writable<(event: Event) => Promise<void>>(() => {}); // for calling +page functions from +layout

export const banner_preview_url = writable("");