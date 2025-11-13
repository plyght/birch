'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { supabase } from '@/lib/supabase'
import { useRouter } from 'next/navigation'
import { IconSettings, IconUsers, IconLogout } from '@tabler/icons-react'

interface HeaderProps {
  user?: {
    email?: string
  } | null
}

export default function Header({ user }: HeaderProps) {
  const router = useRouter()
  const [showUserMenu, setShowUserMenu] = useState(false)

  const handleSignOut = async () => {
    await supabase.auth.signOut()
    router.push('/')
  }

  return (
    <header className="h-16 bg-white/80 backdrop-blur-xl border-b border-border-light shadow-modern-sm sticky top-0 z-20">
      <div className="h-full px-4 sm:px-6 flex items-center justify-between">
        <div className="flex items-center gap-4">
        </div>

        <div className="relative">
          <motion.button
            whileTap={{ scale: 0.95 }}
            onClick={() => setShowUserMenu(!showUserMenu)}
            className="flex items-center gap-3 px-3 py-2 text-sm font-medium text-text-primary hover:bg-paper-dark rounded-xl transition-all duration-200"
          >
            <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-accent-blue to-accent-purple text-white flex items-center justify-center font-semibold shadow-modern-md">
              {user?.email?.charAt(0).toUpperCase() || 'U'}
            </div>
            <span className="hidden md:block">{user?.email || 'User'}</span>
            <motion.svg
              animate={{ rotate: showUserMenu ? 180 : 0 }}
              transition={{ duration: 0.2 }}
              className="w-4 h-4"
              fill="none"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path d="M19 9l-7 7-7-7" />
            </motion.svg>
          </motion.button>

          <AnimatePresence>
            {showUserMenu && (
              <>
                <div
                  className="fixed inset-0 z-30"
                  onClick={() => setShowUserMenu(false)}
                />

                <motion.div
                  initial={{ opacity: 0, y: -10, scale: 0.95 }}
                  animate={{ opacity: 1, y: 0, scale: 1 }}
                  exit={{ opacity: 0, y: -10, scale: 0.95 }}
                  transition={{ duration: 0.15 }}
                  className="absolute right-0 mt-2 w-64 bg-white border border-border-light rounded-2xl shadow-modern-xl z-40 overflow-hidden"
                >
                  <div className="px-4 py-3 border-b border-border-light bg-gradient-to-r from-accent-blue/5 to-accent-purple/5">
                    <p className="text-sm font-semibold text-text-primary truncate">
                      {user?.email}
                    </p>
                    <p className="text-xs text-text-secondary mt-0.5">Signed in</p>
                  </div>

                  <div className="py-2">
                    <motion.button
                      whileHover={{ x: 4 }}
                      onClick={() => {
                        setShowUserMenu(false)
                        router.push('/settings')
                      }}
                      className="w-full text-left px-4 py-2.5 text-sm text-text-primary hover:bg-paper-dark transition-colors flex items-center gap-3"
                    >
                      <IconSettings className="w-4 h-4 text-text-secondary" />
                      Settings
                    </motion.button>

                    <motion.button
                      whileHover={{ x: 4 }}
                      onClick={() => {
                        setShowUserMenu(false)
                        router.push('/team')
                      }}
                      className="w-full text-left px-4 py-2.5 text-sm text-text-primary hover:bg-paper-dark transition-colors flex items-center gap-3"
                    >
                      <IconUsers className="w-4 h-4 text-text-secondary" />
                      Team
                    </motion.button>
                  </div>

                  <div className="border-t border-border-light">
                    <motion.button
                      whileHover={{ x: 4 }}
                      onClick={handleSignOut}
                      className="w-full text-left px-4 py-2.5 text-sm text-red-600 hover:bg-red-50 transition-colors flex items-center gap-3"
                    >
                      <IconLogout className="w-4 h-4" />
                      Sign out
                    </motion.button>
                  </div>
                </motion.div>
              </>
            )}
          </AnimatePresence>
        </div>
      </div>
    </header>
  )
}
