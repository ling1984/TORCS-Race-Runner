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
    <button class="btn-drive {$running ? 'running' : 'stopped'}" type="submit">
      {$running ? 'Stop driver' : 'Start driver'}
    </button>
  </form>
</header>

<slot></slot>