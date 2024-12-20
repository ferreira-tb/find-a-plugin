import { tick } from 'svelte';
import { get, writable } from 'svelte/store';

const SEARCH_KEY = 'find-a-plugin:search';

class Search {
  public value = writable<string>('', () => {
    this.load();
    return () => {
      this.value.set('');
    };
  });

  public readonly subscribe = this.value.subscribe;

  public set(value: string) {
    this.value.set(value);
    this.onUpdate();
  }

  public onUpdate() {
    void tick().then(() => {
      sessionStorage.setItem(SEARCH_KEY, get(this.value));
    });
  }

  private load() {
    const search = sessionStorage.getItem(SEARCH_KEY);
    if (search) {
      this.value.set(search);
    } else {
      this.value.set('');
    }
  }
}

export const search = new Search();
