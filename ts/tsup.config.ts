import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/runtime.ts'],
  format: ['esm', 'cjs'],
  minify: true,
  outExtension({ format }) {
    return {
      js: format === 'cjs' ? '.cjs' : '.js',
    };
  },
});
