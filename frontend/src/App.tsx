import { useState, type FormEvent } from 'react'
import './App.css'

interface ShortenResponse {
  short_code: string
  short_url: string
}

function App() {
  const [url, setUrl] = useState('')
  const [shortUrl, setShortUrl] = useState('')
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)
  const [copied, setCopied] = useState(false)

  const apiUrl = import.meta.env.VITE_API_URL || ''

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()
    setError('')
    setShortUrl('')
    setCopied(false)

    if (!url.trim()) {
      setError('URLを入力してください')
      return
    }

    setLoading(true)

    try {
      const response = await fetch(`${apiUrl}/api/shorten`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ url }),
      })

      if (!response.ok) {
        const text = await response.text()
        throw new Error(text || 'URL の短縮に失敗しました')
      }

      const data: ShortenResponse = await response.json()
      setShortUrl(data.short_url)
      setUrl('')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'エラーが発生しました')
    } finally {
      setLoading(false)
    }
  }

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(shortUrl)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    } catch {
      setError('コピーに失敗しました')
    }
  }

  return (
    <div className="container">
      <h1>Shrimpli</h1>

      <form onSubmit={handleSubmit} className="form">
        <input
          type="url"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="https://example.com/very/long/url..."
          className="url-input"
          disabled={loading}
        />
        <button type="submit" className="submit-btn" disabled={loading}>
          {loading ? '処理中...' : '短縮する'}
        </button>
      </form>

      {error && <div className="error">{error}</div>}

      {shortUrl && (
        <div className="result">
          <p>短縮URL:</p>
          <div className="short-url-container">
            <a href={shortUrl} target="_blank" rel="noopener noreferrer" className="short-url">
              {shortUrl}
            </a>
            <button onClick={handleCopy} className="copy-btn">
              {copied ? 'コピー済み!' : 'コピー'}
            </button>
          </div>
        </div>
      )}
    </div>
  )
}

export default App
