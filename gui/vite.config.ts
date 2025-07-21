import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
    plugins: [tailwindcss(), react()],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
            "@tauri-apps/api": path.resolve(
                __dirname,
                "node_modules/@tauri-apps/api"
            ),
        },
    },
    optimizeDeps: {
        exclude: ["@tauri-apps/api"], // Exclude Tauri API from pre-bundling
    },
});
