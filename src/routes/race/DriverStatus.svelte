<script lang="ts">
	// Define the TypeScript interface for your props
	// interface Props {
	// 	user: { name: string; email: string };
	// 	status: string;
	// 	onUpdateStatus: (newStatus: string) => void; // Method passed from parent
	// }

	// // Destructure the props with the $props() rune
	// let { user, status, onUpdateStatus }: Props = $props();
</script>

<div class="driver-status">
    {#each drivers as driver, i}
      <div>
        Driver {i} :

        {#if driver.state === 'connected'}
          <span style="color: green">
            connected on port {driver.port}
          </span>

        {:else if driver.state === 'connecting'}
          <span style="color: orange">
            connecting on port {driver.port}
          </span>

        {:else}
          <span>
            waiting to connect...
          </span>
        {/if}
      </div>
    {/each}
</div>