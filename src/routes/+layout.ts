// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export interface SliderConfig {
  id: string;
  label: string;
  min: number;
  max: number;
  default: number;
  value: number;
}