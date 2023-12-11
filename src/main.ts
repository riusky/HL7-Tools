import { createApp } from "vue";
import { createPinia } from 'pinia'
import "./styles.css";
import App from "./App.vue";
// 引入router的配置
import router from '@/router/index'
// 引入elementui的图标库
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import ElementPlus from 'element-plus'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import 'element-plus/dist/index.css'
//引入echarts
import * as echarts from 'echarts';
import "@/styles/index.scss";
const pinia = createPinia()
const app = createApp(App)
// 使用elementui的图标库
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}
app.use(ElementPlus, {
    locale: zhCn,
  })
app.use(router)
app.use(pinia)
// 全局挂载
app.config.globalProperties.$echarts = echarts;

app.mount('#app')
