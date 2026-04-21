<script>
  import { goto } from '$app/navigation';

  function startPractice() {
    console.log("Start Practice clicked");
    goto('/single');
  }

  function startRace() {
    console.log("Start Race clicked");
    goto('/multi');
  }

  function editTrackBanner() {
    console.log("Edit Track Banner clicked");
    goto('/banner');
  }

</script>

<style>
.home-container {
  height: 100vh;
  display: flex;
  justify-content: center;  /* center horizontally */
  align-items: center;      /* center vertically */
}

/* Force exactly 3 columns with fixed size */
.home-slider-grid {
  display: grid;
  grid-template-columns: repeat(3, 220px);
  gap: 60px;
  justify-content: center;
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
  width: 48px;
  height: 48px;
}

.home-label {
  font-size: 22px;
  color: #000;
}

/* clean icon style */
svg {
  stroke: #222;
  fill: none;
  stroke-width: 2;
}
</style>
<div class="home-container">
  <div class="home-slider-grid">
    
    <!-- Start Practice -->
    <button class="home-slider-card" type="button" onclick={startPractice}>
      <svg class="home-icon" viewBox="0 0 24 24">
        <polygon points="6,4 20,12 6,20" fill="none"></polygon>
      </svg>
      <div class="home-label">Start Practice</div>
    </button>
    
    <!-- Start Race -->
    <button class="home-slider-card" type="button" onclick={startRace}>
      <svg class="home-icon" viewBox="0 0 24 24">
        <polygon points="6,4 20,12 6,20" fill="none"></polygon>
      </svg>
      <div class="home-label">Start Race</div>
    </button>

    <!-- Edit Track Banner -->
    <button class="home-slider-card" type="button" onclick={editTrackBanner}>
      <svg class="home-icon" viewBox="0 0 24 24">
        <rect x="3" y="14" width="14" height="7"></rect>
        <line x1="14" y1="3" x2="21" y2="10"></line>
        <line x1="13" y1="4" x2="20" y2="11"></line>
      </svg>
      <div class="home-label">Edit Track Banner</div>
    </button>

  </div>
</div>