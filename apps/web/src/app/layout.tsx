import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'Birch Dashboard',
  description: 'Credential management and secret rotation',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="font-sans antialiased">{children}</body>
    </html>
  )
}

