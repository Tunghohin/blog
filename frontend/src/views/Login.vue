<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { authApi } from '../api'
import Header from '../components/Header.vue'

const router = useRouter()

const form = ref({
  username: '',
  password: '',
})

const error = ref('')
const loading = ref(false)

const handleSubmit = async () => {
  loading.value = true
  error.value = ''

  try {
    const res = await authApi.login(form.value.username, form.value.password)
    localStorage.setItem('token', res.data.token)
    router.push('/')
  } catch (err) {
    error.value = '登录失败，请检查用户名和密码'
    console.error(err)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login">
    <Header />

    <main class="main">
      <div class="container">
        <div class="login-card">
          <h1 class="title">登录</h1>

          <form @submit.prevent="handleSubmit" class="login-form">
            <div v-if="error" class="error">{{ error }}</div>

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
                placeholder="输入密码"
                required
                class="input"
              />
            </div>

            <button
              type="submit"
              class="btn btn-primary btn-full"
              :disabled="loading"
            >
              {{ loading ? '登录中...' : '登录' }}
            </button>
          </form>
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

.login-card {
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

.login-form {
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
</style>
