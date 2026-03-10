import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器 - 添加 token
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

export const postApi = {
  // 获取文章列表
  list() {
    return api.get('/posts')
  },

  // 获取文章详情
  get(id) {
    return api.get(`/posts/${id}`)
  },

  // 创建文章
  create(data) {
    return api.post('/posts', data)
  },

  // 更新文章
  update(id, data) {
    return api.put(`/posts/${id}`, data)
  },

  // 删除文章
  delete(id) {
    return api.delete(`/posts/${id}`)
  },
}

export const authApi = {
  login(username, password) {
    return api.post('/auth/login', { username, password })
  },

  register(username, password) {
    return api.post('/auth/register', { username, password })
  },
}

export const commentApi = {
  // 获取文章评论列表
  list(postId) {
    return api.get(`/posts/${postId}/comments`)
  },

  // 创建评论
  create(postId, content, parentId = null) {
    return api.post(`/posts/${postId}/comments`, { content, parent_id: parentId })
  },

  // 删除评论
  delete(id) {
    return api.delete(`/comments/${id}`)
  },
}

export const uploadApi = {
  // 上传图片
  uploadImage(file) {
    const formData = new FormData()
    formData.append('file', file)
    return api.post('/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
  },
}
