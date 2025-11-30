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
import { useUserStore } from '@/stores/user';

export async function registerPlugins(app: App) {
  app.use(vuetify);
  app.use(pinia);

  const userStore = useUserStore();
  await userStore.hydrateUser();

  app.use(router);
  app.use(plugin, options);
}
