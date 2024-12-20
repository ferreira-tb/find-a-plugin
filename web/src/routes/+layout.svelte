<script lang="ts">
  import '../app.css';
  import { ModeWatcher } from 'mode-watcher';
  import { Header, Footer } from '$lib/components';

  let { children } = $props();
</script>

<div>
  <ModeWatcher defaultMode="dark" />
  <div class="flex h-screen flex-col gap-4">
    <Header />
    <main class="flex-1">
      {@render children()}
    </main>
    <Footer />
  </div>
</div>
