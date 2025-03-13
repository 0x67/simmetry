import * as tauriApp from '@tauri-apps/api/app';
import * as tauriFs from 'tauri-plugin-fs-pro-api';
import * as tauriNotification from '@tauri-apps/plugin-notification';
import * as tauriOs from '@tauri-apps/plugin-os';
import * as tauriShell from '@tauri-apps/plugin-shell';
import * as tauriStore from '@tauri-apps/plugin-store';
import { addImports, defineNuxtModule } from 'nuxt/kit';
import * as tauriCore from '@tauri-apps/api/core';
import * as tauriLog from '@tauri-apps/plugin-log';
import * as tauriDialog from '@tauri-apps/plugin-dialog';
import * as tauriProcess from '@tauri-apps/plugin-process';
import * as tauriClipboard from 'tauri-plugin-clipboard-api';
import * as tauriOpener from '@tauri-apps/plugin-opener';

const capitalize = (name: string) => {
  return name.charAt(0).toUpperCase() + name.slice(1);
};

const tauriModules = [
  { module: tauriApp, prefix: 'App', importPath: '@tauri-apps/api/app' },
  {
    module: tauriShell,
    prefix: 'Shell',
    importPath: '@tauri-apps/plugin-shell',
  },
  { module: tauriOs, prefix: 'Os', importPath: '@tauri-apps/plugin-os' },
  {
    module: tauriNotification,
    prefix: 'Notification',
    importPath: '@tauri-apps/plugin-notification',
  },
  { module: tauriFs, prefix: 'Fs', importPath: 'tauri-plugin-fs-pro-api' },
  {
    module: tauriStore,
    prefix: 'Store',
    importPath: '@tauri-apps/plugin-store',
  },
  { module: tauriCore, prefix: '', importPath: '@tauri-apps/api/core' },
  { module: tauriLog, prefix: 'Log', importPath: '@tauri-apps/plugin-log' },
  {
    module: tauriDialog,
    prefix: 'Dialog',
    importPath: '@tauri-apps/plugin-dialog',
  },
  {
    module: tauriProcess,
    prefix: 'Process',
    importPath: '@tauri-apps/plugin-process',
  },
  {
    module: tauriClipboard,
    prefix: 'Clipboard',
    importPath: 'tauri-plugin-clipboard-api',
  },
  {
    module: tauriOpener,
    prefix: 'Opener',
    importPath: '@tauri-apps/plugin-opener',
  },
];

export default defineNuxtModule<ModuleOptions>({
  meta: {
    name: 'nuxt-tauri',
    configKey: 'tauri',
  },
  defaults: {
    prefix: 'useTauri',
  },
  setup(options) {
    tauriModules.forEach(({ module, prefix, importPath }) => {
      Object.keys(module)
        .filter((name) => name !== 'default')
        .forEach((name) => {
          const prefixedName = `${options.prefix}${prefix}` || '';
          const as = prefixedName ? prefixedName + capitalize(name) : name;
          addImports({ from: importPath, name, as });
        });
    });
  },
});
