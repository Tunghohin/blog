import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createPinia } from 'pinia'
import './style.css'
import App from './App.vue'
import { useAuthStore } from './stores/auth'

const app = createApp(App)

// 路由配置
const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'Home', component: () => import('./views/Home.vue') },
    { path: '/posts', name: 'Posts', component: () => import('./views/PostList.vue') },
    { path: '/posts/:id', name: 'PostDetail', component: () => import('./views/PostDetail.vue') },
    { path: '/editor', name: 'Editor', component: () => import('./views/Editor.vue') },
    { path: '/editor/:id', name: 'EditPost', component: () => import('./views/Editor.vue') },
    { path: '/login', name: 'Login', component: () => import('./views/Login.vue') },
  ],
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()
  authStore.init() // 初始化用户状态

  // 保护编辑器页面，只有管理员可以访问
  if (to.name === 'Editor' || to.name === 'EditPost') {
    if (!authStore.isAuthenticated || !authStore.isAdmin) {
      next('/login')
    } else {
      next()
    }
  } else {
    next()
  }
})

app.use(createPinia())
app.use(router)

app.mount('#app')
