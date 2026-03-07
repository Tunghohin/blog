<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRoute, RouterLink } from 'vue-router'
import { postApi } from '../api'
import Header from '../components/Header.vue'
import markdownit from 'markdown-it'

const route = useRoute()
const post = ref(null)
const loading = ref(true)
const md = markdownit()

const htmlContent = computed(() => {
  if (!post.value?.content) return ''
  return md.render(post.value.content)
})

onMounted(async () => {
  try {
    const res = await postApi.get(route.params.id)
    post.value = res.data
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
</style>
