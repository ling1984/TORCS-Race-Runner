<script lang="ts">
	import { race_teams } from '$lib/stores';
    import { listen } from '@tauri-apps/api/event'
    import { get } from 'svelte/store';
    import type { DriverStatus } from '$lib/types';

    let drivers = $state<DriverStatus[]>([]);

    race_teams.forEach((teamStore) => {
        const team = get(teamStore); // Extracts the actual RaceTeam object
        
        if (team.script_path !== "") {
            drivers.push({
                index: drivers.length,
                team_name: team.name,
                state: "waiting",
                port: "",
            });
        }
    });
    
    listen<DriverStatus>('driver-status', (event) => {
        const payload = event.payload
        drivers.forEach((driver) => {
            if (driver.index === payload.index) {
                console.log(`Updating driver ${driver.index} status:`, payload);
                driver.team_name = payload.team_name
                driver.state = payload.state
                driver.port = payload.port
            }
        })
    })
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