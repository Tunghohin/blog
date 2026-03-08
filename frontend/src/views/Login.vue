<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { authApi } from '../api'
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

  try {
    if (isLogin.value) {
      // 登录
      const res = await authApi.login(form.value.username, form.value.password)
      authStore.setUser({
        username: res.data.username,
        role: res.data.role,
      })
      router.push('/')
    } else {
      // 注册
      if (form.value.password !== form.value.confirmPassword) {
        error.value = '两次输入的密码不一致'
        return
      }
      const res = await authApi.register(form.value.username, form.value.password)
      authStore.setUser({
        username: res.data.username,
        role: res.data.role,
      })
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
  <div class="login">
    <Header />

    <main class="main">
      <div class="container">
        <div class="auth-card">
          <h1 class="title">{{ isLogin ? '登录' : '注册' }}</h1>

          <form @submit.prevent="handleSubmit" class="auth-form">
            <div v-if="error" class="error">{{ error }}</div>
            <div v-if="success" class="success">{{ success }}</div>

            <div class="form-group">
              <label>用户名</label>
              <input
                v-model="form.username"
                type="text"
                placeholder="输入用户名"
                required
                class="input"
              />
            </div>

            <div class="form-group">
              <label>密码</label>
              <input
                v-model="form.password"
                type="password"
                :placeholder="isLogin ? '输入密码' : '设置密码'"
                required
                class="input"
              />
            </div>

            <div v-if="!isLogin" class="form-group">
              <label>确认密码</label>
              <input
                v-model="form.confirmPassword"
                type="password"
                placeholder="再次输入密码"
                required
                class="input"
              />
            </div>

            <button
              type="submit"
              class="btn btn-primary btn-full"
              :disabled="loading || !isFormValid"
            >
              {{ loading ? '处理中...' : (isLogin ? '登录' : '注册') }}
            </button>
          </form>

          <div class="toggle-hint">
            <span>{{ isLogin ? '还没有账号？' : '已有账号？' }}</span>
            <button @click="toggleMode" class="toggle-btn">
              {{ isLogin ? '立即注册' : '去登录' }}
            </button>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.login {
  min-height: 100vh;
  background-color: #151515;
}

.main {
  padding-top: 80px;
}

.container {
  max-width: 400px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
}

.auth-card {
  background-color: #1e1e1e;
  border: 1px solid #333;
  border-radius: 8px;
  padding: 2rem;
}

.title {
  font-size: 1.5rem;
  margin: 0 0 1.5rem 0;
  color: #fff;
  text-align: center;
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  color: #ccc;
  font-size: 0.9rem;
}

.input {
  padding: 0.75rem;
  background-color: #2d2d2d;
  border: 1px solid #333;
  border-radius: 6px;
  color: #fff;
  font-size: 1rem;
  transition: border-color 0.2s;
}

.input:focus {
  outline: none;
  border-color: #3b82f6;
}

.btn {
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-size: 1rem;
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

.btn-full {
  width: 100%;
  margin-top: 0.5rem;
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
}

.success {
  background-color: rgba(34, 197, 94, 0.2);
  border: 1px solid #22c55e;
  color: #22c55e;
  padding: 0.75rem;
  border-radius: 6px;
}

.toggle-hint {
  text-align: center;
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid #333;
  color: #888;
  font-size: 0.9rem;
}

.toggle-btn {
  background: none;
  border: none;
  color: #3b82f6;
  cursor: pointer;
  font-size: 0.9rem;
  margin-left: 0.5rem;
  transition: color 0.2s;
}

.toggle-btn:hover {
  color: #2563eb;
  text-decoration: underline;
}
</style>
