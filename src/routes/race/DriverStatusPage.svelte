<script lang="ts">
    let { drivers } = $props();

</script>

<div class="driver-status">
    {#each drivers as driver, i}
      <div>
        { driver.team_name === "" ? `scr_driver ${i}` : driver.team_name } :

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
    