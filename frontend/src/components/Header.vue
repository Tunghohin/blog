<script setup>
import { ref, computed } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useThemeStore } from '../stores/theme'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const themeStore = useThemeStore()

const isAdmin = computed(() => authStore.isAdmin)
const isAuthenticated = computed(() => authStore.isAuthenticated)
const isDark = computed(() => themeStore.currentTheme === 'dark')

const isMobileMenuOpen = ref(false)

const navLinks = [
  { name: '首页', path: '/', icon: 'home' },
  { name: '文章', path: '/posts', icon: 'article' },
]

const handleLogout = () => {
  authStore.logout()
  router.push('/')
  isMobileMenuOpen.value = false
}

const toggleMobileMenu = () => {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

const closeMobileMenu = () => {
  isMobileMenuOpen.value = false
}
</script>

<template>
  <header class="fixed top-0 left-0 right-0 z-50 glass border-b border-theme-border">
    <div class="container-app">
      <div class="flex items-center justify-between h-16">
        <!-- Logo -->
        <RouterLink to="/" class="flex items-center gap-2 group" @click="closeMobileMenu">
          <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-brand-400 to-brand-600 flex items-center justify-center shadow-glow group-hover:shadow-glow-lg transition-shadow">
            <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
            </svg>
          </div>
          <span class="text-xl font-bold gradient-text hidden sm:block">Tunghohin's Blog</span>
        </RouterLink>

        <!-- Desktop Navigation -->
        <nav class="hidden md:flex items-center gap-1">
          <RouterLink
            v-for="link in navLinks"
            :key="link.path"
            :to="link.path"
            class="px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200"
            :class="[
              route.path === link.path
                ? 'bg-theme-accent-light text-theme-accent'
                : 'text-theme-text-secondary hover:text-theme-text hover:bg-theme-bg-hover'
            ]"
          >
            {{ link.name }}
          </RouterLink>
        </nav>

        <!-- Right Actions -->
        <div class="flex items-center gap-2">
          <!-- Theme Toggle -->
          <button
            @click="themeStore.toggleTheme"
            class="p-2.5 rounded-lg text-theme-text-secondary hover:text-theme-text hover:bg-theme-bg-hover transition-all duration-200"
            :title="isDark ? '切换到浅色模式' : '切换到深色模式'"
          >
            <!-- Sun icon (shown in dark mode) -->
            <svg v-if="isDark" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
            <!-- Moon icon (shown in light mode) -->
            <svg v-else class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
            </svg>
          </button>

          <!-- Auth Actions (Desktop) -->
          <div class="hidden md:flex items-center gap-2">
            <template v-if="isAuthenticated">
              <RouterLink v-if="isAdmin" to="/editor" class="btn btn-primary">
                <svg class="w-4 h-4 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                写文章
              </RouterLink>
              <div class="flex items-center gap-2 ml-2">
                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-brand-400 to-brand-600 flex items-center justify-center">
                  <span class="text-white text-sm font-medium">{{ authStore.user?.username?.charAt(0).toUpperCase() }}</span>
                </div>
                <span class="text-sm text-theme-text-secondary">{{ authStore.user?.username }}</span>
              </div>
              <button @click="handleLogout" class="btn btn-ghost">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                </svg>
              </button>
            </template>
            <template v-else>
              <RouterLink to="/login" class="btn btn-primary">
                登录
              </RouterLink>
            </template>
          </div>

          <!-- Mobile Menu Button -->
          <button
            @click="toggleMobileMenu"
            class="md:hidden p-2.5 rounded-lg text-theme-text-secondary hover:text-theme-text hover:bg-theme-bg-hover transition-all duration-200"
          >
            <svg v-if="!isMobileMenuOpen" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
            <svg v-else class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Mobile Menu -->
      <Transition name="slide-down">
        <div v-if="isMobileMenuOpen" class="md:hidden py-4 border-t border-theme-border">
          <nav class="flex flex-col gap-1">
            <RouterLink
              v-for="link in navLinks"
              :key="link.path"
              :to="link.path"
              @click="closeMobileMenu"
              class="px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200"
              :class="[
                route.path === link.path
                  ? 'bg-theme-accent-light text-theme-accent'
                  : 'text-theme-text-secondary hover:text-theme-text hover:bg-theme-bg-hover'
              ]"
            >
              {{ link.name }}
            </RouterLink>
          </nav>

          <!-- Mobile Auth Actions -->
          <div class="mt-4 pt-4 border-t border-theme-border">
            <template v-if="isAuthenticated">
              <div class="flex items-center gap-3 px-4 py-3">
                <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand-400 to-brand-600 flex items-center justify-center">
                  <span class="text-white font-medium">{{ authStore.user?.username?.charAt(0).toUpperCase() }}</span>
                </div>
                <div>
                  <div class="font-medium text-theme-text">{{ authStore.user?.username }}</div>
                  <div class="text-sm text-theme-text-muted">{{ isAdmin ? '管理员' : '用户' }}</div>
                </div>
              </div>
              <RouterLink v-if="isAdmin" to="/editor" @click="closeMobileMenu" class="btn btn-primary w-full mt-2">
                <svg class="w-4 h-4 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                写文章
              </RouterLink>
              <button @click="handleLogout" class="btn btn-secondary w-full mt-2">
                退出登录
              </button>
            </template>
            <template v-else>
              <RouterLink to="/login" @click="closeMobileMenu" class="btn btn-primary w-full">
                登录
              </RouterLink>
            </template>
          </div>
        </div>
      </Transition>
    </div>
  </header>
</template>

<style scoped>
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.3s ease;
}

.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>