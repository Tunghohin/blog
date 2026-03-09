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
  <RouterLink :to="`/posts/${post.id}`" class="card card-hover block group p-6">
    <article>
      <!-- Title -->
      <h2 class="text-lg font-semibold text-theme-text mb-3 group-hover:text-theme-accent transition-colors line-clamp-2">
        {{ post.title }}
      </h2>

      <!-- Summary -->
      <p class="text-theme-text-secondary text-sm leading-relaxed mb-4 line-clamp-3">
        {{ post.summary || '暂无摘要' }}
      </p>

      <!-- Footer -->
      <div class="flex items-center justify-between">
        <!-- Date -->
        <div class="flex items-center gap-2 text-theme-text-muted text-sm">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
          <span>{{ formatDate(post.created_at) }}</span>
        </div>

        <!-- Status Badge -->
        <span
          class="badge"
          :class="post.status === 'published' ? 'badge-primary' : 'badge-warning'"
        >
          {{ post.status === 'published' ? '已发布' : '草稿' }}
        </span>
      </div>
    </article>
  </RouterLink>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>