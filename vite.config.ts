import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";;

import  {resolve}  from 'path'
import AutoImport from 'unplugin-auto-import/vite'
const pathResolve = (path: string): string => resolve(process.cwd(), path)


// https://vitejs.dev/config/
export default defineConfig(async () => ({
  resolve: {
    alias: {
      '@': pathResolve('src'), // 设置 `@` 指向 `src` 目录
      '@components': pathResolve('src/components'), // 设置 `#` 指向 `types` 目录
    }
  },
  // sass 处理器
  // css: {
  //   preprocessorOptions: {
  //     scss: {
  //       additionalData: `@import "@/style/test.scss";`
  //     }
  //   }
  // },
  plugins: [
    vue(),
    AutoImport({
      imports: [
        // 需要自动导入的插件，自定义导入的API
        'vue',
        'vue-router',
        'pinia'
      ]
    })
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    // open:true, //自动打开
    proxy: {
      '/api': {
        target: 'http://127.0.0.1/', //跨域地址
        changeOrigin: true, //是否允许跨域
        rewrite: (path: string) => path.replace(/^\/api/, ""), //重写路径
      }
    },
    port: 9420,
    strictPort: true,
  }
}));
