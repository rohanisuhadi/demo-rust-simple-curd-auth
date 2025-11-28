import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
    proxy: {
      // Add a proxy rule
      "/api": {
        target: "https://api-dev.temanternak.com/", // Your backend server
        changeOrigin: true, // Needed for virtual hosted sites
        // Rewrite the path: remove /api from the start
        // e.g. /api/v1/auth/login -> /v1/auth/login
        // If your backend doesn't have /api, you can rewrite it to ''
        // rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
}));
