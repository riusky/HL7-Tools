//引入路由对象
import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
 
// 路由类型 RouteRecordRaw
// 定义一些路由
// 每个路由都需要映射到一个组件。
const routes: Array<RouteRecordRaw> = [{
    path: '/',
    name: 'index',
    component: () => import('@/components/homepage/index.vue'),  /* 路由懒加载 */
}, {
  path: '/hl7/sendtools',
  name: 'hl7sendtools',
  component: () => import('@/components/hl7/Hl7.vue'),
}, {
  path: '/b',
  name: 'componentsb',
  component: () => import('@/components/B.vue'),
}, {
  path: '/c',
  name: 'componentsc',
  component: () => import('@/components/C.vue'),
}]
 
/* 路由模式 
  createWebHistory: h5
  createWebHashHistory: HASH
  createMemoryHistory: 服务端渲染时
*/
 
const router = createRouter({
    history: createWebHashHistory(),
    routes
})
 
//导出router
export default router
