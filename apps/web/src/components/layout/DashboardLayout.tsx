'use client'

import { useEffect, useState, ReactNode } from 'react'
import { useRouter } from 'next/navigation'
import { motion } from 'framer-motion'
import { supabase } from '@/lib/supabase'
import DashboardSidebar from './Sidebar'
import Header from './Header'
import { cn } from '@/lib/utils'

interface DashboardLayoutProps {
  children: ReactNode
}

export default function DashboardLayout({ children }: DashboardLayoutProps) {
  const router = useRouter()
  const [user, setUser] = useState<{ email?: string } | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const checkAuth = async () => {
      const {
        data: { session },
      } = await supabase.auth.getSession()

      if (!session) {
        router.push('/')
        return
      }

      setUser(session.user)
      setLoading(false)
    }

    checkAuth()

    const {
      data: { subscription },
    } = supabase.auth.onAuthStateChange((_event, session) => {
      if (!session) {
        router.push('/')
      } else {
        setUser(session.user)
      }
    })

    return () => subscription.unsubscribe()
  }, [router])

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-paper via-paper-dark to-paper">
        <motion.div
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          className="text-center"
        >
          <div className="relative">
            <div className="animate-spin rounded-full h-16 w-16 border-4 border-paper-darker mx-auto mb-4"></div>
            <div className="animate-spin rounded-full h-16 w-16 border-4 border-t-accent-blue border-r-accent-purple absolute inset-0 mx-auto"></div>
          </div>
          <p className="text-text-secondary font-medium">Loading...</p>
        </motion.div>
      </div>
    )
  }

  return (
    <div
      className={cn(
        'mx-auto flex w-full flex-1 flex-col overflow-hidden md:flex-row bg-gradient-to-br from-paper via-paper-dark to-paper',
        'min-h-screen'
      )}
    >
      <DashboardSidebar />
      <div className="flex flex-1 flex-col">
        <Header user={user} />
        <main className="flex-1 overflow-auto">
          <div className="h-full w-full flex-1 flex-col rounded-tl-2xl border-l border-t border-border-light bg-white p-4 md:p-8 dark:border-neutral-700 dark:bg-neutral-900">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3 }}
            >
              {children}
            </motion.div>
          </div>
        </main>
      </div>
    </div>
  )
}
