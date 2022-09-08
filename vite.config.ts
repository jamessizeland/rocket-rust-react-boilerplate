import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tsconfigPaths from 'vite-tsconfig-paths';
import svgr from 'vite-plugin-svgr';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), tsconfigPaths(), svgr()],
  server: {
    port: 5000,
    proxy: {
      // https://github.com/http-party/node-http-proxy#options
      '/api': {
        target: 'http://localhost:5001/api',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
  build: {
    // https://vitejs.dev/config/build-options.html
    manifest: true,
  },
});
