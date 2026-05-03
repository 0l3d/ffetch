import { spawn } from 'node:child_process';
import { createHash } from 'node:crypto';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, '../../../..');
const targetDir = resolve(repoRoot, 'target');

export const buildTargets = [
	{
		id: 'cargo',
		label: 'Cargo',
		command: 'cargo',
		args: ['build', '--locked'],
		env: { CARGO_TARGET_DIR: targetDir }
	},
	{
		id: 'zig',
		label: 'Zig',
		command: 'zig',
		args: ['build'],
		env: {
			ZIG_LOCAL_CACHE_DIR: resolve(repoRoot, '.zig-cache'),
			ZIG_GLOBAL_CACHE_DIR: resolve(repoRoot, '.zig-cache-global')
		}
	},
	{ id: 'npm', label: 'NPM', command: 'npm', args: ['run', 'npm:build'] },
	{ id: 'yarn', label: 'Yarn', command: 'yarn', args: ['yarn:build'] },
	{ id: 'pnpm', label: 'PNPM', command: 'pnpm', args: ['run', 'pnpm:build'] },
	{ id: 'bun', label: 'Bun', command: 'bun', args: ['run', 'bun:build'] },
	{ id: 'composer', label: 'Composer', command: 'composer', args: ['run', 'build'] }
];

const maxLogLines = 400;
const storeKey = Symbol.for('ffetch.builder.store');

const store =
	globalThis[storeKey] ??
	(globalThis[storeKey] = {
		targetMap: new Map(buildTargets.map((target) => [target.id, target])),
		listeners: new Set(),
		socketClients: new Set(),
		state: new Map(
			buildTargets.map((target) => [
				target.id,
				{
					id: target.id,
					label: target.label,
					running: false,
					status: 'idle',
					exitCode: null,
					lines: []
				}
			])
		)
	});

const { targetMap, listeners, socketClients, state } = store;

for (const target of buildTargets) {
	targetMap.set(target.id, target);

	if (!state.has(target.id)) {
		state.set(target.id, {
			id: target.id,
			label: target.label,
			running: false,
			status: 'idle',
			exitCode: null,
			lines: []
		});
	}
}

function pushLine(targetId, line) {
	const entry = state.get(targetId);
	if (!entry) return;
	entry.lines.push(line);
	if (entry.lines.length > maxLogLines) {
		entry.lines.splice(0, entry.lines.length - maxLogLines);
	}
}

function targetSnapshot(targetId) {
	const entry = state.get(targetId);
	return {
		id: entry.id,
		label: entry.label,
		running: entry.running,
		status: entry.status,
		exitCode: entry.exitCode,
		lines: [...entry.lines]
	};
}

function broadcast(message) {
	for (const listener of listeners) {
		listener(message);
	}

	const payload = JSON.stringify(message);
	for (const client of socketClients) {
		try {
			client.send(payload);
		} catch {
			socketClients.delete(client);
		}
	}
}

function updateTarget(targetId, patch) {
	const entry = state.get(targetId);
	if (!entry) return;
	Object.assign(entry, patch);
	broadcast({ type: 'target', target: targetSnapshot(targetId) });
}

function emitLine(targetId, chunk) {
	const text = chunk.toString();
	for (const line of text.split(/\r?\n/)) {
		if (!line.length) continue;
		pushLine(targetId, line);
		broadcast({ type: 'line', targetId, line });
	}
}

export function getBuildTargets() {
	return buildTargets.map((target) => ({ id: target.id, label: target.label }));
}

export function getBuildSnapshot() {
	return {
		targets: buildTargets.map((target) => targetSnapshot(target.id))
	};
}

export function subscribeBuilds(listener) {
	listeners.add(listener);
	return () => listeners.delete(listener);
}

export function startBuild(targetId) {
	const target = targetMap.get(targetId);
	if (!target) {
		return { ok: false, status: 404, error: `unknown target: ${targetId}` };
	}

	const entry = state.get(targetId);
	if (entry.running) {
		return { ok: false, status: 409, error: `${target.label} build is already running` };
	}

	entry.running = true;
	entry.status = 'running';
	entry.exitCode = null;
	entry.lines = [];
	broadcast({ type: 'target', target: targetSnapshot(targetId) });

	pushLine(targetId, `$ ${target.command} ${target.args.join(' ')}`);
	broadcast({ type: 'line', targetId, line: `$ ${target.command} ${target.args.join(' ')}` });

	const child = spawn(target.command, target.args, {
		cwd: repoRoot,
		env: { ...process.env, ...(target.env ?? {}) },
		stdio: ['ignore', 'pipe', 'pipe']
	});

	child.stdout.on('data', (chunk) => emitLine(targetId, chunk));
	child.stderr.on('data', (chunk) => emitLine(targetId, chunk));
	child.on('error', (error) => {
		pushLine(targetId, `process error: ${error.message}`);
		broadcast({ type: 'line', targetId, line: `process error: ${error.message}` });
		updateTarget(targetId, { running: false, status: 'failed', exitCode: null });
	});
	child.on('close', (code) => {
		updateTarget(targetId, {
			running: false,
			status: code === 0 ? 'succeeded' : 'failed',
			exitCode: code
		});
	});

	return { ok: true, status: 202 };
}

function websocketAccept(key) {
	return createHash('sha1')
		.update(`${key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11`)
		.digest('base64');
}

function encodeFrame(text) {
	const payload = Buffer.from(text);

	if (payload.length < 126) {
		return Buffer.concat([Buffer.from([0x81, payload.length]), payload]);
	}

	if (payload.length <= 0xffff) {
		const header = Buffer.allocUnsafe(4);
		header[0] = 0x81;
		header[1] = 126;
		header.writeUInt16BE(payload.length, 2);
		return Buffer.concat([header, payload]);
	}

	const header = Buffer.allocUnsafe(10);
	header[0] = 0x81;
	header[1] = 127;
	header.writeBigUInt64BE(BigInt(payload.length), 2);
	return Buffer.concat([header, payload]);
}

function createSocketClient(socket) {
	return {
		send(text) {
			socket.write(encodeFrame(text));
		},
		close() {
			socket.end();
		}
	};
}

export function handleBuildWebSocket(request, socket) {
	const key = request.headers['sec-websocket-key'];
	if (!key) {
		socket.destroy();
		return;
	}

	const headers = [
		'HTTP/1.1 101 Switching Protocols',
		'Upgrade: websocket',
		'Connection: Upgrade',
		`Sec-WebSocket-Accept: ${websocketAccept(key)}`,
		'', ''
	];
	socket.write(headers.join('\r\n'));

	const client = createSocketClient(socket);
	socketClients.add(client);

	client.send(JSON.stringify({ type: 'snapshot', ...getBuildSnapshot() }));

	const cleanup = () => {
		socketClients.delete(client);
	};

	socket.on('close', cleanup);
	socket.on('end', cleanup);
	socket.on('error', cleanup);
}
