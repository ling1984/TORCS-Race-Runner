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
					connecting on port {driver.port}

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
		grid-template-columns: minmax(0, 1fr) auto minmax(220px, auto);
		align-items: center;

		padding: 1rem 1.25rem;
		border-radius: 12px;

		background: #1b1b1b;
		border: 1px solid #333;

		font-size: 1.2rem;
		min-height: 72px;
	}

	.driver-name {
		font-weight: 600;
		text-align: left;

		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;

		padding-right: 1rem;
	}

	.divider {
		width: 1px;
		align-self: stretch;
		background: #444;
		margin-right: 1rem;
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