import { ReactNode } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { IconX } from '@tabler/icons-react'
import { cn } from '@/lib/utils'

interface ModalProps {
  isOpen: boolean
  onClose: () => void
  children: ReactNode
  title?: string
  size?: 'sm' | 'md' | 'lg' | 'xl'
}

export default function Modal({ isOpen, onClose, children, title, size = 'md' }: ModalProps) {
  const sizeStyles = {
    sm: 'max-w-md',
    md: 'max-w-lg',
    lg: 'max-w-2xl',
    xl: 'max-w-4xl',
  }

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="fixed inset-0 bg-black/50 backdrop-blur-sm z-40"
            onClick={onClose}
          />

          <div className="fixed inset-0 z-50 overflow-y-auto">
            <div className="flex min-h-full items-center justify-center p-4">
              <motion.div
                initial={{ opacity: 0, scale: 0.95, y: 20 }}
                animate={{ opacity: 1, scale: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.95, y: 20 }}
                transition={{ duration: 0.2 }}
                className={cn(
                  'relative w-full bg-white rounded-2xl shadow-modern-2xl overflow-hidden',
                  sizeStyles[size]
                )}
                onClick={(e) => e.stopPropagation()}
              >
                {title && (
                  <div className="flex items-center justify-between px-6 py-4 border-b border-border-light bg-gradient-to-r from-accent-blue/5 to-accent-purple/5">
                    <h3 className="text-xl font-serif font-semibold text-text-primary">
                      {title}
                    </h3>
                    <motion.button
                      whileHover={{ scale: 1.1, rotate: 90 }}
                      whileTap={{ scale: 0.9 }}
                      onClick={onClose}
                      className="p-2 rounded-xl hover:bg-paper-dark transition-colors"
                    >
                      <IconX className="w-5 h-5 text-text-secondary" />
                    </motion.button>
                  </div>
                )}

                <div className="px-6 py-6">{children}</div>
              </motion.div>
            </div>
          </div>
        </>
      )}
    </AnimatePresence>
  )
}

interface ModalFooterProps {
  children: ReactNode
}

export function ModalFooter({ children }: ModalFooterProps) {
  return (
    <div className="flex items-center justify-end gap-3 pt-6 border-t border-border-light mt-6">
      {children}
    </div>
  )
}
