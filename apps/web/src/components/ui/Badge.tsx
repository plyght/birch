import { HTMLAttributes } from 'react'
import { motion } from 'framer-motion'
import { cn } from '@/lib/utils'

interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  variant?: 'default' | 'success' | 'warning' | 'danger' | 'info' | 'gradient'
  children: React.ReactNode
  pulse?: boolean
}

export default function Badge({ variant = 'default', children, pulse = false, className = '', ...props }: BadgeProps) {
  const variantStyles = {
    default: 'bg-gray-100 text-text-secondary border border-gray-200',
    success: 'bg-green-100 text-green-800 border border-green-200',
    warning: 'bg-orange-100 text-orange-800 border border-orange-200',
    danger: 'bg-red-100 text-red-800 border border-red-200',
    info: 'bg-blue-100 text-blue-800 border border-blue-200',
    gradient: 'bg-gradient-to-r from-accent-blue to-accent-purple text-white border-0',
  }

  const BadgeComponent = pulse ? motion.span : 'span'

  return (
    <BadgeComponent
      className={cn(
        'inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-medium transition-all duration-200',
        variantStyles[variant],
        pulse && 'animate-pulse',
        className
      )}
      {...(pulse ? {
        animate: { scale: [1, 1.05, 1] },
        transition: { duration: 2, repeat: Infinity }
      } : {})}
      {...props}
    >
      {children}
    </BadgeComponent>
  )
}
