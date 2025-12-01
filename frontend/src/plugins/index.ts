/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.ts`
 */

// Plugins
import pinia from './pinia';
import router from './router';
import vuetify from './vuetify';
import { options, plugin } from './toasts';

// Types
import type { App } from 'vue'

export function registerPlugins(app: App) {
  app.use(vuetify);
  app.use(pinia);
  app.use(router);
  app.use(plugin, options);
}
