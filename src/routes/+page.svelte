<script>
  import { goto } from '$app/navigation';

  function startPractice() {
    console.log("Start Practice clicked");
    goto('/single');
  }

  function editTrackBanner() {
    console.log("Edit Track Banner clicked");
  }

</script>

<style>

  .card-button {
    width: 220px;
    height: 180px;
    border-radius: 20px;
    background: #f2f2f2;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    box-shadow: 0 2px 6px rgba(0,0,0,0.1);
    cursor: pointer;
    transition: transform 0.1s ease;
  }
</style>

<h1>Welcome</h1>
<!-- <button on:click={startPractice}>Start Practice</button>
<button on:click={editTrackBanner}>Edit Track Banner</button> -->

<div class="container">
  <div class="button-row">
    
    <!-- Start Race -->
    <div class="btn" on:click={startPractice}>
      <svg class="icon" viewBox="0 0 24 24">
        <polygon points="6,4 20,12 6,20" fill="none"></polygon>
      </svg>
      <div class="label">Start Race</div>
    </div>

    <!-- Edit Track -->
    <div class="card-button" on:click={editTrackBanner}>
      <svg class="icon" viewBox="0 0 24 24">
        <rect x="3" y="14" width="14" height="7"></rect>
        <line x1="14" y1="3" x2="21" y2="10"></line>
        <line x1="13" y1="4" x2="20" y2="11"></line>
      </svg>
      <div class="label">Edit track</div>
    </div>

  </div>
</div>