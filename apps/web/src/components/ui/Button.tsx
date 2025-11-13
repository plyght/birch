import { ButtonHTMLAttributes, forwardRef } from 'react'
import { motion } from 'framer-motion'
import { cn } from '@/lib/utils'

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost' | 'gradient'
  size?: 'sm' | 'md' | 'lg'
  loading?: boolean
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      children,
      className = '',
      variant = 'primary',
      size = 'md',
      loading = false,
      disabled,
      ...props
    },
    ref
  ) => {
    const baseStyles =
      'inline-flex items-center justify-center font-medium transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed relative overflow-hidden'

    const variantStyles = {
      primary:
        'bg-accent-blue text-white hover:bg-accent-blue-hover focus:ring-accent-blue shadow-modern-sm hover:shadow-modern-md',
      secondary:
        'bg-white text-text-primary border border-border hover:bg-paper-dark focus:ring-accent-blue shadow-modern-sm',
      danger:
        'bg-red-600 text-white hover:bg-red-700 focus:ring-red-600 shadow-modern-sm hover:shadow-modern-md',
      ghost:
        'bg-transparent text-text-primary hover:bg-paper-dark focus:ring-accent-blue',
      gradient:
        'bg-gradient-to-r from-accent-blue to-accent-purple text-white hover:shadow-modern-lg focus:ring-accent-blue shadow-modern-md hover:shadow-glow-blue'
    }

    const sizeStyles = {
      sm: 'px-3 py-2 text-sm rounded-lg',
      md: 'px-4 py-2.5 text-base rounded-xl',
      lg: 'px-6 py-3 text-lg rounded-xl',
    }

    return (
      <motion.button
        ref={ref}
        whileTap={{ scale: 0.98 }}
        className={cn(baseStyles, variantStyles[variant], sizeStyles[size], className)}
        disabled={disabled || loading}
        {...props}
      >
        {loading && (
          <svg
            className="animate-spin -ml-1 mr-2 h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        )}
        {children}
      </motion.button>
    )
  }
)

Button.displayName = 'Button'

export default Button
