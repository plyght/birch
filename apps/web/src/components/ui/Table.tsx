import { HTMLAttributes } from 'react'
import { motion } from 'framer-motion'
import { cn } from '@/lib/utils'

interface TableProps extends HTMLAttributes<HTMLTableElement> {
  children: React.ReactNode
}

export default function Table({ children, className = '', ...props }: TableProps) {
  return (
    <div className="w-full overflow-x-auto rounded-xl border border-border-light shadow-modern-sm">
      <table
        className={cn('w-full border-collapse text-sm', className)}
        {...props}
      >
        {children}
      </table>
    </div>
  )
}

interface TableHeaderProps extends HTMLAttributes<HTMLTableSectionElement> {
  children: React.ReactNode
}

export function TableHeader({ children, className = '', ...props }: TableHeaderProps) {
  return (
    <thead
      className={cn(
        'bg-gradient-to-r from-paper-dark to-paper border-b-2 border-border-strong',
        className
      )}
      {...props}
    >
      {children}
    </thead>
  )
}

interface TableBodyProps extends HTMLAttributes<HTMLTableSectionElement> {
  children: React.ReactNode
}

export function TableBody({ children, className = '', ...props }: TableBodyProps) {
  return (
    <tbody className={cn('bg-white divide-y divide-border-light', className)} {...props}>
      {children}
    </tbody>
  )
}

interface TableRowProps extends HTMLAttributes<HTMLTableRowElement> {
  children: React.ReactNode
  animated?: boolean
}

export function TableRow({ children, animated = true, className = '', ...props }: TableRowProps) {
  const Component = animated ? motion.tr : 'tr'
  
  return (
    <Component
      className={cn(
        'transition-colors duration-150',
        'hover:bg-paper-dark',
        className
      )}
      {...(animated ? {
        whileHover: { scale: 1.005 },
        transition: { duration: 0.15 }
      } : {})}
      {...props}
    >
      {children}
    </Component>
  )
}

interface TableHeadProps extends HTMLAttributes<HTMLTableCellElement> {
  children: React.ReactNode
}

export function TableHead({ children, className = '', ...props }: TableHeadProps) {
  return (
    <th
      className={cn(
        'px-6 py-3 text-left text-xs font-semibold text-text-primary uppercase tracking-wider',
        className
      )}
      {...props}
    >
      {children}
    </th>
  )
}

interface TableCellProps extends HTMLAttributes<HTMLTableCellElement> {
  children: React.ReactNode
}

export function TableCell({ children, className = '', ...props }: TableCellProps) {
  return (
    <td
      className={cn('px-6 py-4 text-text-primary whitespace-nowrap', className)}
      {...props}
    >
      {children}
    </td>
  )
}
