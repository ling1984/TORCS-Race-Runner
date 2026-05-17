<script lang="ts">
	import { race_teams } from '$lib/stores';
    import { listen } from '@tauri-apps/api/event'
    import { get } from 'svelte/store';

    interface DriverStatus {
        team_name: string
        state: string
        port: string
    }

    let drivers: DriverStatus[] = []

    const teams = race_teams.map(team => get(team));
    teams.forEach(team => {
        if (!(team.script_path === "")) {
            drivers.push({
                team_name: team.name,
                state: "waiting",
                port: "",
            });
        }
    })
    
    listen<DriverStatus>('driver-status', (event) => {
        const payload = event.payload

        drivers.forEach((driver) => {
            if (driver.team_name === payload.team_name) {
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
        `scr_driver`

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