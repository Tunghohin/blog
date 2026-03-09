<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import Header from '../components/Header.vue'

const router = useRouter()
const authStore = useAuthStore()

const isLogin = ref(true)

const form = ref({
  username: '',
  password: '',
  confirmPassword: '',
})

const error = ref('')
const success = ref('')
const loading = ref(false)

const isFormValid = computed(() => {
  if (!form.value.username || !form.value.password) return false
  if (!isLogin.value && form.value.password !== form.value.confirmPassword) return false
  return true
})

const handleSubmit = async () => {
  loading.value = true
  error.value = ''
  success.value = ''

  // 注册时检查密码是否一致
  if (!isLogin.value && form.value.password !== form.value.confirmPassword) {
    error.value = '两次输入的密码不一致'
    loading.value = false
    return
  }

  try {
    if (isLogin.value) {
      // 登录
      await authStore.login(form.value.username, form.value.password)
      router.push('/')
    } else {
      // 注册
      await authStore.register(form.value.username, form.value.password)
      success.value = '注册成功！正在跳转...'
      setTimeout(() => router.push('/'), 1000)
    }
  } catch (err) {
    error.value = isLogin.value ? '登录失败，请检查用户名和密码' : '注册失败，用户名可能已存在'
    console.error(err)
  } finally {
    loading.value = false
  }
}

const toggleMode = () => {
  isLogin.value = !isLogin.value
  error.value = ''
  success.value = ''
}
</script>

<template>
  <div class="min-h-screen bg-theme-bg">
    <Header />

    <main class="pt-24 pb-16">
      <div class="container-app">
        <div class="max-w-md mx-auto">
          <!-- Auth Card -->
          <div class="card p-8 animate-scale-in">
            <!-- Header -->
            <div class="text-center mb-8">
              <div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-gradient-to-br from-brand-400 to-brand-600 flex items-center justify-center shadow-glow">
                <svg class="w-8 h-8 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
              </div>
              <h1 class="text-2xl font-bold text-theme-text">{{ isLogin ? '欢迎回来' : '创建账号' }}</h1>
              <p class="text-theme-text-secondary mt-1">{{ isLogin ? '登录您的账号' : '注册一个新账号' }}</p>
            </div>

            <!-- Form -->
            <form @submit.prevent="handleSubmit" class="space-y-4">
              <!-- Error Message -->
              <div v-if="error" class="badge-error py-3 px-4 rounded-lg text-sm">
                {{ error }}
              </div>

              <!-- Success Message -->
              <div v-if="success" class="badge-success py-3 px-4 rounded-lg text-sm">
                {{ success }}
              </div>

              <!-- Username -->
              <div>
                <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">用户名</label>
                <input
                  v-model="form.username"
                  type="text"
                  placeholder="输入用户名"
                  required
                  class="input-field"
                />
              </div>

              <!-- Password -->
              <div>
                <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">密码</label>
                <input
                  v-model="form.password"
                  type="password"
                  :placeholder="isLogin ? '输入密码' : '设置密码'"
                  required
                  class="input-field"
                />
              </div>

              <!-- Confirm Password (Register only) -->
              <div v-if="!isLogin">
                <label class="block text-sm font-medium text-theme-text-secondary mb-1.5">确认密码</label>
                <input
                  v-model="form.confirmPassword"
                  type="password"
                  placeholder="再次输入密码"
                  required
                  class="input-field"
                />
              </div>

              <!-- Submit Button -->
              <button
                type="submit"
                class="btn btn-primary w-full py-3"
                :disabled="loading || !isFormValid"
              >
                <svg v-if="loading" class="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                {{ loading ? '处理中...' : (isLogin ? '登录' : '注册') }}
              </button>
            </form>

            <!-- Toggle Mode -->
            <div class="mt-6 pt-6 border-t border-theme-border text-center">
              <span class="text-theme-text-secondary">{{ isLogin ? '还没有账号？' : '已有账号？' }}</span>
              <button @click="toggleMode" class="ml-2 text-theme-accent hover:text-theme-accent-hover font-medium transition-colors">
                {{ isLogin ? '立即注册' : '去登录' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>