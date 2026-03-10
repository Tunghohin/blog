<script setup>
import { ref, onMounted, watch, computed, shallowRef } from 'vue'
import { EditorView, keymap, lineNumbers, highlightActiveLine, highlightActiveLineGutter } from '@codemirror/view'
import { EditorState } from '@codemirror/state'
import { markdown, markdownLanguage } from '@codemirror/lang-markdown'
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands'
import { useThemeStore } from '../stores/theme'
import { uploadApi } from '../api'
import markdownit from 'markdown-it'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  },
  placeholder: {
    type: String,
    default: '开始写作...'
  }
})

const emit = defineEmits(['update:modelValue'])

const themeStore = useThemeStore()
const isDark = computed(() => themeStore.currentTheme === 'dark')

// DOM refs
const editorContainer = ref(null)
const previewContainer = ref(null)

// Editor instance
const editorView = shallowRef(null)

// Markdown parser
const md = markdownit({
  html: true,
  linkify: true,
  typographer: true
})

// Computed preview HTML
const previewHtml = computed(() => {
  if (!props.modelValue) return ''
  return md.render(props.modelValue)
})

// Upload state
const isUploading = ref(false)
const uploadError = ref('')

// Create editor theme
const createEditorTheme = () => {
  const bgColor = isDark.value ? '#1e293b' : '#ffffff'
  const textColor = isDark.value ? '#f1f5f9' : '#1e293b'
  const borderColor = isDark.value ? '#334155' : '#e2e8f0'
  const selectionBg = isDark.value ? 'rgba(129, 140, 248, 0.3)' : 'rgba(99, 102, 241, 0.2)'
  const cursorColor = isDark.value ? '#818cf8' : '#6366f1'
  const gutterBg = isDark.value ? '#0f172a' : '#f8fafc'
  const gutterText = isDark.value ? '#64748b' : '#94a3b8'
  const activeLineBg = isDark.value ? 'rgba(255,255,255,0.05)' : 'rgba(0,0,0,0.03)'

  return EditorView.theme({
    '&': {
      backgroundColor: bgColor,
      color: textColor,
      height: '100%'
    },
    '.cm-content': {
      caretColor: cursorColor,
      fontFamily: "'JetBrains Mono', 'SF Mono', Monaco, Consolas, monospace",
      fontSize: '14px',
      lineHeight: '1.6',
      padding: '16px 0'
    },
    '.cm-cursor': {
      borderLeftColor: cursorColor
    },
    '.cm-selectionBackground, ::selection': {
      backgroundColor: selectionBg
    },
    '.cm-focused .cm-selectionBackground': {
      backgroundColor: selectionBg
    },
    '.cm-gutters': {
      backgroundColor: gutterBg,
      color: gutterText,
      border: 'none',
      borderRight: `1px solid ${borderColor}`
    },
    '.cm-activeLineGutter': {
      backgroundColor: activeLineBg
    },
    '.cm-activeLine': {
      backgroundColor: activeLineBg
    },
    '.cm-scroller': {
      overflow: 'auto'
    }
  })
}

// Create editor state
const createState = (doc) => {
  return EditorState.create({
    doc,
    extensions: [
      lineNumbers(),
      highlightActiveLine(),
      highlightActiveLineGutter(),
      history(),
      markdown({ base: markdownLanguage }),
      keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
        indentWithTab,
        // Custom keybindings
        {
          key: 'Mod-b',
          run: () => wrapSelection('**', '**')
        },
        {
          key: 'Mod-i',
          run: () => wrapSelection('*', '*')
        },
        {
          key: 'Mod-k',
          run: insertLink
        }
      ]),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          emit('update:modelValue', update.state.doc.toString())
        }
      }),
      createEditorTheme(),
      EditorView.lineWrapping
    ]
  })
}

// Helper: Wrap selection with text
const wrapSelection = (before, after) => {
  if (!editorView.value) return false
  const { state } = editorView.value
  const { from, to } = state.selection.main
  if (from === to) return false

  const transaction = state.update({
    changes: [
      { from, insert: before },
      { from: to, insert: after }
    ]
  })
  editorView.value.dispatch(transaction)
  return true
}

