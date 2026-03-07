<script setup>
import { ref, onMounted } from 'vue'
import { postApi } from '../api'
import PostCard from '../components/PostCard.vue'
import Header from '../components/Header.vue'

const posts = ref([])
const loading = ref(true)

onMounted(async () => {
  try {
    const res = await postApi.list()
    posts.value = res.data
  } catch (error) {
    console.error('Failed to fetch posts:', error)
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="post-list">
    <Header />

    <main class="main">
      <div class="container">
        <h1 class="page-title">所有文章</h1>

        <div v-if="loading" class="loading">加载中...</div>

        <div v-else-if="posts.length === 0" class="empty">
          暂无文章
        </div>

        <div v-else class="posts-list">
          <PostCard
            v-for="post in posts"
            :key="post.id"
            :post="post"
          />
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.post-list {
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

.loading,
.empty {
  text-align: center;
  color: #888;
  padding: 4rem 0;
}

.posts-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
