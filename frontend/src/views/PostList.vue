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
        <!-- Page Header -->
        <div class="mb-8">
          <h1 class="text-3xl font-bold text-theme-text mb-2">所有文章</h1>
          <p class="text-theme-text-secondary">浏览所有已发布的文章</p>
        </div>

        <!-- Loading State -->
        <div v-if="loading" class="space-y-4">
          <div v-for="i in 5" :key="i" class="card p-6 animate-pulse">
            <div class="skeleton h-6 w-2/3 mb-4"></div>
            <div class="skeleton h-4 w-full mb-2"></div>
            <div class="skeleton h-4 w-3/4 mb-4"></div>
            <div class="flex justify-between">
              <div class="skeleton h-4 w-32"></div>
              <div class="skeleton h-6 w-20 rounded-full"></div>
            </div>
          </div>
        </div>

        <!-- Empty State -->
        <div v-else-if="posts.length === 0" class="card p-16 text-center">
          <div class="w-20 h-20 mx-auto mb-6 rounded-full bg-theme-bg-secondary flex items-center justify-center">
            <svg class="w-10 h-10 text-theme-text-muted" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </div>
          <h3 class="text-xl font-semibold text-theme-text mb-2">暂无文章</h3>
          <p class="text-theme-text-secondary mb-6">还没有发布任何文章</p>
          <RouterLink to="/" class="btn btn-primary">
            返回首页
          </RouterLink>
        </div>

        <!-- Posts List -->
        <div v-else class="space-y-4">
          <PostCard
            v-for="(post, index) in posts"
            :key="post.id"
            :post="post"
            :style="{ animationDelay: `${index * 50}ms` }"
            class="animate-slide-up"
          />
        </div>

        <!-- Posts Count -->
        <div v-if="!loading && posts.length > 0" class="mt-8 text-center text-theme-text-muted text-sm">
          共 {{ posts.length }} 篇文章
        </div>
      </div>
    </main>
  </div>
</template>