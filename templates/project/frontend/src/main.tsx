import React, { useEffect, useState } from 'react'
import ReactDOM from 'react-dom/client'

interface BackendInfo {
  framework: string
  version: string
  project_id: string
  environment: string
  http_server: string
}

function App() {
  const [info, setInfo] = useState<BackendInfo | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetch('/api/_rusters/info')
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`)
        return res.json() as Promise<BackendInfo>
      })
      .then(setInfo)
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false))
  }, [])

  return (
    <div className="min-h-screen bg-[#0f0f13] text-[#e8e8f0] font-sans flex flex-col">
      {/* Header */}
      <header className="text-center py-16 px-8 border-b border-[#2a2a3a] bg-gradient-to-b from-[#7c6df0]/10 to-transparent">
        <h1 className="text-4xl md:text-6xl font-extrabold tracking-tight bg-gradient-to-r from-white to-[#7c6df0] bg-clip-text text-transparent">
          {{ PROJECT_NAME }}
        </h1>
        <p className="mt-4 text-[#888899] text-lg">
          Powered by <span className="text-[#7c6df0] font-semibold">Rusters</span> + React
        </p>
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-4xl mx-auto w-full p-8 flex flex-col md:flex-row gap-6">
        <section className="flex-1 bg-[#1a1a24] border border-[#2a2a3a] rounded-2xl p-8 shadow-2xl hover:border-[#7c6df0] hover:shadow-[0_4px_32px_rgba(124,109,240,0.15)] transition-all duration-300">
          <h2 className="text-xs font-semibold text-[#888899] uppercase tracking-widest mb-4">
            Backend Status
          </h2>
          
          {loading && (
            <div className="flex items-center gap-3 text-[#888899]">
              <svg className="animate-spin h-5 w-5 text-[#7c6df0]" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Connecting to Rusters backend…
            </div>
          )}
          
          {error && (
            <p className="text-red-400 font-medium flex items-center gap-2">
              <span className="text-xl">⚠</span> {error}
            </p>
          )}
          
          {info && (
            <pre className="font-mono text-sm bg-[#0f0f13] border border-[#2a2a3a] rounded-xl p-6 text-green-400 overflow-x-auto leading-relaxed shadow-inner">
              {JSON.stringify(info, null, 2)}
            </pre>
          )}
        </section>
      </main>
    </div>
  )
}

ReactDOM.createRoot(document.getElementById('app')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
