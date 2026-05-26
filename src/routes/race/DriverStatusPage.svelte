<script lang="ts">
	let { drivers } = $props();

	function displayName(driver: any, i: number) {
		return driver.team_name?.trim() || `scr_driver ${i}`;
	}
</script>

<div class="driver-grid">
	{#each drivers as driver, i}
		<div class="driver-card">
			<div class="driver-name" title={displayName(driver, i)}>
				{displayName(driver, i)}
			</div>

			<div class="divider"></div>

			<div
				class:connected={driver.state === 'connected'}
				class:connecting={driver.state === 'connecting'}
				class:waiting={driver.state !== 'connected' && driver.state !== 'connecting'}
				class="driver-status"
			>
				{#if driver.state === 'connected'}
					connected on port {driver.port}

				{:else if driver.state === 'connecting'}
					connecting on port {driver.port}...

				{:else}
					waiting to connect...
				{/if}
			</div>
		</div>
	{/each}
</div>

<style>
	.driver-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(420px, 1fr));
		gap: 1rem;
		width: 100%;
		padding: 1rem;
		box-sizing: border-box;
	}

	.driver-card {
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		align-items: center;

		padding: 0.5rem 1.25rem;
		border-radius: 12px;
		/* background: var(--slider-card-background, #f6f6f6); */
		border: 2px solid var(--race-menu-outline);
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
		background: var(--button-background);
		font-size: 1.3rem;
		min-height: 72px;
	}

	/**equal padding*/
	.driver-name,
	.driver-status {
		padding: 0 1rem;
	}

	.driver-name {
		font-weight: 600;
		text-align: left;

		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.divider {
		width: 2px;
		align-self: stretch;
		background: var(--race-menu-outline);
	}

	.driver-status {
		text-align: left;
		white-space: nowrap;
		font-weight: 500;
	}

	.connected {
		color: #4ade80;
	}

	.connecting {
		color: #f59e0b;
	}

	.waiting {
		color: #a1a1aa;
	}

	@media (max-width: 600px) {
		.driver-card {
			grid-template-columns: 1fr;
			gap: 0.5rem;
		}

		.divider {
			display: none;
		}

		.driver-status {
			white-space: normal;
		}
	}
</style>