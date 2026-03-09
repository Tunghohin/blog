<script setup>
import { RouterView } from 'vue-router'
import { onMounted } from 'vue'
import { useThemeStore } from './stores/theme'

// Initialize theme on app mount
const themeStore = useThemeStore()

onMounted(() => {
  // Theme is automatically initialized in the store
})
</script>

<template>
  <RouterView />
</template>

<style>
/* Global transitions handled by CSS */
</style>