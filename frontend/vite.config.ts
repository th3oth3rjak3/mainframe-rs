import path from "path";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  build: {
    outDir: "dist",
    rollupOptions: {
      output: {
        manualChunks: {
          // React core
          "react-vendor": ["react", "react-dom", "react/jsx-runtime"],

          // TanStack suite
          "tanstack-vendor": [
            "@tanstack/react-router",
            "@tanstack/react-query",
            "@tanstack/react-table",
          ],

          // UI library - Radix UI components
          "ui-vendor": [
            "lucide-react",
            "@radix-ui/react-dialog",
            "@radix-ui/react-dropdown-menu",
            "@radix-ui/react-label",
            "@radix-ui/react-popover",
            "@radix-ui/react-select",
            "@radix-ui/react-separator",
            "@radix-ui/react-slot",
            "@radix-ui/react-tooltip",
            "@radix-ui/react-checkbox",
          ],

          // Form/validation
          "form-vendor": ["react-hook-form", "zod"],

          // Utilities
          "utils-vendor": ["date-fns", "clsx", "tailwind-merge", "sonner", "ky"],
        },
      },
    },
    chunkSizeWarningLimit: 300,
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:3030",
        changeOrigin: true,
        secure: false,
      },
    },
  },
});
