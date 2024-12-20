<script lang="ts">
  import type { Plugin } from 'wasm';
  import GridCard from './grid-card.svelte';

  interface Props {
    plugins: Plugin[];
  }

  const { plugins }: Props = $props();
</script>

<div class="grid grid-cols-1 gap-4 px-4 md:grid-cols-2 md:px-16 xl:grid-cols-3">
  {#each plugins as plugin}
    <GridCard {plugin} />
  {/each}
</div>
