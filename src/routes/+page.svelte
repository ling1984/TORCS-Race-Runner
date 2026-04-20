<script>
  import { goto } from '$app/navigation';

  function openApp() {
    goto('/single');
  }
</script>

<h1>Welcome</h1>
<button on:click={openApp}>Open App</button>