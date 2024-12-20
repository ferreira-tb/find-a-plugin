<script lang="ts">
  import ExternalLink from './external-link.svelte';

  const cratesIo = 'https://crates.io';
  const repository = 'https://github.com/ferreira-tb/find-a-plugin';
  const pullRequest = `${repository}/pulls`;
</script>

<footer class="bg-secondary/90 px-16 py-12">
  {#snippet link(href: string, text: string)}
    <ExternalLink {href} class="font-bold">{text}</ExternalLink>
  {/snippet}

  <span class="text-sm">
    Plugin data is fetched directly from
    {@render link(cratesIo, 'crates.io')}. If you want to add or remove a plugin, open a
    {@render link(pullRequest, 'pull request')} in our {@render link(repository, 'repository')}.
  </span>
</footer>
