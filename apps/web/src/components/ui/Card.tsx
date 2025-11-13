import { HTMLAttributes, ReactNode } from 'react'
import { motion } from 'framer-motion'
import { cn } from '@/lib/utils'

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode
  hover?: boolean
  gradient?: boolean
}

export default function Card({ children, hover = false, gradient = false, className = '', ...props }: CardProps) {
  const Component = hover ? motion.div : 'div'
  
  return (
    <Component
      className={cn(
        'bg-white border border-border-light rounded-2xl shadow-modern-md p-6 transition-all duration-200',
        hover && 'hover:shadow-modern-lg hover:-translate-y-1 cursor-pointer',
        gradient && 'bg-gradient-to-br from-white to-paper-dark',
        className
      )}
      {...(hover ? {
        whileHover: { y: -4 },
        transition: { duration: 0.2 }
      } : {})}
      {...props}
    >
      {children}
    </Component>
  )
}

interface CardHeaderProps extends HTMLAttributes<HTMLDivElement> {
  title: string
  description?: string
  action?: ReactNode
  icon?: ReactNode
}

export function CardHeader({ title, description, action, icon, className = '', ...props }: CardHeaderProps) {
  return (
    <div className={cn('flex items-start justify-between mb-6', className)} {...props}>
      <div className="flex items-start gap-3 flex-1">
        {icon && (
          <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-accent-blue to-accent-purple flex items-center justify-center text-white shrink-0">
            {icon}
          </div>
        )}
        <div className="flex-1">
          <h3 className="text-xl font-serif font-semibold text-text-primary">{title}</h3>
          {description && <p className="text-sm text-text-secondary mt-1">{description}</p>}
        </div>
      </div>
      {action && <div className="ml-4 shrink-0">{action}</div>}
    </div>
  )
}

interface CardFooterProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode
}

export function CardFooter({ children, className = '', ...props }: CardFooterProps) {
  return (
    <div
      className={cn('mt-6 pt-4 border-t border-border-light flex items-center gap-3', className)}
      {...props}
    >
      {children}
    </div>
  )
}
