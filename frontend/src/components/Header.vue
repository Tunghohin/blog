<script setup>
import { computed } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const isAdmin = computed(() => authStore.isAdmin)
const isAuthenticated = computed(() => authStore.isAuthenticated)

const navLinks = [
  { name: '首页', path: '/' },
  { name: '文章', path: '/posts' },
]

const handleLogout = () => {
  authStore.logout()
  router.push('/')
}
</script>

<template>
  <header class="header">
    <div class="header-content">
      <RouterLink to="/" class="logo">
        My Blog
      </RouterLink>

      <nav class="nav">
        <RouterLink
          v-for="link in navLinks"
          :key="link.path"
          :to="link.path"
          class="nav-link"
          :class="{ active: route.path === link.path }"
        >
          {{ link.name }}
        </RouterLink>
      </nav>

      <div class="header-actions">
        <template v-if="isAuthenticated">
          <RouterLink v-if="isAdmin" to="/editor" class="btn btn-primary">
            写文章
          </RouterLink>
          <span class="user-name">{{ authStore.user?.username }}</span>
          <button @click="handleLogout" class="btn btn-secondary">
            退出
          </button>
        </template>
        <template v-else>
          <RouterLink to="/login" class="btn btn-primary">
            登录
          </RouterLink>
        </template>
      </div>
    </div>
  </header>
</template>

<style scoped>
.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  background-color: rgba(21, 21, 21, 0.95);
  border-bottom: 1px solid #333;
  z-index: 100;
  backdrop-filter: blur(8px);
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 1rem 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  color: #fff;
  text-decoration: none;
  transition: color 0.2s;
}

.logo:hover {
  color: #3b82f6;
}

.nav {
  display: flex;
  gap: 2rem;
}

.nav-link {
  color: #ccc;
  text-decoration: none;
  font-size: 0.95rem;
  transition: color 0.2s;
}

.nav-link:hover,
.nav-link.active {
  color: #fff;
}

.header-actions {
  display: flex;
  gap: 1rem;
}

.btn {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  text-decoration: none;
  font-size: 0.9rem;
  transition: all 0.2s;
  cursor: pointer;
}

.btn-primary {
  background-color: #3b82f6;
  color: #fff;
  border: none;
}

.btn-primary:hover {
  background-color: #2563eb;
}

.btn-secondary {
  background-color: #333;
  color: #fff;
}

.btn-secondary:hover {
  background-color: #444;
}

.user-name {
  color: #ccc;
  font-size: 0.9rem;
  margin-right: 0.5rem;
}
</style>
