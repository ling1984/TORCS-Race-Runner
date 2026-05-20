<script>
  import { goto } from '$app/navigation';
  import { Timer, Flag, Palette, Settings } from '@lucide/svelte';

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

  let settingsOpen = $state(false)
</script>

<header class="header" style="height: 72px;">
  <h1>TORCS Race Runner</h1>

  <button onclick={() => settingsOpen = true}>
    <Settings size={20} />
  </button>
</header>

<div class="home-container">
  <div class="home-slider-grid">
    
    <!-- Start Practice -->
    <button class="home-slider-card" type="button" onclick={startPractice}>
      <Timer size={72} strokeWidth={1.5}/>
      <div class="home-label">Practice</div>
    </button>
    
    <!-- Start Race -->
    <button class="home-slider-card" type="button" onclick={startRace}>
      <Flag size={72} strokeWidth={1.5} />
      <div class="home-label">Race</div>
    </button>

    <!-- Edit Banner -->
    <button class="home-slider-card" type="button" onclick={editBanner}>
      <Palette size={72} strokeWidth={1.5} />
      <div class="home-label">Edit Banner</div>
    </button>
  </div>
</div>