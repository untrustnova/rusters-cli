<template>
  <div class="app">
    <header class="app-header">
      <h1>{{ PROJECT_NAME }}</h1>
      <p class="subtitle">Powered by <span class="brand">Rusters</span> + Vue 3</p>
    </header>

    <main class="app-main">
      <section class="card">
        <h2>Backend Status</h2>
        <p v-if="loading">Connecting to Rusters backend…</p>
        <p v-else-if="error" class="error">{{ error }}</p>
        <pre v-else class="status-json">{{ JSON.stringify(backendInfo, null, 2) }}</pre>
      </section>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const backendInfo = ref<Record<string, unknown> | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    const res = await fetch('/api/_rusters/info')
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    backendInfo.value = await res.json()
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
})
</script>
