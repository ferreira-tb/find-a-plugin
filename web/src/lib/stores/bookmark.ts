import { tick } from 'svelte';
import { get, writable } from 'svelte/store';

const BOOKMARKS_KEY = 'find-a-plugin:bookmarks';

class Bookmark {
  public value = writable<string[]>([], () => {
    this.load();
    return () => {
      this.value.set([]);
    };
  });

  public readonly subscribe = this.value.subscribe;

  public add(plugin: string) {
    this.value.update((bookmarks) => {
      if (bookmarks.includes(plugin)) {
        return bookmarks;
      }

      return [...bookmarks, plugin];
    });

    this.onUpdate();
  }

  public remove(plugin: string) {
    this.value.update((bookmarks) => {
      return bookmarks.filter((bookmark) => bookmark !== plugin);
    });

    this.onUpdate();
  }

  private onUpdate() {
    void tick().then(() => {
      const value = JSON.stringify(get(this.value));
      localStorage.setItem(BOOKMARKS_KEY, value);
    });
  }

  private load() {
    const bookmarks = localStorage.getItem(BOOKMARKS_KEY);
    if (bookmarks) {
      this.value.set(JSON.parse(bookmarks));
    } else {
      this.value.set([]);
    }
  }
}

export const bookmark = new Bookmark();
