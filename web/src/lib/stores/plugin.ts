import { writable } from 'svelte/store';
import { handleError } from '$lib/error';
import { get_plugins, init, type Plugin } from 'wasm';

class PluginData {
  private ready = false;
  private initialized = false;

  public value = writable<Plugin[]>([], () => {
    void this.load();
    return () => {
      this.value.set([]);
      this.ready = false;
    };
  });

  public readonly subscribe = this.value.subscribe;

  private async load() {
    if (this.ready) return;
    try {
      await this.init();
      this.value.set(get_plugins());
      this.ready = true;
    } catch (err) {
      this.ready = false;
      handleError(err);
    }
  }

  private async init() {
    if (this.initialized) return;
    try {
      await init();
      this.initialized = true;
    } catch (err) {
      this.initialized = false;
      handleError(err);
    }
  }
}

export const pluginData = new PluginData();
