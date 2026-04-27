<script>
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let long_banner_path = $state("");
  let long_banner_preview_url = $state("");
  let square_banner_path = $state("");
  let square_banner_preview_url = $state("");
  let msg = $state("");
  
  async function pick_long_banner_file() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp']
        }]
      });

      if (selected) {
        long_banner_path = selected;
        await invoke("set_long_banner_path", { path: long_banner_path });
        long_banner_preview_url = convertFileSrc(long_banner_path);
      }
    } catch (error) {
      msg = `Error selecting long banner: ${error}`;
    }
  }

  async function pick_square_banner_file() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp']
        }]
      });

      if (selected) {
        square_banner_path = selected;
        await invoke("set_square_banner_path", { path: square_banner_path });
        square_banner_preview_url = convertFileSrc(square_banner_path);
      }
    } catch (error) {
      msg = `Error selecting square banner: ${error}`;
    }
  }
</script>

<main class="container">
  <!-- <p>{msg}</p> -->
  <div class="slider-grid">
    <div class="slider-card">
    <header>
            <div class="label-container">
              <span class="label">Square banner image</span>
              <span class="help-icon" data-tooltip="Upload a square image (1:1 aspect ratio).">?</span>
            </div>
          </header>
      {#if square_banner_preview_url}
        <div class="preview-container">
          <img src={square_banner_preview_url} alt="Team logo preview" class="logo-preview" />
        </div>
      {/if}
      <button onclick={pick_square_banner_file}>Find file</button>
      <button onclick={() => {square_banner_path = ""; square_banner_preview_url = ""; invoke("set_square_banner_path", { path: square_banner_path });}}>Reset</button>
    </div>
    <div class="slider-card">
    <header>
            <div class="label-container">
              <span class="label">Rectangular banner image</span>
              <span class="help-icon" data-tooltip="Upload a rectangular image (2:1 aspect ratio ideally).">?</span>
            </div>
          </header>
      {#if long_banner_preview_url}
        <div class="preview-container">
          <img src={long_banner_preview_url} alt="Team logo preview" class="logo-preview" />
        </div>
      {/if}
      <button onclick={pick_long_banner_file}>Find file</button>
      <button onclick={() => {long_banner_path = ""; long_banner_preview_url = ""; invoke("set_long_banner_path", { path: long_banner_path });}}>Reset</button>
    </div>
</div>
</main>