<script>
  import { goto } from '$app/navigation';

  function openApp() {
    goto('/single'); // navigate to your old content
  }
</script>

<h1>Welcome</h1>
<button on:click={openApp}>Open App</button>