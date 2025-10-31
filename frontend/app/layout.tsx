import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import { Providers } from './providers'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'ArcPilot - AI-Powered Trading on Arc',
  description: 'Voice-controlled AI trading agent for USDC arbitrage on Arc blockchain',
  keywords: ['Arc', 'USDC', 'AI', 'Trading', 'DeFi', 'Arbitrage'],
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Providers>
          {children}
        </Providers>
      </body>
    </html>
  )
}

