<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { postApi } from '../api'
import Header from '../components/Header.vue'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const isAdmin = computed(() => authStore.isAdmin)

const isEdit = computed(() => !!route.params.id)

const form = ref({
  title: '',
  slug: '',
  content: '',
  summary: '',
  status: 'draft',
})

const saving = ref(false)
const error = ref('')

onMounted(async () => {
  // 检查是否为管理员
  if (!authStore.isAuthenticated || !authStore.isAdmin) {
    error.value = '只有管理员可以发布文章'
    setTimeout(() => router.push('/'), 2000)
    return
  }

  if (isEdit.value) {
    try {
      const res = await postApi.get(route.params.id)
      form.value = {
        title: res.data.title,
        slug: res.data.slug,
        content: res.data.content,
        summary: res.data.summary || '',
        status: res.data.status,
      }
    } catch (err) {
      error.value = '加载文章失败'
      console.error(err)
    }
  }
})

const handleSubmit = async () => {
  saving.value = true
  error.value = ''

  try {
    if (isEdit.value) {
      await postApi.update(route.params.id, form.value)
    } else {
      await postApi.create(form.value)
    }
    router.push('/posts')
  } catch (err) {
    error.value = isEdit.value ? '更新失败' : '创建失败'
    console.error(err)
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <div class="min-h-screen bg-theme-bg">
    <Header />

    <main class="pt-24 pb-16">
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        <!-- Page Header -->
        <div class="flex items-center justify-between mb-8">
          <div>
            <h1 class="text-3xl font-bold text-theme-text">{{ isEdit ? '编辑文章' : '写文章' }}</h1>
            <p class="text-theme-text-secondary mt-1">{{ isEdit ? '修改现有文章内容' : '创建一篇新文章' }}</p>
          </div>
          <button @click="router.back()" class="btn btn-ghost">
            <svg class="w-5 h-5 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
            返回
          </button>
        </div>

        <!-- Error Message -->
        <div v-if="error" class="badge-error py-3 px-4 rounded-lg text-sm mb-6">
          {{ error }}
        </div>

        <!-- Editor Form -->
        <form @submit.prevent="handleSubmit" class="card p-6 sm:p-8">
          <div class="space-y-6">
            <!-- Title -->
            <div>
              <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">
                文章标题
              </label>
              <input
                v-model="form.title"
                type="text"
                placeholder="输入一个吸引人的标题"
                required
                class="input-field text-lg"
              />
            </div>

            <!-- Slug -->
            <div>
              <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">
                URL Slug
              </label>
              <input
                v-model="form.slug"
                type="text"
                placeholder="my-article-slug"
                required
                class="input-field font-mono"
              />
              <p class="text-xs text-theme-text-muted mt-1">用于 URL 中的文章标识符</p>
            </div>

            <!-- Summary -->
            <div>
              <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">
                文章摘要
              </label>
              <textarea
                v-model="form.summary"
                placeholder="简要描述这篇文章的内容..."
                rows="3"
                class="input-field resize-none"
              ></textarea>
            </div>

            <!-- Content -->
            <div>
              <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">
                正文内容
                <span class="text-theme-text-muted font-normal">(Markdown 格式)</span>
              </label>
              <textarea
                v-model="form.content"
                placeholder="# 标题&#10;&#10;开始写作..."
                rows="20"
                required
                class="input-field resize-none font-mono text-sm leading-relaxed"
              ></textarea>
            </div>

            <!-- Status -->
            <div>
              <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">
                发布状态
              </label>
              <select v-model="form.status" class="input-field">
                <option value="draft">草稿</option>
                <option value="published">发布</option>
              </select>
            </div>

            <!-- Actions -->
            <div class="flex items-center justify-end gap-3 pt-4 border-t border-theme-border">
              <button
                type="button"
                @click="router.back()"
                class="btn btn-secondary"
              >
                取消
              </button>
              <button
                type="submit"
                class="btn btn-primary"
                :disabled="saving"
              >
                <svg v-if="saving" class="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                {{ saving ? '保存中...' : '保存文章' }}
              </button>
            </div>
          </div>
        </form>
      </div>
    </main>
  </div>
</template>