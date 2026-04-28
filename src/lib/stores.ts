import { writable } from 'svelte/store';

// practice
export const running = writable(false);

// all layout/ page button interactions
export const injectableMethod = writable<(event: Event) => Promise<void>>(() => {}); // for calling +page functions from +layout

// banner
export const banner_preview_url = writable("");
export const is_banner_saved = writable(true);