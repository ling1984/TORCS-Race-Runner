<script lang="ts">
  import { running, onStartDriver } from '$lib/stores';
  import { goto } from '$app/navigation';

  async function handleButtonClick(event: Event) {
    const handler = $onStartDriver;
    await handler(event);
  }

  function handleBackClick() {
    console.log("Back clicked");
    goto('/');
  }
</script>

<header class="header">
    <button class="btn" type="button" onclick={handleBackClick}>
      Back
    </button>
  <h1>Practice</h1>
    <button class="btn-drive {$running ? 'running' : 'stopped'}" type="button" onclick={handleButtonClick}>
      {$running ? 'Stop' : 'Start'}
    </button>
</header>

<slot></slot>