<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRoute, RouterLink } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { postApi, commentApi } from '../api'
import Header from '../components/Header.vue'
import markdownit from 'markdown-it'

const route = useRoute()
const authStore = useAuthStore()
const post = ref(null)
const comments = ref([])
const loading = ref(true)
const md = markdownit()

const isAuthenticated = computed(() => authStore.isAuthenticated)

const htmlContent = computed(() => {
  if (!post.value?.content) return ''
  return md.render(post.value.content)
})

const newComment = ref('')
const submitting = ref(false)

const loadComments = async () => {
  try {
    const res = await commentApi.list(route.params.id)
    comments.value = res.data
  } catch (error) {
    console.error('Failed to fetch comments:', error)
  }
}

const handleSubmitComment = async () => {
  if (!newComment.value.trim()) return

  submitting.value = true
  try {
    await commentApi.create(route.params.id, newComment.value)
    newComment.value = ''
    await loadComments()
  } catch (error) {
    console.error('Failed to submit comment:', error)
  } finally {
    submitting.value = false
  }
}

const handleDeleteComment = async (id) => {
  if (!confirm('确定要删除这条评论吗？')) return

  try {
    await commentApi.delete(id)
    await loadComments()
  } catch (error) {
    console.error('Failed to delete comment:', error)
  }
}

const formatDate = (dateStr) => {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

const formatDateTime = (dateStr) => {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}

onMounted(async () => {
  authStore.init()
  try {
    const res = await postApi.get(route.params.id)
    post.value = res.data
    await loadComments()
  } catch (error) {
    console.error('Failed to fetch post:', error)
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="min-h-screen bg-theme-bg">
    <Header />

    <main class="pt-24 pb-16">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
        <!-- Back Link -->
        <RouterLink to="/posts" class="inline-flex items-center gap-2 text-theme-text-secondary hover:text-theme-text mb-8 transition-colors">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
          返回文章列表
        </RouterLink>

        <!-- Loading State -->
        <div v-if="loading" class="animate-pulse">
          <div class="skeleton h-10 w-3/4 mb-4"></div>
          <div class="skeleton h-4 w-48 mb-8"></div>
          <div class="card p-8">
            <div class="skeleton h-4 w-full mb-4"></div>
            <div class="skeleton h-4 w-full mb-4"></div>
            <div class="skeleton h-4 w-2/3 mb-4"></div>
          </div>
        </div>

        <!-- Article -->
        <article v-else-if="post" class="animate-fade-in">
          <!-- Article Header -->
          <header class="mb-8">
            <h1 class="text-3xl sm:text-4xl font-bold text-theme-text mb-4">{{ post.title }}</h1>
            <div class="flex flex-wrap items-center gap-4 text-sm text-theme-text-secondary">
              <div class="flex items-center gap-2">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
                <span>{{ formatDate(post.created_at) }}</span>
              </div>
              <span
                class="badge"
                :class="post.status === 'published' ? 'badge-primary' : 'badge-warning'"
              >
                {{ post.status === 'published' ? '已发布' : '草稿' }}
              </span>
            </div>
          </header>

          <!-- Article Content -->
          <div class="card p-6 sm:p-8 mb-8">
            <div class="prose-app" v-html="htmlContent"></div>
          </div>

          <!-- Comments Section -->
          <section class="card p-6 sm:p-8">
            <div class="flex items-center gap-2 mb-6">
              <svg class="w-5 h-5 text-theme-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
              <h2 class="text-xl font-semibold text-theme-text">
                评论
                <span class="text-theme-text-muted font-normal">({{ comments.length }})</span>
              </h2>
            </div>

            <!-- Comment Form -->
            <div v-if="isAuthenticated" class="mb-6">
              <textarea
                v-model="newComment"
                placeholder="写下你的评论..."
                rows="4"
                class="input-field resize-none mb-3"
              ></textarea>
              <button
                @click="handleSubmitComment"
                :disabled="submitting || !newComment.trim()"
                class="btn btn-primary"
              >
                <svg v-if="submitting" class="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                {{ submitting ? '提交中...' : '发表评论' }}
              </button>
            </div>

            <div v-else class="mb-6 p-4 rounded-lg bg-theme-bg-secondary text-center">
              <p class="text-theme-text-secondary mb-3">登录后才能发表评论</p>
              <RouterLink to="/login" class="btn btn-primary">
                登录
              </RouterLink>
            </div>

            <!-- Comments List -->
            <div class="space-y-4">
              <div
                v-for="comment in comments"
                :key="comment.id"
                class="p-4 rounded-lg bg-theme-bg-secondary"
              >
                <div class="flex items-start justify-between mb-2">
                  <div class="flex items-center gap-2">
                    <div class="w-8 h-8 rounded-full bg-gradient-to-br from-brand-400 to-brand-600 flex items-center justify-center">
                      <span class="text-white text-xs font-medium">{{ comment.author_name?.charAt(0).toUpperCase() }}</span>
                    </div>
                    <span class="font-medium text-theme-text">{{ comment.author_name }}</span>
                  </div>
                  <span class="text-xs text-theme-text-muted">{{ formatDateTime(comment.created_at) }}</span>
                </div>
                <p class="text-theme-text-secondary text-sm leading-relaxed pl-10">{{ comment.content }}</p>
              </div>

              <div v-if="comments.length === 0" class="py-8 text-center">
                <p class="text-theme-text-muted">暂无评论，快来抢沙发吧</p>
              </div>
            </div>
          </section>
        </article>

        <!-- Not Found -->
        <div v-else class="card p-12 text-center">
          <p class="text-theme-text-secondary">文章不存在或已被删除</p>
        </div>
      </div>
    </main>
  </div>
</template>