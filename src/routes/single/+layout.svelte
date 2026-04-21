<script lang="ts">
  import { running, onStartDriver } from '$lib/stores';
  import { goto } from '$app/navigation';
  import { ArrowLeft, Play, Pause } from '@lucide/svelte';

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
      <ArrowLeft size={24}/>
    </button>
  <h1>Practice</h1>
    <button class="btn-drive {$running ? 'running' : 'stopped'}" type="button" onclick={handleButtonClick}>
      {#if $running}
        <Pause size={24} />
      {:else}
        <Play size={24} />
      {/if}
    </button>
</header>

<slot></slot>