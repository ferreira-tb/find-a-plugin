<script lang="ts">
  import { Badge } from './ui/badge';
  import GitHub from './icons/github.svelte';
  import Favorite from './icons/favorite.svelte';
  import ExternalLink from './external-link.svelte';
  import { formatInteger } from '$lib/intl';
  import { Calendar, Download } from 'lucide-svelte';
  import { since } from '$lib/date';
  import { innerWidth } from 'svelte/reactivity/window';
  import type { Plugin } from 'wasm';
  import { bookmark } from '$lib/stores';

  interface Props {
    plugin: Plugin;
  }

  const { plugin }: Props = $props();

  let selected = {
    get value() {
      return $bookmark.includes(plugin.name);
    },
    set value(it) {
      if (it) {
        bookmark.add(plugin.name);
      } else {
        bookmark.remove(plugin.name);
      }
    },
  };

  function cratesIoUrl(name: string) {
    return `https://crates.io/crates/${name}`;
  }
</script>

<div id={plugin.name} class="flex flex-col gap-4 rounded-md border px-4 pb-2 pt-4">
  <div class="flex h-full flex-col">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2 pb-1">
        <div class="flex items-center gap-1">
          <ExternalLink href={cratesIoUrl(plugin.name)}>
            <h2 class="text-lg font-bold">{plugin.plugin_name}</h2>
          </ExternalLink>
          <sup>{plugin.default_version}</sup>
        </div>
        {#if plugin.official}
          <Badge variant="secondary">official</Badge>
        {/if}
      </div>

      {#if plugin.repository && plugin.repository.includes('github.com')}
        <ExternalLink href={plugin.repository}>
          <GitHub size="1.2rem" />
        </ExternalLink>
      {/if}
    </div>
    <p>{plugin.description}</p>
  </div>

  <div class="flex items-center justify-between">
    <Favorite bind:selected={selected.value} size="1.2rem" />
    <div class="flex justify-end gap-2 text-xs">
      {#if innerWidth.current && innerWidth.current > 1600}
        <div class="flex select-none items-center gap-1">
          <Calendar class="size-4" />
          <span>Updated: {since(plugin.updated_at)}</span>
        </div>
      {/if}
      {#snippet download(text: string)}
        <div class="flex select-none items-center gap-1">
          <Download class="size-4" />
          <span>{text}</span>
        </div>
      {/snippet}
      {@render download(`Recent: ${formatInteger(plugin.recent_downloads)}`)}
      {@render download(`Total: ${formatInteger(plugin.downloads)}`)}
    </div>
  </div>
</div>
