<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { open, message } from "@tauri-apps/plugin-dialog";
  import type { RaceTeam } from '$lib/types';
  import { race_teams, race_running, injectableMethod } from '$lib/stores';
  

  let msg = $state("");
  let curr_team_index = $state(0);
  let preview_url = $state("");

  let current_team = $derived(race_teams[curr_team_index]);

  if ($current_team.logo_path) {
    preview_url = convertFileSrc($current_team.logo_path);
  }

  $effect(() => {
    injectableMethod.set(handle_start_race);
  });

  async function pick_logo_file() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp']
        }]
      });

      if (selected) {
        current_team.update((team) => ({ ...team, logo_path: selected }));
      }
    } catch (error) {
      msg = `Error selecting logo: ${error}`;
    }
  }

  async function pick_driver_script_file() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Python Script',
          extensions: ['py']
        }]
      });

      if (selected) {
        current_team.update((team) => ({ ...team, driver_script_path: selected }));
      }
    } catch (error) {
      msg = `Error selecting driver script: ${error}`;
    }
  }

  async function showAlert(contents: string, title: string = "Info", kind: "info" | "error" = "info") {
    await message(contents, { 
      title: title, 
      kind: kind 
    });
  }

  async function handle_start_race(event: Event) {
    event.preventDefault();
      // try {
      //   const params = collectParams();
      //   await invoke("handle_params", { params });
      //   msg = await invoke("start_racer");
      //   running.set(true);
      //   showAlert("Driver started successfully! Now in the TORCS window navigate: Race -> Practice -> New Race, ", "Success", "info");
      // } catch (error) {
      //   msg = `Error: ${error}`;
      //   showAlert(`Failed to start driver: ${error}`, "Error", "error");
      // }
  }
  function resetTeam(index: number) {
    current_team.update((team) => ({ ...team, name: "", logo_path: "", driver_script_path: "" }));
  }
</script>

<main class="container">
  <p>{msg}</p>
  <div class="slider-grid">
   <!-- // TEAM NAME -->
    <div class="slider-card">
      <header>
        <div class="label-container">
          <span class="label">Team name</span>
          <span class="help-icon" data-tooltip="Choose your team name.">?</span>
        </div>
      </header>
      <input id="target-input" autocomplete="off" placeholder="Enter your team name..." bind:value={$current_team.name} />
      <button onclick={() => current_team.update((team) => ({ ...team, name: "" }))}>Reset</button>
    </div>
    <!-- // TEAM LOGO -->
    <div class="slider-card">
      <header>
        <div class="label-container">
          <span class="label">Team logo</span>
          <span class="help-icon" data-tooltip="Upload your team logo. 1.85:1 aspect ratio and 61x33 pixels is recommended.">?</span>
        </div>
      </header>
    {#if preview_url}
      <div class="preview-container">
        <img src={preview_url} alt="Team logo preview" class="logo-preview" />
      </div>
    {/if}
      <button onclick={pick_logo_file}>Find file</button>
      <button onclick={() => {current_team.update((team) => ({ ...team, logo_path: "" })); preview_url = ""; invoke("set_logo_path", { path: $current_team.logo_path });}}>Reset</button>
    </div>

    <!-- // DRIVER SCRIPT -->
    <div class="slider-card">
      <header>
        <div class="label-container">
          <span class="label">Driver script</span>
          <span class="help-icon" data-tooltip="Upload your driver script.">?</span>
        </div>
      </header>
      <button onclick={pick_driver_script_file}>Find file</button>
      <button onclick={() => current_team.update((team) => ({ ...team, driver_script_path: "" }))}>Reset</button>
    </div>
  </div>
</main>