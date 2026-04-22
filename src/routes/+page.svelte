<script>
  import { goto } from '$app/navigation';
  import { Timer, Flag, Palette } from '@lucide/svelte';

  function startPractice() {
    console.log("Start Practice clicked");
    goto('/practice');
  }

  function startRace() {
    console.log("Start Race clicked");
    goto('/race');
  }

  function editTrackBanner() {
    console.log("Edit Track Banner clicked");
    goto('/banner');
  }

</script>

<header class="header">
  <h1>TORCS Race Runner</h1>
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

    <!-- Edit Track Banner -->
    <button class="home-slider-card" type="button" onclick={editTrackBanner}>
      <Palette size={72} strokeWidth={1.5} />
      <div class="home-label">Edit Track Banner</div>
    </button>

  </div>
</div>