<script lang="ts">
  import { practice_running, injectableMethod } from '$lib/stores';
  import { goto } from '$app/navigation';
  import { ArrowLeft, Play, Pause } from '@lucide/svelte';

  async function handleButtonClick(event: Event) {
    const handler = $injectableMethod;
    await handler(event);
  }

  function handleBackClick() {
    console.log("Back clicked");
    goto('/');
  }
</script>

<header class="header" style="height: 72px;">
    <button class="btn" type="button" onclick={handleBackClick} disabled={$practice_running}>
      <ArrowLeft size={24}/>
    </button>
  <h1>Practice</h1>
    <button class="btn-drive {$practice_running ? 'running' : 'stopped'}" type="button" onclick={handleButtonClick}>
      {#if $practice_running}
        <Pause size={24} />
      {:else}
        <Play size={24} />
      {/if}
    </button>
</header>

<slot></slot>