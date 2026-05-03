<script>
	import { onMount } from 'svelte';

	const targets = [
		{ id: 'cargo', label: 'Cargo' },
		{ id: 'zig', label: 'Zig' },
		{ id: 'npm', label: 'NPM' },
		{ id: 'yarn', label: 'Yarn' },
		{ id: 'pnpm', label: 'PNPM' },
		{ id: 'bun', label: 'Bun' },
		{ id: 'composer', label: 'Composer' }
	];

	let buildState = $state(
		Object.fromEntries(
		targets.map((target) => [
			target.id,
			{ id: target.id, label: target.label, running: false, status: 'idle', exitCode: null, lines: [] }
		])
		)
	);
	let socketState = $state('connecting');
	let requestError = $state('');

	function replaceTarget(target) {
		buildState = { ...buildState, [target.id]: target };
	}

	function appendLine(targetId, line) {
		const current = buildState[targetId];
		if (!current) return;

		const lines = [...current.lines, line].slice(-400);
		buildState = { ...buildState, [targetId]: { ...current, lines } };
	}

	async function loadSnapshot() {
		const response = await fetch('/api/build');
		const data = await response.json();
		for (const target of data.targets) {
			replaceTarget(target);
		}
	}

	async function startBuild(targetId) {
		requestError = '';
		const current = buildState[targetId];
		if (current) {
			buildState = {
				...buildState,
				[targetId]: { ...current, running: true, status: 'running', exitCode: null, lines: [] }
			};
		}

		try {
			const response = await fetch('/api/build', {
				method: 'POST',
				headers: { 'content-type': 'application/json' },
				body: JSON.stringify({ target: targetId })
			});

			if (!response.ok) {
				const data = await response.json().catch(() => ({ error: 'build request failed' }));
				requestError = data.error ?? 'build request failed';
			}

			await loadSnapshot();
		} catch (error) {
			requestError = error.message;
			if (current) {
				buildState = { ...buildState, [targetId]: current };
			}
		}
	}

	onMount(() => {
		loadSnapshot().catch((error) => {
			requestError = error.message;
		});

		const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
		const socket = new WebSocket(`${protocol}//${window.location.host}/build-logs`);

		socket.addEventListener('open', () => {
			socketState = 'connected';
		});

		socket.addEventListener('close', () => {
			socketState = 'disconnected';
		});

		socket.addEventListener('error', () => {
			socketState = 'error';
		});

		socket.addEventListener('message', (event) => {
			const message = JSON.parse(event.data);
			if (message.type === 'snapshot') {
				for (const target of message.targets) {
					replaceTarget(target);
				}
				return;
			}

			if (message.type === 'target') {
				replaceTarget(message.target);
				return;
			}

			if (message.type === 'line') {
				appendLine(message.targetId, message.line);
			}
		});

		return () => socket.close();
	});
</script>

<svelte:head>
	<title>Ffetch Builder</title>
</svelte:head>

<div class="page">
	<section class="hero">
		<div>
			<p class="eyebrow">Build Console</p>
			<h1>Run every wrapper from one screen.</h1>
		</div>
		<div class="socket {socketState}">{socketState}</div>
	</section>

	{#if requestError}
		<p class="error">{requestError}</p>
	{/if}

	<section class="grid">
		{#each targets as target}
			{@const current = buildState[target.id]}
			<section class="panel">
				<header>
					<div>
						<h2>{target.label}</h2>
						<p>{current.status}{#if current.exitCode !== null} ({current.exitCode}){/if}</p>
					</div>
					<button onclick={() => startBuild(target.id)} disabled={current.running}>
						{current.running ? 'Running' : 'Run build'}
					</button>
				</header>

				<pre>{current.lines.length ? current.lines.join('\n') : 'No logs yet.'}</pre>
			</section>
		{/each}
	</section>
</div>

<style>
	:global(body) {
		margin: 0;
		font-family:
			Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		background: #0b1020;
		color: #e5edf7;
	}

	.page {
		min-height: 100vh;
		padding: 32px;
		background:
			radial-gradient(circle at top left, rgba(34, 197, 94, 0.14), transparent 28%),
			radial-gradient(circle at top right, rgba(14, 165, 233, 0.16), transparent 24%),
			#0b1020;
	}

	.hero {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 16px;
		margin-bottom: 24px;
	}

	.eyebrow {
		margin: 0 0 8px;
		color: #8da2c0;
		font-size: 13px;
		text-transform: uppercase;
	}

	h1 {
		margin: 0;
		font-size: clamp(32px, 4vw, 52px);
		line-height: 1.05;
		max-width: 760px;
	}

	.socket {
		padding: 8px 12px;
		border: 1px solid #24324b;
		border-radius: 8px;
		background: #11192b;
		color: #8da2c0;
		text-transform: capitalize;
	}

	.socket.connected {
		color: #8ef0b5;
		border-color: #215c3b;
	}

	.socket.error,
	.socket.disconnected {
		color: #ff9f9f;
		border-color: #6a2d35;
	}

	.error {
		margin: 0 0 20px;
		padding: 12px 14px;
		border: 1px solid #6a2d35;
		border-radius: 8px;
		background: rgba(122, 31, 46, 0.28);
		color: #ffb5b5;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
		gap: 16px;
	}

	.panel {
		display: grid;
		grid-template-rows: auto 1fr;
		min-height: 320px;
		border: 1px solid #24324b;
		border-radius: 8px;
		background: rgba(10, 15, 29, 0.84);
		overflow: hidden;
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
		padding: 16px;
		border-bottom: 1px solid #24324b;
	}

	h2 {
		margin: 0 0 4px;
		font-size: 18px;
	}

	header p {
		margin: 0;
		color: #8da2c0;
		font-size: 13px;
		text-transform: capitalize;
	}

	button {
		height: 38px;
		padding: 0 14px;
		border: 1px solid #31507a;
		border-radius: 8px;
		background: #17335c;
		color: #f4f8ff;
		font: inherit;
		cursor: pointer;
	}

	button:disabled {
		cursor: default;
		opacity: 0.6;
	}

	pre {
		margin: 0;
		padding: 16px;
		overflow: auto;
		background: #060a13;
		color: #cfe2ff;
		font-size: 12px;
		line-height: 1.5;
		white-space: pre-wrap;
		word-break: break-word;
	}

	@media (max-width: 700px) {
		.page {
			padding: 20px;
		}

		.hero {
			flex-direction: column;
		}

		header {
			align-items: flex-start;
			flex-direction: column;
		}

		button {
			width: 100%;
		}
	}
</style>
