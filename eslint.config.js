import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['web/tsconfig.json'],
  ignores: ['src/lib/components/ui/*', 'crates/wasm/pkg/*'],
  features: {
    svelte: true,
    vue: false,
  },
  overrides: {
    javascript: {
      'no-undefined': 'off',
    },
    perfectionist: {
      'perfectionist/sort-interfaces': 'off',
    },
  },
});
