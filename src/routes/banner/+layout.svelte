<script lang="ts">
  import { injectableMethod } from '$lib/stores';
  import { goto } from '$app/navigation';
  import { ArrowLeft, Save } from '@lucide/svelte';

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
    <button class="btn" type="button" onclick={handleBackClick}>
      <ArrowLeft size={24}/>
    </button>
  <h1>Edit Banner</h1>
    <button class="btn-drive" type="button" onclick={handleButtonClick}>
      <Save size={24} />
    </button>
</header>

<slot></slot>