// Helper: Insert link
const insertLink = () => {
  if (!editorView.value) return false
  const { state } = editorView.value
  const { from, to } = state.selection.main
  const selectedText = state.sliceDoc(from, to)
  const linkText = selectedText || '链接文字'
  const markdown = `[${linkText}](url)`

  const transaction = state.update({
    changes: { from, to, insert: markdown }
  })
  editorView.value.dispatch(transaction)
  return true
}

// Initialize editor
onMounted(() => {
  if (editorContainer.value) {
    const state = createState(props.modelValue)
    editorView.value = new EditorView({
      state,
      parent: editorContainer.value
    })
  }
})

// Watch for external changes to modelValue
watch(() => props.modelValue, (newValue) => {
  if (editorView.value && editorView.value.state.doc.toString() !== newValue) {
    editorView.value.dispatch({
      changes: {
        from: 0,
        to: editorView.value.state.doc.length,
        insert: newValue
      }
    })
  }
})

// Watch for theme changes
watch(isDark, () => {
  if (editorView.value) {
    editorView.value.dispatch({
      effects: EditorView.theme.reconfigure(createEditorTheme())
    })
  }
})

// Toolbar actions
const insertHeading = (level) => {
  if (!editorView.value) return
  const { state } = editorView.value
  const { from } = state.selection.main
  const prefix = '#'.repeat(level) + ' '

  editorView.value.dispatch(state.update({
    changes: { from, insert: prefix }
  }))
  editorView.value.focus()
}

const insertBold = () => {
  wrapSelection('**', '**')
  editorView.value?.focus()
}

const insertItalic = () => {
  wrapSelection('*', '*')
  editorView.value?.focus()
}

const insertCode = () => {
  wrapSelection('`', '`')
  editorView.value?.focus()
}

const insertCodeBlock = () => {
  if (!editorView.value) return
  const { state } = editorView.value
  const { from, to } = state.selection.main
  const code = state.sliceDoc(from, to)
  const block = '```\n' + code + '\n```'

  editorView.value.dispatch(state.update({
    changes: { from, to, insert: block }
  }))
  editorView.value.focus()
}

// Image upload
const handleImageUpload = async (file) => {
  if (!file) return

  // Validate file type
  const allowedTypes = ['image/jpeg', 'image/png', 'image/gif', 'image/webp']
  if (!allowedTypes.includes(file.type)) {
    uploadError.value = '不支持的图片格式'
    return
  }

  // Validate file size (5MB)
  if (file.size > 5 * 1024 * 1024) {
    uploadError.value = '图片大小不能超过 5MB'
    return
  }

  isUploading.value = true
  uploadError.value = ''

  try {
    const res = await uploadApi.uploadImage(file)
    const imageUrl = res.data.url

    // Insert image markdown
    if (editorView.value) {
      const { state } = editorView.value
      const { from } = state.selection.main
      const imageMarkdown = `![${file.name}](${imageUrl})`

      editorView.value.dispatch(state.update({
        changes: { from, insert: imageMarkdown }
      }))
      editorView.value.focus()
    }
  } catch (err) {
    uploadError.value = '上传失败，请重试'
    console.error(err)
  } finally {
    isUploading.value = false
  }
}

// Drag and drop
const isDragging = ref(false)

const handleDragOver = (e) => {
  e.preventDefault()
  isDragging.value = true
}

const handleDragLeave = (e) => {
  e.preventDefault()
  isDragging.value = false
}

const handleDrop = (e) => {
  e.preventDefault()
  isDragging.value = false

  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    handleImageUpload(files[0])
  }
}

// Paste image
const handlePaste = (e) => {
  const items = e.clipboardData?.items
  if (!items) return

  for (const item of items) {
    if (item.type.startsWith('image/')) {
      const file = item.getAsFile()
      if (file) {
        handleImageUpload(file)
      }
      break
    }
  }
}

// File input
const fileInput = ref(null)

const triggerFileInput = () => {
  fileInput.value?.click()
}

const handleFileSelect = (e) => {
  const file = e.target.files?.[0]
  if (file) {
    handleImageUpload(file)
  }
  e.target.value = ''
}

