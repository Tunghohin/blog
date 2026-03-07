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
  <div class="home">
    <Header />

    <main class="main">
      <div class="container">
        <h1 class="page-title">最新文章</h1>

        <div v-if="loading" class="loading">加载中...</div>

        <div v-else-if="posts.length === 0" class="empty">
          暂无文章
        </div>

        <div v-else class="posts-grid">
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
.home {
  min-height: 100vh;
  background-color: #151515;
}

.main {
  padding-top: 80px;
}

.container {
  max-width: 1200px;
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

.posts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1.5rem;
}

@media (max-width: 768px) {
  .posts-grid {
    grid-template-columns: 1fr;
  }
}
</style>
