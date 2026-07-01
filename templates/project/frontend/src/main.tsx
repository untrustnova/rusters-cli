import React, { useEffect, useState } from 'react'

interface BackendInfo {
  framework: string
  version: string
  project_id: string
  environment: string
  http_server: string
}

export default function App() {
  const [info, setInfo] = useState<BackendInfo | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetch('/_rusters/info')
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`)
        return res.json() as Promise<BackendInfo>
      })
      .then(setInfo)
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false))
  }, [])

  return (
    <div className="app">
      <header className="app-header">
        <h1>{{ PROJECT_NAME }}</h1>
        <p className="subtitle">
          Powered by <span className="brand">Rusters</span> + React
        </p>
      </header>

      <main className="app-main">
        <section className="card">
          <h2>Backend Status</h2>
          {loading && <p>Connecting to Rusters backend…</p>}
          {error && <p className="error">{error}</p>}
          {info && <pre className="status-json">{JSON.stringify(info, null, 2)}</pre>}
        </section>
      </main>
    </div>
  )
}
