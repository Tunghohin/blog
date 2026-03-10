<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { postApi } from '../api'
import Header from '../components/Header.vue'
import MarkdownEditor from '../components/MarkdownEditor.vue'

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
  <div class="min-h-screen bg-theme-bg flex flex-col">
    <Header />

    <main class="flex-1 pt-16 flex flex-col">
      <div class="flex-1 flex flex-col max-w-7xl mx-auto w-full px-4 sm:px-6 lg:px-8 py-6">
        <!-- Page Header -->
        <div class="flex items-center justify-between mb-6">
          <div>
            <h1 class="text-2xl font-bold text-theme-text">{{ isEdit ? '编辑文章' : '写文章' }}</h1>
            <p class="text-theme-text-secondary text-sm mt-1">{{ isEdit ? '修改现有文章内容' : '创建一篇新文章' }}</p>
          </div>
          <div class="flex items-center gap-3">
            <button @click="router.back()" class="btn btn-ghost">
              <svg class="w-5 h-5 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
              返回
            </button>
            <select v-model="form.status" class="input-field w-auto">
              <option value="draft">保存为草稿</option>
              <option value="published">发布文章</option>
            </select>
            <button
              @click="handleSubmit"
              class="btn btn-primary"
              :disabled="saving || !form.title || !form.slug || !form.content"
            >
              <svg v-if="saving" class="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              {{ saving ? '保存中...' : '保存文章' }}
            </button>
          </div>
        </div>

        <!-- Error Message -->
        <div v-if="error" class="badge-error py-3 px-4 rounded-lg text-sm mb-4">
          {{ error }}
        </div>

        <!-- Meta Fields -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
          <div>
            <input
              v-model="form.title"
              type="text"
              placeholder="文章标题"
              required
              class="input-field text-lg font-medium"
            />
          </div>
          <div class="flex gap-4">
            <input
              v-model="form.slug"
              type="text"
              placeholder="url-slug"
              required
              class="input-field font-mono flex-1"
            />
          </div>
        </div>

        <!-- Summary -->
        <div class="mb-4">
          <input
            v-model="form.summary"
            type="text"
            placeholder="文章摘要（可选）"
            class="input-field"
          />
        </div>

        <!-- Markdown Editor -->
        <div class="flex-1 min-h-[500px]">
          <MarkdownEditor v-model="form.content" />
        </div>
      </div>
    </main>
  </div>
</template>