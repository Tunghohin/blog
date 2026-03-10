<script setup>
import { ref } from 'vue'
import { useAuthStore } from '../stores/auth'
import { commentApi } from '../api'

const props = defineProps({
  comment: {
    type: Object,
    required: true
  },
  depth: {
    type: Number,
    default: 0
  }
})

const emit = defineEmits(['refresh'])
const authStore = useAuthStore()

const showReplyForm = ref(false)
const replyContent = ref('')
const submitting = ref(false)

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

const handleReply = async () => {
  if (!replyContent.value.trim()) return

  submitting.value = true
  try {
    await commentApi.create(props.comment.post_id, replyContent.value, props.comment.id)
    replyContent.value = ''
    showReplyForm.value = false
    emit('refresh')
  } catch (error) {
    console.error('Failed to submit reply:', error)
  } finally {
    submitting.value = false
  }
}

const handleDelete = async () => {
  if (!confirm('确定要删除这条评论吗？')) return

  try {
    await commentApi.delete(props.comment.id)
    emit('refresh')
  } catch (error) {
    console.error('Failed to delete comment:', error)
  }
}

const canDelete = () => {
  return authStore.user?.id === props.comment.author_id || authStore.user?.role === 'admin'
}
</script>

<template>
  <div class="comment-item">
    <div class="flex items-start gap-3">
      <!-- Avatar -->
      <div class="w-8 h-8 rounded-full bg-gradient-to-br from-brand-400 to-brand-600 flex items-center justify-center flex-shrink-0">
        <span class="text-white text-xs font-medium">{{ comment.author_name?.charAt(0).toUpperCase() }}</span>
      </div>

      <div class="flex-1 min-w-0">
        <!-- Header -->
        <div class="flex items-center gap-2 mb-1">
          <span class="font-medium text-theme-text text-sm">{{ comment.author_name }}</span>
          <span class="text-xs text-theme-text-muted">{{ formatDateTime(comment.created_at) }}</span>
        </div>

        <!-- Content -->
        <p class="text-theme-text-secondary text-sm leading-relaxed mb-2">{{ comment.content }}</p>

        <!-- Actions -->
        <div class="flex items-center gap-3 text-xs">
          <button
            v-if="authStore.isAuthenticated"
            @click="showReplyForm = !showReplyForm"
            class="text-theme-text-muted hover:text-theme-accent transition-colors"
          >
            回复
          </button>
          <button
            v-if="canDelete()"
            @click="handleDelete"
            class="text-theme-text-muted hover:text-red-500 transition-colors"
          >
            删除
          </button>
        </div>

        <!-- Reply Form -->
        <div v-if="showReplyForm" class="mt-3">
          <textarea
            v-model="replyContent"
            placeholder="写下你的回复..."
            rows="3"
            class="input-field resize-none text-sm"
          ></textarea>
          <div class="flex gap-2 mt-2">
            <button
              @click="handleReply"
              :disabled="submitting || !replyContent.trim()"
              class="btn btn-primary text-xs py-1 px-3"
            >
              {{ submitting ? '提交中...' : '回复' }}
            </button>
            <button
              @click="showReplyForm = false"
              class="btn btn-secondary text-xs py-1 px-3"
            >
              取消
            </button>
          </div>
        </div>

        <!-- Nested Replies -->
        <div
          v-if="comment.replies && comment.replies.length > 0"
          class="mt-3 pl-4 border-l-2 border-theme-border"
        >
          <CommentItem
            v-for="reply in comment.replies"
            :key="reply.id"
            :comment="reply"
            :depth="depth + 1"
            @refresh="$emit('refresh')"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.comment-item {
  @apply py-3;
}
</style>