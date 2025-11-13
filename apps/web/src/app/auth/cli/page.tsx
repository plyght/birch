'use client'

import { useEffect, useState } from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { supabase } from '@/lib/supabase'

export default function CliAuthPage() {
  const router = useRouter()
  const searchParams = useSearchParams()
  const [error, setError] = useState<string | null>(null)
  const [authorizing, setAuthorizing] = useState(false)

  useEffect(() => {
    const handleCliAuth = async () => {
      try {
        // Get state and callback URL from query params
        const state = searchParams.get('state')
        const callbackUrl = searchParams.get('callback')

        if (!state) {
          setError('Missing state parameter')
          return
        }

        if (!callbackUrl) {
          setError('Missing callback parameter')
          return
        }

        // Check if user is authenticated
        const { data: { session }, error: sessionError } = await supabase.auth.getSession()

        if (sessionError) {
          throw sessionError
        }

        if (!session) {
          // Redirect to login, then come back here
          const returnUrl = `/auth/cli?state=${encodeURIComponent(state)}&callback=${encodeURIComponent(callbackUrl)}`
          router.push(`/?redirect=${encodeURIComponent(returnUrl)}`)
          return
        }

        // User is authenticated, show authorization UI
        setAuthorizing(true)
      } catch (err: any) {
        console.error('CLI auth error:', err)
        setError(err.message || 'An error occurred')
      }
    }

    handleCliAuth()
  }, [router, searchParams])

  const handleAuthorize = async () => {
    try {
      const state = searchParams.get('state')
      const callbackUrl = searchParams.get('callback')

      if (!state || !callbackUrl) {
        setError('Missing required parameters')
        return
      }

      // Get current session
      const { data: { session }, error: sessionError } = await supabase.auth.getSession()

      if (sessionError || !session) {
        setError('Not authenticated')
        return
      }

      // Use the access token from Supabase session as the JWT
      const token = session.access_token

      // Redirect to CLI callback with token
      const redirectUrl = `${callbackUrl}?token=${encodeURIComponent(token)}&state=${encodeURIComponent(state)}`
      window.location.href = redirectUrl
    } catch (err: any) {
      console.error('Authorization error:', err)
      setError(err.message || 'Failed to authorize')
    }
  }

  const handleCancel = () => {
    const callbackUrl = searchParams.get('callback')
    if (callbackUrl) {
      const state = searchParams.get('state')
      const redirectUrl = `${callbackUrl}?error=access_denied&error_description=User%20cancelled%20authorization&state=${encodeURIComponent(state || '')}`
      window.location.href = redirectUrl
    } else {
      router.push('/')
    }
  }

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[#faf9f6]">
        <div className="text-center max-w-md p-8">
          <div className="text-6xl mb-4">✗</div>
          <h1 className="text-2xl font-semibold mb-2 text-[#1a1a1a]">Authorization Error</h1>
          <p className="text-gray-600 mb-6">{error}</p>
          <button
            onClick={() => router.push('/')}
            className="px-6 py-2 bg-[#2563eb] text-white rounded hover:bg-[#1d4ed8] transition-colors"
          >
            Return to Dashboard
          </button>
        </div>
      </div>
    )
  }

  if (!authorizing) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-[#faf9f6]">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[#2563eb] mx-auto mb-4"></div>
          <p className="text-gray-600">Checking authentication...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-[#faf9f6] p-4">
      <div className="max-w-md w-full bg-white border border-[#e5e5e5] rounded-lg shadow-sm p-8">
        <div className="text-center mb-6">
          <h1 className="text-2xl font-serif font-semibold mb-2 text-[#1a1a1a]">
            Authorize Birch CLI
          </h1>
          <p className="text-gray-600">
            The Birch CLI is requesting access to your account.
          </p>
        </div>

        <div className="bg-[#f5f5f5] border border-[#e5e5e5] rounded p-4 mb-6">
          <h2 className="font-medium mb-2 text-[#1a1a1a]">This will allow the CLI to:</h2>
          <ul className="text-sm text-gray-600 space-y-2">
            <li className="flex items-start">
              <span className="mr-2">•</span>
              <span>Manage your workspaces and credentials</span>
            </li>
            <li className="flex items-start">
              <span className="mr-2">•</span>
              <span>Configure providers and policies</span>
            </li>
            <li className="flex items-start">
              <span className="mr-2">•</span>
              <span>Perform secret rotations</span>
            </li>
            <li className="flex items-start">
              <span className="mr-2">•</span>
              <span>Access audit logs</span>
            </li>
          </ul>
        </div>

        <div className="flex gap-3">
          <button
            onClick={handleCancel}
            className="flex-1 px-4 py-2 border border-[#e5e5e5] text-gray-700 rounded hover:bg-gray-50 transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleAuthorize}
            className="flex-1 px-4 py-2 bg-[#2563eb] text-white rounded hover:bg-[#1d4ed8] transition-colors"
          >
            Authorize
          </button>
        </div>

        <p className="text-xs text-gray-500 text-center mt-4">
          You can revoke this access at any time from your account settings.
        </p>
      </div>
    </div>
  )
}

