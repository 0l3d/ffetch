import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { handleBuildWebSocket } from './src/lib/server/builds.js';

export default defineConfig({
	plugins: [
		sveltekit(),
		{
			name: 'build-log-websocket',
			configureServer(server) {
				server.httpServer?.on('upgrade', (request, socket) => {
					if (request.url !== '/build-logs') {
						return;
					}

					handleBuildWebSocket(request, socket);
				});
			}
		}
	]
});
