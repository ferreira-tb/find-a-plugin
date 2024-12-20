<script lang="ts">
  import * as wasm from 'wasm';
  import { onMount } from 'svelte';
  import { pluginData, gridMode, bookmark, search } from '$lib/stores';
  import { Grid } from '$lib/components';
  import { handleError } from '$lib/utils';

  const plugins = $derived.by(() => {
    let values = $pluginData;
    if (values.length === 0) {
      return [];
    }

    if ($gridMode === 'bookmarks') {
      values = values.filter((plugin) => {
        return $bookmark.includes(plugin.name);
      });
    }

    const query = $search.trim();
    if (query.length === 0) {
      return values;
    }

    const matches = wasm.search(query);
    return values.filter((plugin) => matches.includes(plugin.name));
  });

  onMount(() => void wasm.init().catch(handleError));
</script>

<div>
  <Grid {plugins} />
</div>
