import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createPinia } from 'pinia'
import './style.css'
import App from './App.vue'

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

app.use(router)
app.use(createPinia())

app.mount('#app')
