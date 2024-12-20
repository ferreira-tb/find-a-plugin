<script lang="ts">
  import { cn } from '$lib/utils';
  import { gridMode, search } from '$lib/stores';
  import { Input } from '$lib/components';
</script>

<header class="flex flex-col gap-2 bg-secondary/90 px-4 pt-6 md:px-16">
  <h1 class="select-none text-xl">Find a plugin</h1>
  <div class="flex w-full max-w-sm items-center space-x-2">
    <Input bind:value={$search} type="text" placeholder="name, keyword, description" />
  </div>
  <nav class="flex gap-6 pt-2">
    {#snippet menuItem(mode: GridMode, label: string)}
      <button
        type="button"
        onclick={() => ($gridMode = mode)}
        class={cn('px-6 py-1', $gridMode === mode && 'bg-background')}
      >
        <span>{label}</span>
      </button>
    {/snippet}

    {@render menuItem('all', 'Plugins')}
    {@render menuItem('bookmarks', 'Bookmarks')}
  </nav>
</header>
