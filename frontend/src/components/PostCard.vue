<script setup>
import { defineProps } from 'vue'
import { RouterLink } from 'vue-router'

const props = defineProps({
  post: {
    type: Object,
    required: true,
  },
})

const formatDate = (dateStr) => {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}
</script>

<template>
  <RouterLink :to="`/posts/${post.id}`" class="post-card">
    <article>
      <h2 class="title">{{ post.title }}</h2>
      <p class="summary">{{ post.summary || '暂无摘要' }}</p>
      <div class="meta">
        <span class="date">{{ formatDate(post.created_at) }}</span>
        <span class="status" :class="post.status">
          {{ post.status === 'published' ? '已发布' : '草稿' }}
        </span>
      </div>
    </article>
  </RouterLink>
</template>

<style scoped>
.post-card {
  display: block;
  background-color: #1e1e1e;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 1.5rem;
  text-decoration: none;
  color: inherit;
  transition: all 0.2s;
}

.post-card:hover {
  border-color: #454545;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.title {
  font-size: 1.25rem;
  margin: 0 0 0.75rem 0;
  color: #fff;
  transition: color 0.2s;
}

.post-card:hover .title {
  color: #3b82f6;
}

.summary {
  color: #ccc;
  font-size: 0.9rem;
  line-height: 1.6;
  margin: 0 0 1rem 0;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.meta {
  display: flex;
  gap: 1rem;
  align-items: center;
  font-size: 0.85rem;
}

.date {
  color: #888;
}

.status {
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
}

.status.published {
  background-color: rgba(59, 130, 246, 0.2);
  color: #3b82f6;
}

.status.draft {
  background-color: rgba(107, 114, 128, 0.2);
  color: #888;
}
</style>
