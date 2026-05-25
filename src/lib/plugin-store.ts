import { load } from '@tauri-apps/plugin-store';

export const store = await load('settings.json', { autoSave: true, defaults: {}});