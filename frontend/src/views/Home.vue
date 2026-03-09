<script setup>
import { ref, onMounted } from 'vue'
import { postApi } from '../api'
import PostCard from '../components/PostCard.vue'
import Header from '../components/Header.vue'

const posts = ref([])
const loading = ref(true)

onMounted(async () => {
  try {
    const res = await postApi.list()
    posts.value = res.data
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="min-h-screen bg-theme-bg">
    <Header />

    <main class="pt-24 pb-16">
      <div class="container-app">
        <!-- Hero Section -->
        <div class="relative mb-16">
          <!-- Background decoration -->
          <div class="absolute inset-0 -z-10 overflow-hidden">
            <div class="absolute top-0 left-1/4 w-96 h-96 bg-brand-500/10 rounded-full blur-3xl"></div>
            <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-brand-400/10 rounded-full blur-3xl"></div>
          </div>

          <div class="text-center py-12">
            <h1 class="text-4xl sm:text-5xl font-bold mb-4 animate-fade-in">
              <span class="gradient-text">探索、学习、分享</span>
            </h1>
            <p class="text-lg text-theme-text-secondary max-w-2xl mx-auto animate-slide-up">
              记录技术成长，分享编程心得，与志同道合的开发者一起进步
            </p>
          </div>
        </div>

        <!-- Posts Section -->
        <section>
          <div class="flex items-center justify-between mb-8">
            <h2 class="text-2xl font-bold text-theme-text flex items-center gap-2">
              <svg class="w-6 h-6 text-theme-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z" />
              </svg>
              最新文章
            </h2>
            <RouterLink to="/posts" class="text-theme-accent hover:text-theme-accent-hover flex items-center gap-1 text-sm font-medium transition-colors">
              查看全部
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </RouterLink>
          </div>

          <!-- Loading State -->
          <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="i in 6" :key="i" class="card p-6 animate-pulse">
              <div class="skeleton h-6 w-3/4 mb-4"></div>
              <div class="skeleton h-4 w-full mb-2"></div>
              <div class="skeleton h-4 w-2/3 mb-4"></div>
              <div class="flex justify-between">
                <div class="skeleton h-4 w-24"></div>
                <div class="skeleton h-6 w-16 rounded-full"></div>
              </div>
            </div>
          </div>

          <!-- Empty State -->
          <div v-else-if="posts.length === 0" class="card p-12 text-center">
            <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-theme-bg-secondary flex items-center justify-center">
              <svg class="w-8 h-8 text-theme-text-muted" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
            <h3 class="text-lg font-medium text-theme-text mb-2">暂无文章</h3>
            <p class="text-theme-text-muted">还没有发布任何文章，敬请期待...</p>
          </div>

          <!-- Posts Grid -->
          <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <PostCard
              v-for="(post, index) in posts"
              :key="post.id"
              :post="post"
              :style="{ animationDelay: `${index * 50}ms` }"
              class="animate-slide-up"
            />
          </div>
        </section>
      </div>
    </main>

    <!-- Footer -->
    <footer class="border-t border-theme-border py-8">
      <div class="container-app">
        <div class="flex flex-col sm:flex-row items-center justify-between gap-4">
          <div class="flex items-center gap-2">
            <div class="w-6 h-6 rounded bg-gradient-to-br from-brand-400 to-brand-600 flex items-center justify-center">
              <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
              </svg>
            </div>
            <span class="text-sm text-theme-text-muted">Tunghohin's Blog</span>
          </div>
          <p class="text-sm text-theme-text-muted">
            Made with Vue 3 + Tailwind CSS
          </p>
        </div>
      </div>
    </footer>
  </div>
</template>