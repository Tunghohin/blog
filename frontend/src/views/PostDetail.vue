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
  <div class="post-detail">
    <Header />

    <main class="main">
      <div class="container">
        <RouterLink to="/posts" class="back-link">
          &larr; 返回
        </RouterLink>

        <div v-if="loading" class="loading">加载中...</div>

        <article v-else-if="post" class="article">
          <h1 class="article-title">{{ post.title }}</h1>

          <div class="article-meta">
            <span class="date">
              {{ new Date(post.created_at).toLocaleDateString('zh-CN') }}
            </span>
            <span class="status">{{ post.status === 'published' ? '已发布' : '草稿' }}</span>
          </div>

          <div class="article-content markdown-body" v-html="htmlContent"></div>
        </article>

        <!-- 评论区 -->
        <div v-if="post" class="comments-section">
          <h2 class="comments-title">评论 ({{ comments.length }})</h2>

          <!-- 发表评论 -->
          <div v-if="isAuthenticated" class="comment-form">
            <textarea
              v-model="newComment"
              placeholder="写下你的评论..."
              rows="4"
              class="comment-input"
            ></textarea>
            <button
              @click="handleSubmitComment"
              :disabled="submitting || !newComment.trim()"
              class="btn btn-primary"
            >
              {{ submitting ? '提交中...' : '发表评论' }}
            </button>
          </div>

          <div v-else class="login提示">
            <RouterLink to="/login" class="btn btn-primary">登录</RouterLink>
            <p>需要登录后才能发表评论</p>
          </div>

          <!-- 评论列表 -->
          <div class="comments-list">
            <div
              v-for="comment in comments"
              :key="comment.id"
              class="comment-item"
            >
              <div class="comment-header">
                <span class="comment-author">{{ comment.author_name }}</span>
                <span class="comment-date">
                  {{ new Date(comment.created_at).toLocaleString('zh-CN', { year: 'numeric', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', second: '2-digit' }) }}
                </span>
              </div>
              <div class="comment-content">{{ comment.content }}</div>
            </div>
            <div v-if="comments.length === 0" class="no-comments">
              暂无评论，快来抢沙发吧
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.post-detail {
  min-height: 100vh;
  background-color: #151515;
}

.main {
  padding-top: 80px;
}

.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
}

.back-link {
  display: inline-block;
  color: #888;
  text-decoration: none;
  margin-bottom: 1.5rem;
  transition: color 0.2s;
}

.back-link:hover {
  color: #3b82f6;
}

.loading {
  text-align: center;
  color: #888;
  padding: 4rem 0;
}

.article {
  background-color: #1e1e1e;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 2rem;
}

.article-title {
  font-size: 2rem;
  margin: 0 0 1rem 0;
  color: #fff;
}

.article-meta {
  display: flex;
  gap: 1rem;
  color: #888;
  font-size: 0.9rem;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid #333;
}

.article-content {
  color: #ccc;
  line-height: 1.8;
}

.article-content :deep(h1),
.article-content :deep(h2),
.article-content :deep(h3),
.article-content :deep(h4),
.article-content :deep(h5),
.article-content :deep(h6) {
  color: #fff;
  margin-top: 2em;
  margin-bottom: 0.5em;
}

.article-content :deep(h1):first-child {
  margin-top: 0;
}

.article-content :deep(p) {
  margin: 1em 0;
}

.article-content :deep(a) {
  color: #3b82f6;
  text-decoration: none;
}

.article-content :deep(a):hover {
  text-decoration: underline;
}

.article-content :deep(code) {
  background-color: #2d2d2d;
  padding: 0.2em 0.4em;
  border-radius: 4px;
  font-family: 'SF Mono', Monaco, Consolas, monospace;
  font-size: 0.9em;
}

.article-content :deep(pre) {
  background-color: #2d2d2d;
  padding: 1rem;
  border-radius: 8px;
  overflow-x: auto;
  margin: 1.5em 0;
}

.article-content :deep(pre code) {
  background: none;
  padding: 0;
}

.article-content :deep(ul),
.article-content :deep(ol) {
  padding-left: 1.5em;
}

.article-content :deep(blockquote) {
  border-left: 4px solid #3b82f6;
  margin: 1.5em 0;
  padding-left: 1em;
  color: #888;
}

.article-content :deep(img) {
  max-width: 100%;
  border-radius: 8px;
}

.comments-section {
  margin-top: 2rem;
  background-color: #1e1e1e;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 2rem;
}

.comments-title {
  font-size: 1.5rem;
  margin: 0 0 1.5rem 0;
  color: #fff;
  padding-bottom: 1rem;
  border-bottom: 1px solid #333;
}

.comment-form {
  margin-bottom: 2rem;
}

.comment-input {
  width: 100%;
  padding: 0.75rem;
  background-color: #2d2d2d;
  border: 1px solid #333;
  border-radius: 6px;
  color: #fff;
  font-size: 1rem;
  font-family: inherit;
  resize: vertical;
  margin-bottom: 1rem;
}

.comment-input:focus {
  outline: none;
  border-color: #3b82f6;
}

.login-tip {
  color: #888;
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem 0;
}

.comments-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.comment-item {
  background-color: #2d2d2d;
  border-radius: 6px;
  padding: 1rem;
}

.comment-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.comment-author {
  color: #fff;
  font-weight: 500;
}

.comment-date {
  color: #888;
  font-size: 0.85rem;
}

.comment-content {
  color: #ccc;
  line-height: 1.6;
  word-break: break-word;
}

.no-comments {
  text-align: center;
  color: #888;
  padding: 2rem 0;
}

.btn {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background-color: #3b82f6;
  color: #fff;
}

.btn-primary:hover {
  background-color: #2563eb;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
