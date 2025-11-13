'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  IconHome,
  IconFolder,
  IconKey,
  IconShieldCheck,
  IconCircleCheck,
  IconClipboardList,
  IconUsers,
  IconSettings,
} from '@tabler/icons-react'
import { Sidebar, SidebarBody, SidebarLink } from '@/components/ui/sidebar'
import { cn } from '@/lib/utils'

const navigation = [
  { label: 'Dashboard', href: '/dashboard', icon: IconHome },
  { label: 'Workspaces', href: '/workspaces', icon: IconFolder },
  { label: 'Credentials', href: '/credentials', icon: IconKey },
  { label: 'Policies', href: '/policies', icon: IconShieldCheck },
  { label: 'Approvals', href: '/approvals', icon: IconCircleCheck },
  { label: 'Audit Logs', href: '/audit', icon: IconClipboardList },
  { label: 'Team', href: '/team', icon: IconUsers },
  { label: 'Settings', href: '/settings', icon: IconSettings },
]

export default function DashboardSidebar() {
  const pathname = usePathname()
  const [open, setOpen] = useState(false)

  const links = navigation.map((item) => {
    const isActive = pathname === item.href || pathname?.startsWith(item.href + '/')
    const Icon = item.icon
    
    return {
      label: item.label,
      href: item.href,
      icon: (
        <Icon
          className={cn(
            'h-5 w-5 shrink-0 transition-colors',
            isActive
              ? 'text-accent-blue dark:text-accent-blue'
              : 'text-neutral-700 dark:text-neutral-200'
          )}
        />
      ),
    }
  })

  return (
    <Sidebar open={open} setOpen={setOpen}>
      <SidebarBody className="justify-between gap-10 bg-white dark:bg-neutral-900 border-r border-border-light">
        <div className="flex flex-1 flex-col overflow-x-hidden overflow-y-auto">
          {open ? <Logo /> : <LogoIcon />}
          <div className="mt-8 flex flex-col gap-2">
            {links.map((link, idx) => (
              <SidebarLink
                key={idx}
                link={link}
                className={cn(
                  'rounded-xl transition-all duration-200 px-3',
                  pathname === link.href || pathname?.startsWith(link.href + '/')
                    ? 'bg-gradient-to-r from-accent-blue/10 to-accent-purple/10 border border-accent-blue/20'
                    : 'hover:bg-paper-dark'
                )}
              />
            ))}
          </div>
        </div>
        <div>
          <div className="px-3 py-3 rounded-xl bg-gradient-to-r from-accent-blue/10 to-accent-purple/10 border border-accent-blue/20 mb-4">
            <div className="flex items-center gap-3">
              <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse shrink-0" />
              {open && (
                <motion.div
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  className="flex-1 min-w-0"
                >
                  <p className="text-xs font-medium text-text-primary truncate">System Active</p>
                  <p className="text-xs text-text-tertiary truncate">All services running</p>
                </motion.div>
              )}
            </div>
          </div>
          {open && (
            <motion.p
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="text-xs text-text-tertiary text-center"
            >
              © {new Date().getFullYear()} Birch
            </motion.p>
          )}
        </div>
      </SidebarBody>
    </Sidebar>
  )
}

export const Logo = () => {
  return (
    <Link
      href="/dashboard"
      className="relative z-20 flex items-center space-x-2 py-1 text-sm font-normal"
    >
      <div className="h-7 w-7 shrink-0 rounded-xl bg-gradient-to-br from-accent-blue to-accent-purple flex items-center justify-center text-white font-bold text-sm">
        B
      </div>
      <motion.span
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="font-serif font-semibold text-xl bg-gradient-to-r from-accent-blue to-accent-purple bg-clip-text text-transparent whitespace-pre"
      >
        Birch
      </motion.span>
    </Link>
  )
}

export const LogoIcon = () => {
  return (
    <Link
      href="/dashboard"
      className="relative z-20 flex items-center space-x-2 py-1 text-sm font-normal"
    >
      <div className="h-7 w-7 shrink-0 rounded-xl bg-gradient-to-br from-accent-blue to-accent-purple flex items-center justify-center text-white font-bold text-sm">
        B
      </div>
    </Link>
  )
}
