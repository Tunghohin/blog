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
  <div class="editor">
    <Header />

    <main class="main">
      <div class="container">
        <h1 class="page-title">
          {{ isEdit ? '编辑文章' : '写文章' }}
        </h1>

        <form @submit.prevent="handleSubmit" class="editor-form">
          <div v-if="error" class="error">{{ error }}</div>

          <div class="form-group">
            <label>标题</label>
            <input
              v-model="form.title"
              type="text"
              placeholder="输入文章标题"
              required
              class="input"
            />
          </div>

          <div class="form-group">
            <label>URL Slug</label>
            <input
              v-model="form.slug"
              type="text"
              placeholder="my-article-slug"
              required
              class="input"
            />
          </div>

          <div class="form-group">
            <label>摘要</label>
            <textarea
              v-model="form.summary"
              placeholder="文章摘要（可选）"
              rows="3"
              class="textarea"
            ></textarea>
          </div>

          <div class="form-group">
            <label>内容 (Markdown)</label>
            <textarea
              v-model="form.content"
              placeholder="# 标题&#10;&#10;开始写作..."
              rows="20"
              required
              class="textarea markdown-editor"
            ></textarea>
          </div>

          <div class="form-group">
            <label>状态</label>
            <select v-model="form.status" class="select">
              <option value="draft">草稿</option>
              <option value="published">发布</option>
            </select>
          </div>

          <div class="form-actions">
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
              {{ saving ? '保存中...' : '保存' }}
            </button>
          </div>
        </form>
      </div>
    </main>
  </div>
</template>

<style scoped>
.editor {
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

.page-title {
  font-size: 2rem;
  margin: 0 0 2rem 0;
  color: #fff;
}

.editor-form {
  background-color: #1e1e1e;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 2rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  color: #ccc;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.input,
.textarea,
.select {
  width: 100%;
  padding: 0.75rem;
  background-color: #2d2d2d;
  border: 1px solid #333;
  border-radius: 6px;
  color: #fff;
  font-size: 1rem;
  font-family: inherit;
  transition: border-color 0.2s;
}

.input:focus,
.textarea:focus,
.select:focus {
  outline: none;
  border-color: #3b82f6;
}

.textarea {
  resize: vertical;
  font-family: 'SF Mono', Monaco, Consolas, monospace;
  font-size: 0.9rem;
  line-height: 1.6;
}

.markdown-editor {
  min-height: 400px;
}

.select {
  cursor: pointer;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 2rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-secondary {
  background-color: #333;
  color: #fff;
}

.btn-secondary:hover {
  background-color: #444;
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

.error {
  background-color: rgba(239, 68, 68, 0.2);
  border: 1px solid #ef4444;
  color: #ef4444;
  padding: 0.75rem;
  border-radius: 6px;
  margin-bottom: 1.5rem;
}
</style>
