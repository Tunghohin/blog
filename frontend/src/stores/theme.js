import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  const themes = {
    light: {
      name: 'light',
      colors: {
        bg: '#ffffff',
        bgSecondary: '#f8fafc',
        bgCard: '#ffffff',
        bgHover: '#f1f5f9',
        text: '#1e293b',
        textSecondary: '#64748b',
        textMuted: '#94a3b8',
        border: '#e2e8f0',
        accent: '#6366f1',
        accentHover: '#4f46e5',
        accentLight: 'rgba(99, 102, 241, 0.1)',
        success: '#22c55e',
        error: '#ef4444',
        warning: '#f59e0b',
      },
    },
    dark: {
      name: 'dark',
      colors: {
        bg: '#0f172a',
        bgSecondary: '#1e293b',
        bgCard: '#1e293b',
        bgHover: '#334155',
        text: '#f1f5f9',
        textSecondary: '#cbd5e1',
        textMuted: '#64748b',
        border: '#334155',
        accent: '#818cf8',
        accentHover: '#a5b4fc',
        accentLight: 'rgba(129, 140, 248, 0.15)',
        success: '#4ade80',
        error: '#f87171',
        warning: '#fbbf24',
      },
    },
  }

  // 初始化主题
  const savedTheme = localStorage.getItem('theme')
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  const initialTheme = savedTheme || (prefersDark ? 'dark' : 'light')

  const currentTheme = ref(initialTheme)
  const colors = ref(themes[initialTheme].colors)

  // 切换主题
  function toggleTheme() {
    currentTheme.value = currentTheme.value === 'light' ? 'dark' : 'light'
  }

  // 设置主题
  function setTheme(themeName) {
    if (themes[themeName]) {
      currentTheme.value = themeName
    }
  }

  // 监听主题变化
  watch(currentTheme, (newTheme) => {
    colors.value = themes[newTheme].colors
    localStorage.setItem('theme', newTheme)

    // 更新 HTML class
    const html = document.documentElement
    html.classList.remove('light', 'dark')
    html.classList.add(newTheme)

    // 更新 CSS 变量
    updateCSSVariables(themes[newTheme].colors)
  }, { immediate: true })

  // 更新 CSS 变量
  function updateCSSVariables(themeColors) {
    const root = document.documentElement
    root.style.setProperty('--color-bg', themeColors.bg)
    root.style.setProperty('--color-bg-secondary', themeColors.bgSecondary)
    root.style.setProperty('--color-bg-card', themeColors.bgCard)
    root.style.setProperty('--color-bg-hover', themeColors.bgHover)
    root.style.setProperty('--color-text', themeColors.text)
    root.style.setProperty('--color-text-secondary', themeColors.textSecondary)
    root.style.setProperty('--color-text-muted', themeColors.textMuted)
    root.style.setProperty('--color-border', themeColors.border)
    root.style.setProperty('--color-accent', themeColors.accent)
    root.style.setProperty('--color-accent-hover', themeColors.accentHover)
    root.style.setProperty('--color-accent-light', themeColors.accentLight)
    root.style.setProperty('--color-success', themeColors.success)
    root.style.setProperty('--color-error', themeColors.error)
    root.style.setProperty('--color-warning', themeColors.warning)
  }

  // 初始化时更新 CSS 变量
  updateCSSVariables(colors.value)
  document.documentElement.classList.add(currentTheme.value)

  return {
    currentTheme,
    colors,
    themes,
    toggleTheme,
    setTheme,
  }
})