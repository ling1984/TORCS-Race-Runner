<script>
  import { goto } from '$app/navigation';
  import { Timer, Flag, Palette, Settings } from '@lucide/svelte';
  import SettingsModal from './SettingsModal.svelte';
  import { store } from '../lib/plugin-store';
  import { onMount } from 'svelte';

  function startPractice() {
    console.log("Start Practice clicked");
    goto('/practice');
  }

  function startRace() {
    console.log("Start Race clicked");
    goto('/race');
  }

  function editBanner() {
    console.log("Edit Banner clicked");
    goto('/banner');
  }

  let settingsOpen = $state(false);

  onMount(async () => {
    // read existing value
    let folder_path = (await store.get('folder_path')) ?? '';

    console.log(folder_path);
    if (folder_path === '') {
      settingsOpen = true;
    }
  });
</script>

<header class="header" style="height: 72px;">
  <h1>TORCS Race Runner</h1>

  <button class="btn" type="button" style="margin-left: auto;" onclick={() => settingsOpen = true}>
    <Settings size={24} />
  </button>
</header>

<div class="home-container">
  <div class="home-slider-grid">
    
    <!-- Start Practice -->
    <button class="home-slider-card frontpage-button" type="button" onclick={startPractice}>
      <Timer size={72} strokeWidth={1.5}/>
      <div class="home-label">Practice</div>
    </button>
    
    <!-- Start Race -->
    <button class="home-slider-card frontpage-button" type="button" onclick={startRace}>
      <Flag size={72} strokeWidth={1.5} />
      <div class="home-label">Race</div>
    </button>

    <!-- Edit Banner -->
    <button class="home-slider-card frontpage-button" type="button" onclick={editBanner}>
      <Palette size={72} strokeWidth={1.5} />
      <div class="home-label">Edit Banner</div>
    </button>
  </div>
</div>

<div>
  <SettingsModal 
    settingsOpen={settingsOpen}
    closeSettings={() => settingsOpen = false}/>
</div>

<style>
  .frontpage-button {
    background: var(--button-background);
  }
</style>