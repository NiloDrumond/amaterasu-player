import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.ts'],
		environment: 'node',
	},
	ssr: {
		noExternal: ['@thisux/sveltednd'],
	},
	server: {
		proxy: {
			'/api': 'http://localhost:8080',
			'/admin/logs': 'http://localhost:8080',
		},
	},
});
