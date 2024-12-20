<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    href: string;
    children: Snippet;
    class?: string;
  }

  const { href, class: classList, children }: Props = $props();
</script>

<a {href} target="_blank" rel="noopener noreferrer" class={classList}>
  {@render children()}
</a>
