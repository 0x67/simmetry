import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
import { defineNuxtConfig } from 'nuxt/config';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  workspaceDir: '../../',
  srcDir: 'src',
  devtools: { enabled: true },
  modules: [
    '@vueuse/nuxt',
    'nuxt-svgo',
    '@nuxt/eslint',
    '@unocss/nuxt',
    '@pinia/nuxt',
  ],
  plugins: ['~/plugins/pinia.ts'],
  app: {
    head: {
      title: 'Simmetry - All in one platform for sim racing telemetry',
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      meta: [{ name: 'format-detection', content: 'no' }],
    },
    pageTransition: {
      name: 'page',
      mode: 'out-in',
    },
    layoutTransition: {
      name: 'layout',
      mode: 'out-in',
    },
  },
  devServer: {
    host: 'localhost',
    port: 4200,
  },
  typescript: {
    typeCheck: true,
    tsConfig: {
      extends: '../tsconfig.app.json', // Nuxt copies this string as-is to the `./.nuxt/tsconfig.json`, therefore it needs to be relative to that directory
    },
  },
  imports: {
    autoImport: true,
    presets: [
      {
        from: 'zod',
        imports: [
          'z',
          {
            name: 'infer',
            as: 'zInfer',
            type: true,
          },
        ],
      },
    ],
  },
  dir: { modules: './src/modules' },
  ssr: false,
  css: ['~/assets/css/styles.scss'],
  vite: {
    plugins: [nxViteTsPaths()],
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
      hmr: {
        protocol: 'ws',
        host: '0.0.0.0',
        port: 3001,
      },
      watch: {
        // ignored: ['**/src-tauri/**'],
      },
    },
  },
  experimental: {
    typedPages: true,
  },
  future: {
    compatibilityVersion: 4,
  },
  compatibilityDate: '2025-02-01',
});
