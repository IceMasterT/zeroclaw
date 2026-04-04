import { Component, type ReactNode } from 'react'

interface ErrorBoundaryProps {
  children: ReactNode
}

interface ErrorBoundaryState {
  hasError: boolean
  message: string
}

export default class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props)
    this.state = { hasError: false, message: '' }
  }

  static getDerivedStateFromError(error: unknown): ErrorBoundaryState {
    const message = error instanceof Error ? error.message : 'Unknown UI error'
    return { hasError: true, message }
  }

  componentDidCatch(error: unknown): void {
    const message = error instanceof Error ? error.stack ?? error.message : String(error)
    console.error('[ZeroClaw UI] Caught render error', message)
  }

  handleReload = (): void => {
    window.location.reload()
  }

  handleHome = (): void => {
    window.location.assign('/')
  }

  render(): ReactNode {
    if (!this.state.hasError) {
      return this.props.children
    }

    return (
      <div className="min-h-screen bg-[#060d1b] text-white flex items-center justify-center p-6">
        <div className="max-w-xl w-full border border-red-700/60 bg-red-950/20 rounded-xl p-6 space-y-4">
          <h1 className="text-xl font-semibold text-red-200">UI Error Recovered</h1>
          <p className="text-sm text-red-100/90">
            A dashboard view crashed, but ZeroClaw is still running. You can reload safely.
          </p>
          <pre className="text-xs bg-black/30 border border-red-900/50 rounded-lg p-3 overflow-auto max-h-40 text-red-200">
            {this.state.message}
          </pre>
          <div className="flex gap-3">
            <button
              onClick={this.handleReload}
              className="px-4 py-2 rounded-lg bg-red-600 hover:bg-red-500 text-sm font-medium"
            >
              Reload
            </button>
            <button
              onClick={this.handleHome}
              className="px-4 py-2 rounded-lg border border-red-600/70 hover:bg-red-900/40 text-sm"
            >
              Go Home
            </button>
          </div>
        </div>
      </div>
    )
  }
}
