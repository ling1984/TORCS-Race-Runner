import { writable } from 'svelte/store';
import type { RaceTeam } from './types';

// practice
export const running = writable(false);
export const team_logo_path = writable("");
export const team_name = writable("");

// running
export const race_running = writable(false);

export const race_teams = Array.from({ length: 6 }, () =>
	writable<RaceTeam>({
		name: '',
		logo_path: '',
		script_path: ''
	})
);

// all layout + page button interactions
export const injectableMethod = writable<(event: Event) => Promise<void>>(async (event: Event) => {}); // for calling +page functions from +layout

// banner
export const banner_path = writable("");
export const is_banner_saved = writable(true);
