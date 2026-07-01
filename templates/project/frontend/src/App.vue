<template>
  <div class="min-height-screen bg-[#0f0f13] text-[#e8e8f0] font-sans flex flex-col min-h-screen">
    <!-- Header -->
    <header class="text-center py-16 px-8 border-b border-[#2a2a3a] bg-gradient-to-b from-[#7c6df0]/10 to-transparent">
      <h1 class="text-4xl md:text-6xl font-extrabold tracking-tight bg-gradient-to-r from-white to-[#7c6df0] bg-clip-text text-transparent">
        {{ PROJECT_NAME }}
      </h1>
      <p class="mt-4 text-[#888899] text-lg">
        Powered by <span class="text-[#7c6df0] font-semibold">Rusters</span> + Vue 3
      </p>
    </header>

    <!-- Main Content -->
    <main class="flex-1 max-w-4xl mx-auto w-full p-8 flex flex-col md:flex-row gap-6">
      <section class="flex-1 bg-[#1a1a24] border border-[#2a2a3a] rounded-2xl p-8 shadow-2xl hover:border-[#7c6df0] hover:shadow-[0_4px_32px_rgba(124,109,240,0.15)] transition-all duration-300">
        <h2 class="text-xs font-semibold text-[#888899] uppercase tracking-widest mb-4">
          Backend Status
        </h2>
        
        <div v-if="loading" class="flex items-center gap-3 text-[#888899]">
          <svg class="animate-spin h-5 w-5 text-[#7c6df0]" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Connecting to Rusters backend…
        </div>
        
        <p v-else-if="error" class="text-red-400 font-medium flex items-center gap-2">
          <span class="text-xl">⚠</span> {{ error }}
        </p>
        
        <pre v-else class="font-mono text-sm bg-[#0f0f13] border border-[#2a2a3a] rounded-xl p-6 text-green-400 overflow-x-auto leading-relaxed shadow-inner">
{{ JSON.stringify(backendInfo, null, 2) }}</pre>
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
