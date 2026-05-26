<script lang="ts">
  import RaceMenu from './RaceMenu.svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { message } from "@tauri-apps/plugin-dialog";
  import { race_teams, race_running, injectableMethod } from '$lib/stores';
  import { get } from "svelte/store";
  import { listen } from '@tauri-apps/api/event';
  import DriverStatusPage from './DriverStatusPage.svelte';
  import type { DriverStatus } from '$lib/types';

  let drivers = $state<DriverStatus[]>([]);
  // we need to set the listener immediately
  listen<DriverStatus>('driver-status', (event) => {
    const payload = event.payload;

    const driver = drivers.find(d => d.index === payload.index);

    if (driver) {
      driver.team_name = payload.team_name;
      driver.state = payload.state;
      driver.port = payload.port.replace(/\.+$/, ""); // Remove trailing '....'
      drivers = drivers;
    }
  });

  let msg = $state("");

  $effect(() => {
    injectableMethod.set(handle_start_race);
  });

  async function showAlert(contents: string, title: string = "Info", kind: "info" | "error" = "info") {
    await message(contents, { 
      title: title, 
      kind: kind 
    });
  }
  
  async function handle_start_race(event: Event) {
    event.preventDefault();
    if (!$race_running) {
      try {
        setUpDrivers();
        const teams = race_teams.map(team => get(team));
        console.log(JSON.stringify(teams, null, 2));
        msg = await invoke("start_race", {raceTeams: teams});
        race_running.set(true);
        showAlert("Race started successfully! In the TORCS window: Race -> Race -> New Race", "Success", "info");
      } catch (error) {
        msg = `Error: ${error}`;
        showAlert(`Failed to start race: ${error}`, "Error", "error");
      }
    } else {
      try {
        msg = await invoke("stop_race");
        msg = "Stopped";
        race_running.set(false);
      } catch (error) {
        msg = `Error: ${error}`;
        showAlert(`Failed to stop race: ${error}`, "Error", "error");
      }
    }
  }

  function setUpDrivers() {
    drivers = [];
    let index = 0;
    race_teams.forEach((teamStore) => {
      const team = get(teamStore);
      if (team.script_path !== '') {
        drivers.push({
          index: index,
          team_name: team.name,
          state: 'waiting',
          port: ''
        });
      }
      index++;
    });
    console.log("Drivers set up:", drivers);
  };
</script>

<main class="container">
  <!-- <p>{msg}</p> -->
  {#if !$race_running}
    <RaceMenu 
      msg={msg}/>
  {:else}
    <DriverStatusPage 
      drivers={drivers}/>
  {/if}
</main>
