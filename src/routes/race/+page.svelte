<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { open, message } from "@tauri-apps/plugin-dialog";
  import { race_teams, race_running, injectableMethod, curr_team_index } from '$lib/stores';
  

  let msg = $state("");
  let preview_url = $state("");

  let current_team = $derived(race_teams[$curr_team_index]);
  let tab_team_names = ["Team 0", "Team 1", "Team 2", "Team 3", "Team 4", "Team 5"];

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
        preview_url = convertFileSrc($current_team.logo_path);
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
      race_running.set(true);
      //   showAlert("Driver started successfully! Now in the TORCS window navigate: Race -> Practice -> New Race, ", "Success", "info");
      // } catch (error) {
      //   msg = `Error: ${error}`;
      //   showAlert(`Failed to start driver: ${error}`, "Error", "error");
      // }
  }
  function resetCurrentTeam() {
    current_team.update((team) => ({ ...team, name: "", logo_path: "", driver_script_path: "" }));
  }
</script>

<main class="container">
  <p>{msg}</p>
  <div class="layout">
    <!-- Left tabs -->
    <div class="tabs">
        {#each tab_team_names as tab_team_name, index}
            <button class="tab-button"
                class:selected={$curr_team_index === index}
                onclick={() => curr_team_index.set(index)}
            >
                {tab_team_name}
            </button>
        {/each}
    </div>
  <!-- The title and two cards -->

  <div class="content">
      <h1>{$current_team.name ? $current_team.name : "Team Name"}</h1>
      

    <div class="slider-grid">
      <!-- // DRIVER SCRIPT -->
      <div class="slider-card">
        <header>
          <div class="label-container">
            <span class="label">Driver script</span>
            <span class="help-icon" data-tooltip="Upload your driver script (.py file only).">?</span>
          </div>
        </header>
        <button onclick={pick_driver_script_file}>Find file</button>
        <button onclick={() => current_team.update((team) => ({ ...team, driver_script_path: "" }))}>Reset</button>
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
        <button onclick={() => {current_team.update((team) => ({ ...team, logo_path: "" })); preview_url = "";}}>Reset</button>
      </div>
    </div>

  </div>
  </div>
</main>


<style>
  .layout {
    display: flex;
  }

  .tabs {
    display: flex;
    flex-direction: column;
    /* margin-top: 40px; */
  }
  

  .tab-button {
    width: 80px;

    background: #222;
    color: white;

    border: 1px solid #666;
    border-right: 1px solid #666;

    border-radius: 0;
    box-shadow: none;

    padding: 12px;

    transition:
    background 0.15s,
    border-color 0.15s;
  }

  .tab-button:first-child {
    border-top-left-radius: 8px;
  }

  .tab-button:last-child {
    border-bottom-left-radius: 8px;
  }

  .tab-button:hover {
    background: #333;
    border-color: #888;
  }

  .tab-button.selected {
    background: #111;

    border-right: none;

    margin-right: -1px;

    position: relative;
    z-index: 2;
  }

  .content {
    flex: 1;
    border: 1px solid #666;
    padding: 24px;
  }
</style>