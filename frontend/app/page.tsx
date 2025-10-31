'use client'

import { ConnectButton } from "thirdweb/react"
import { createThirdwebClient } from "thirdweb"
import { Mic, TrendingUp, Shield, Zap } from "lucide-react"

const client = createThirdwebClient({
  clientId: process.env.NEXT_PUBLIC_THIRDWEB_CLIENT_ID || "",
})

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-blue-900 via-blue-800 to-blue-900">
      {/* Header */}
      <header className="border-b border-blue-700 bg-blue-900/50 backdrop-blur-sm">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div className="flex items-center gap-2">
            <div className="w-10 h-10 bg-gradient-to-br from-blue-400 to-purple-500 rounded-lg flex items-center justify-center">
              <Zap className="text-white" size={24} />
            </div>
            <h1 className="text-2xl font-bold text-white">ArcPilot</h1>
          </div>
          <ConnectButton 
            client={client}
            theme="dark"
          />
        </div>
      </header>

      {/* Hero Section */}
      <section className="container mx-auto px-4 py-20 text-center">
        <div className="max-w-4xl mx-auto">
          <div className="inline-flex items-center gap-2 bg-blue-800/50 border border-blue-600 rounded-full px-4 py-2 mb-6">
            <Mic className="text-blue-300" size={16} />
            <span className="text-blue-200 text-sm">Voice-First AI Trading Agent</span>
          </div>
          
          <h2 className="text-5xl md:text-7xl font-bold text-white mb-6 leading-tight">
            Trade with Your
            <span className="bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent"> Voice</span>
          </h2>
          
          <p className="text-xl text-blue-200 mb-8 max-w-2xl mx-auto">
            AI-powered arbitrage trading on Arc blockchain. Just talk to your co-pilot and earn with USDC. No complex commands needed.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center mb-12">
            <button className="bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white px-8 py-4 rounded-lg font-semibold text-lg transition-all transform hover:scale-105 shadow-lg">
              Start Trading
            </button>
            <button className="border-2 border-blue-400 text-blue-300 hover:bg-blue-800/50 px-8 py-4 rounded-lg font-semibold text-lg transition-all">
              Watch Demo
            </button>
          </div>

          {/* Stats */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-3xl mx-auto">
            <div className="bg-blue-800/30 border border-blue-700 rounded-lg p-6 backdrop-blur-sm">
              <div className="text-3xl font-bold text-white mb-2">$0.05</div>
              <div className="text-blue-300 text-sm">Avg Gas Cost (USDC)</div>
            </div>
            <div className="bg-blue-800/30 border border-blue-700 rounded-lg p-6 backdrop-blur-sm">
              <div className="text-3xl font-bold text-white mb-2">&lt;1s</div>
              <div className="text-blue-300 text-sm">Settlement Time</div>
            </div>
            <div className="bg-blue-800/30 border border-blue-700 rounded-lg p-6 backdrop-blur-sm">
              <div className="text-3xl font-bold text-white mb-2">98%</div>
              <div className="text-blue-300 text-sm">AI Accuracy</div>
            </div>
          </div>
        </div>
      </section>

      {/* Features */}
      <section className="container mx-auto px-4 py-20">
        <div className="max-w-6xl mx-auto">
          <h3 className="text-3xl font-bold text-white text-center mb-12">
            Why ArcPilot?
          </h3>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <FeatureCard
              icon={<Mic size={32} />}
              title="Voice-First Trading"
              description="Just talk naturally. No complex interfaces or commands. Your AI co-pilot understands and executes."
            />
            <FeatureCard
              icon={<TrendingUp size={32} />}
              title="AI-Powered Arbitrage"
              description="ML models detect profitable opportunities across DEXs in real-time with 98% accuracy."
            />
            <FeatureCard
              icon={<Shield size={32} />}
              title="Predictable Costs"
              description="Gas fees in USDC on Arc. No ETH volatility. Know exactly what you'll pay."
            />
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-blue-700 bg-blue-900/50 backdrop-blur-sm py-8">
        <div className="container mx-auto px-4 text-center text-blue-300">
          <p>Built for AI Agents on Arc with USDC Hackathon</p>
          <p className="text-sm mt-2">Powered by Circle Arc, Thirdweb & ElevenLabs</p>
        </div>
      </footer>
    </main>
  )
}

function FeatureCard({ icon, title, description }: { icon: React.ReactNode, title: string, description: string }) {
  return (
    <div className="bg-blue-800/30 border border-blue-700 rounded-lg p-8 backdrop-blur-sm hover:bg-blue-800/40 transition-all">
      <div className="text-blue-400 mb-4">{icon}</div>
      <h4 className="text-xl font-semibold text-white mb-3">{title}</h4>
      <p className="text-blue-300">{description}</p>
    </div>
  )
}

