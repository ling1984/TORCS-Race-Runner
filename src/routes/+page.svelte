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

<style>
.home-container {
  height: 80vh;
  display: flex;
  justify-content: center;  /* center horizontally */
  align-items: center;      /* center vertically */
}

/* Force exactly 3 columns with fixed size */
.home-slider-grid {
  display: grid;
  grid-template-columns: repeat(3, 198px);
  gap: 48px;
  justify-content: center;
  padding-left: 2rem;
}

/* Make buttons look like the cards from your image */
.home-slider-card {
  width: 198px;
  height: 162px;
  border-radius: 20px;
  background: var(--slider-card-background, #f2f2f2);

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  gap: 12px;
  border: none;
  cursor: pointer;

  box-shadow: 0 2px 6px rgba(0,0,0,0.15);
  transition: transform 0.1s ease;
}

.home-slider-card:hover {
  transform: scale(1.03);
}

.home-icon {
  width: 96px;
  height: 72px;
}

.home-label {
  font-size: 22px;
  color: #000;
}
</style>

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