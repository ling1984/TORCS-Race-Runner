<script lang="ts">
  import { running, onStartDriver } from '$lib/stores';

  async function handleButtonClick(event: Event) {
    const handler = $onStartDriver;
    await handler(event);
  }
</script>

<header class="header">
  <h1>TORCS Race Runner</h1>
  <form class="button-container" onsubmit={handleButtonClick}>
    <button class="btn-drive" type="submit">
      {$running ? 'Stop driver' : 'Start driver'}
    </button>
  </form>
</header>

<slot></slot>

<style>
.header {
    /* position: sticky; when it can behave itself, it can come back to being sticky.
    issues:
    1. it is see through on light mode
    2. help icons and tooltips appear above it
    3. header height at top is slightly larger than height when scrolling */
    top: 0;
    overflow: visible;
    width: 100%;
    box-sizing: border-box;

    display: flex;
    justify-content: space-between;
    align-items: center;

    padding: 1.5rem 2rem;
    background: var(--background);
    border-bottom: 1px solid var(--line);
}

  h1 {
    margin: 0;
    font-size: 2rem;
  }

  .button-container {
    display: flex;
  }

  .btn-drive {
    background: #007bff;
    color: white;
    border: none;
    padding: 0.6rem 1.5rem;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-drive:hover {
    background: #0056b3;
  }

  .btn-drive:active {
    background: #003d82;
  }

  @media (prefers-color-scheme: dark) {
    .header {
      background: var(--background, #2f2f2f);
    }
  }
</style>
