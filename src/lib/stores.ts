import { writable } from 'svelte/store';

// practice
export const running = writable(false);
export const team_logo_path = writable("");
export const team_name = writable("");

// all layout/ page button interactions
export const injectableMethod = writable<(event: Event) => Promise<void>>(() => {}); // for calling +page functions from +layout

// banner
export const banner_path = writable("");
export const is_banner_saved = writable(true);