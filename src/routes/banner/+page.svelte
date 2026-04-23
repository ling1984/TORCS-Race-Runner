<script>
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let logo_path = $state("");
  let preview_url = $state("");
  let msg = $state("");
  
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
        logo_path = selected;
        await invoke("set_logo_path", { path: logo_path });
        preview_url = convertFileSrc(logo_path);
      }
    } catch (error) {
      msg = `Error selecting logo: ${error}`;
    }
  }
</script>

<div class="slider-grid">
  <div class="slider-card">

      {#if preview_url}
        <div class="preview-container">
          <img src={preview_url} alt="Team logo preview" class="logo-preview" />
        </div>
      {/if}
      <button onclick={pick_logo_file}>Find your logo file</button>
      <button onclick={() => {logo_path = ""; preview_url = ""; invoke("set_logo_path", { path: logo_path });}}>Reset</button>
    </div>
</div>