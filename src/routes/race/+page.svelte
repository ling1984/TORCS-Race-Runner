<script lang="ts">
  import RaceMenu from './RaceMenu.svelte';
  import DriverStatus from './DriverStatus.svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { message } from "@tauri-apps/plugin-dialog";
  import { race_teams, race_running, injectableMethod } from '$lib/stores';
  import { get } from "svelte/store";
  

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
        const teams = race_teams.map(team => get(team));
        console.log(JSON.stringify(teams, null, 2));
        msg = await invoke("start_race", {raceTeams: teams});
        race_running.set(true);
        showAlert("Race started successfully! Now in the TORCS window navigate: Race -> Race -> New Race, ", "Success", "info");
      } catch (error) {
        msg = `Error: ${error}`;
        showAlert(`Failed to start race: ${error}`, "Error", "error");
      }
    } else {
      try {
        msg = await invoke("stop_race");
      } catch (error) {
        msg = `Error: ${error}`;
        showAlert(`Failed to stop race: ${error}`, "Error", "error");
      }
    }
  }
  
</script>

<main class="container">
  <p>{msg}</p>
  {#if !$race_running}
      <RaceMenu 
        msg={msg}/>
  {:else}
    <DriverStatus/>
  {/if}
</main>
