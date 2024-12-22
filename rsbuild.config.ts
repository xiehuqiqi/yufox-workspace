import { defineConfig } from '@rsbuild/core';
import { pluginVue } from '@rsbuild/plugin-vue';
import { pluginPug } from '@rsbuild/plugin-pug';
import AutoImport from 'unplugin-auto-import/rspack';
// import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
// import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/rspack';
import { pluginBabel } from "@rsbuild/plugin-babel";
import { pluginVueJsx } from "@rsbuild/plugin-vue-jsx";

export default defineConfig({
  plugins: [
    pluginBabel({
      include: /\.(?:jsx|tsx)$/,
    }),
    pluginVue(),
    pluginPug(),
    pluginVueJsx(),
  ],
  server: {
    base: '/',
    host: "0.0.0.0",
    port: 24680,
  },
  resolve: {
    alias: {
      '@': './src',
      '@yu': './src/yufox'
    },
  },
  source: {
    entry: {
      index: './src/yufox.ts',
    },
  },
  html: {
    title: 'yufox workspace',
    favicon: './public/assets/AppIcon.png',
    appIcon: {
      name: 'yufox Workspace | yufox 工作区',
      icons: [
        { src: './public/assets/AppIcon-192x192.png', size: 192 },
        { src: './public/assets/AppIcon-512x512.png', size: 512 },
      ],
    },
    meta: {
      description: 'yufox workspace',
    },
    tags: [
      // { tag: 'div', attrs: { id: 'init' } },
    ],
  },
  tools: {
    rspack: {
      plugins: [
        AutoImport({
          dts: 'src/types/auto-imports.d.ts',
          dirs: ['src/composables', 'src/utils'],
          resolvers: [],
          imports: [
            'vue',
            'vue-router',
            'pinia'
          ]
        }),
        Components({
          // allow auto load markdown components under `./src/components/`
          extensions: ['vue', 'md'],
          // allow auto import and register components used in markdown
          include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
          dirs: ['src/components/', 'src/layout/', 'src/views', 'src/assets'],
          dts: 'src/types/components.d.ts',
          resolvers: []
        })
      ],
    },
  },
  performance: {
    chunkSplit: {
      strategy: 'split-by-size',
      minSize: 30000,
      maxSize: 50000,
    },
  },
});