// Scroll sync
const handleEditorScroll = () => {
  if (!editorView.value || !previewContainer.value) return
  const scrollTop = editorView.value.scrollDOM.scrollTop
  const scrollHeight = editorView.value.scrollDOM.scrollHeight
  const clientHeight = editorView.value.scrollDOM.clientHeight

  const scrollPercent = scrollTop / (scrollHeight - clientHeight)

  const previewScrollHeight = previewContainer.value.scrollHeight
  const previewClientHeight = previewContainer.value.clientHeight

  previewContainer.value.scrollTop = scrollPercent * (previewScrollHeight - previewClientHeight)
}
</script>

<template>
  <div class="markdown-editor">
    <!-- Toolbar -->
    <div class="toolbar border-b border-theme-border px-4 py-2 flex items-center gap-1 flex-wrap">
      <!-- Headings -->
      <div class="flex items-center gap-1 pr-2 border-r border-theme-border">
        <button @click="insertHeading(1)" class="toolbar-btn" title="一级标题">H1</button>
        <button @click="insertHeading(2)" class="toolbar-btn" title="二级标题">H2</button>
        <button @click="insertHeading(3)" class="toolbar-btn" title="三级标题">H3</button>
      </div>

      <!-- Format -->
      <div class="flex items-center gap-1 pr-2 border-r border-theme-border">
        <button @click="insertBold" class="toolbar-btn" title="粗体 (Ctrl+B)">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 4h8a4 4 0 014 4 4 4 0 01-4 4H6z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 12h9a4 4 0 014 4 4 4 0 01-4 4H6z"/>
          </svg>
        </button>
        <button @click="insertItalic" class="toolbar-btn" title="斜体 (Ctrl+I)">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 4h4l-2 16h-4l2-16z"/>
          </svg>
        </button>
        <button @click="insertCode" class="toolbar-btn" title="行内代码">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/>
          </svg>
        </button>
        <button @click="insertCodeBlock" class="toolbar-btn" title="代码块">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h10M4 18h6"/>
          </svg>
        </button>
      </div>

      <!-- Link & Image -->
      <div class="flex items-center gap-1">
        <button @click="insertLink" class="toolbar-btn" title="链接 (Ctrl+K)">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/>
          </svg>
        </button>
        <button @click="triggerFileInput" class="toolbar-btn" title="上传图片" :disabled="isUploading">
          <svg v-if="isUploading" class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
          </svg>
          <svg v-else class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
          </svg>
        </button>
        <input ref="fileInput" type="file" accept="image/*" class="hidden" @change="handleFileSelect" />
      </div>

      <!-- Upload error -->
      <div v-if="uploadError" class="ml-auto text-sm text-theme-error">
        {{ uploadError }}
      </div>
    </div>

    <!-- Editor & Preview -->
    <div class="editor-body flex-1 flex min-h-0">
      <!-- Editor -->
      <div
        ref="editorContainer"
        class="editor-pane flex-1 overflow-hidden border-r border-theme-border relative"
        :class="{ 'drag-over': isDragging }"
        @dragover="handleDragOver"
        @dragleave="handleDragLeave"
        @drop="handleDrop"
        @paste="handlePaste"
      >
        <!-- Drag overlay -->
        <div v-if="isDragging" class="drag-overlay">
          <div class="drag-content">
            <svg class="w-12 h-12 text-theme-accent mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"/>
            </svg>
            <span class="text-theme-text">拖放图片到这里上传</span>
          </div>
        </div>
      </div>

      <!-- Preview -->
      <div
        ref="previewContainer"
        class="preview-pane flex-1 overflow-auto p-4 bg-theme-bg-secondary"
      >
        <div v-if="previewHtml" class="prose-app" v-html="previewHtml"></div>
        <div v-else class="h-full flex items-center justify-center text-theme-text-muted">
          <span>预览区域</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.markdown-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  overflow: hidden;
}

.toolbar-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 0.375rem;
  color: var(--color-text-secondary);
  transition: all 0.15s ease;
}

.toolbar-btn:hover:not(:disabled) {
  background-color: var(--color-bg-hover);
  color: var(--color-text);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.editor-pane {
  position: relative;
}

.editor-pane.drag-over {
  opacity: 0.5;
}

.drag-overlay {
  position: absolute;
  inset: 0;
  background-color: var(--color-bg);
  opacity: 0.95;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

.drag-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  pointer-events: none;
}

/* CodeMirror container */
:deep(.cm-editor) {
  height: 100%;
}

:deep(.cm-scroller) {
  padding: 0 16px;
}
</style>