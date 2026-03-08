import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '../api'

export const useAuthStore = defineStore('auth', () => {
  const user = ref(null)
  const token = ref(localStorage.getItem('token'))

  const isAuthenticated = computed(() => !!token.value)
  const isAdmin = computed(() => user.value?.role === 'admin')

  async function login(username, password) {
    const res = await authApi.login(username, password)
    const { token: newToken, username: name, role } = res.data

    token.value = newToken
    const userData = { username: name, role }
    user.value = userData
    localStorage.setItem('token', newToken)
    localStorage.setItem('user', JSON.stringify(userData))

    return res.data
  }

  async function register(username, password) {
    const res = await authApi.register(username, password)
    const { token: newToken, username: name, role } = res.data

    token.value = newToken
    const userData = { username: name, role }
    user.value = userData
    localStorage.setItem('token', newToken)
    localStorage.setItem('user', JSON.stringify(userData))

    return res.data
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  // 初始化时尝试从 localStorage 恢复用户信息
  // 注意：这里简化处理，实际需要解码 JWT 获取角色信息
  function init() {
    const savedToken = localStorage.getItem('token')
    if (savedToken) {
      token.value = savedToken
      // 简单判断：如果 token 存在，尝试从存储中获取用户信息
      const savedUser = localStorage.getItem('user')
      if (savedUser) {
        user.value = JSON.parse(savedUser)
      }
    }
  }

  // 保存用户信息到 localStorage
  function setUser(userData) {
    user.value = userData
    localStorage.setItem('user', JSON.stringify(userData))
  }

  return {
    user,
    token,
    isAuthenticated,
    isAdmin,
    login,
    register,
    logout,
    init,
    setUser,
  }
})
