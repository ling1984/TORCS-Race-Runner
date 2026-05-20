<script>
  import { goto } from '$app/navigation';
  import { Timer, Flag, Palette, Settings, X } from '@lucide/svelte';

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

  function closeSettings() {
    settingsOpen = false;
  }

  let settingsOpen = $state(false)
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
  {#if settingsOpen}
  <div class="overlay">
    <div class="modal">
      <div class="modal-header">
        <h2>Settings</h2>
        <button class="btn" type="button" onclick={closeSettings}>
          <X size={24} />
        </button>
      </div>
      <div class="modal-content">
        <p>Settings options coming soon...</p>
      </div>
    </div>
  </div>
  {/if}
</div>

<style>
  .frontpage-button {
    background: var(--button-background);
  }
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background-color: var(--color-bg-0, white);
    border-radius: 8px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
    width: 90%;
    max-width: 500px;
    max-height: 80vh;
    overflow-y: auto;
    position: relative;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px;
    border-bottom: 1px solid var(--color-border, #e0e0e0);
  }

  .modal-content {
    padding: 20px;
  }

</style>