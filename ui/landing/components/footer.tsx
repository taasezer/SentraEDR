import Link from "next/link"
import { Github, Twitter, ShieldCheck } from "lucide-react"

export function Footer() {
  return (
    <div className="relative">
      {/* Soft monochrome backdrop */}
      <div className="absolute -top-[20vw] left-0 right-0 w-full h-[50vw] z-0 overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-b from-zinc-100 via-zinc-50 to-transparent" />
      </div>

      <div className="absolute -top-[15vw] left-0 right-0 flex items-end justify-center overflow-visible pointer-events-none z-10">
        <h2 className="font-bold text-center text-[28vw] sm:text-[25vw] md:text-[22vw] lg:text-[20vw] leading-[0.85] tracking-tighter text-zinc-200 whitespace-nowrap">
          SENTRA
        </h2>
      </div>

      <footer id="contact" className="relative z-20 border-t border-zinc-200 py-14 px-6 bg-background">
        <div className="max-w-6xl mx-auto">
          <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-8 mb-12">
            <div>
              <Link href="/" className="flex items-center gap-2 mb-3">
                <ShieldCheck className="w-5 h-5 text-foreground" strokeWidth={1.75} />
                <span className="text-base font-medium text-foreground">SentraEDR</span>
              </Link>
              <p className="text-sm text-muted-foreground max-w-sm">
                Lightweight Anti-RAT &amp; Endpoint Detection for Windows.
              </p>
            </div>

            <nav className="flex flex-wrap items-center gap-x-6 gap-y-2">
              <Link href="#features" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                Platform
              </Link>
              <Link href="#architecture" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                Architecture
              </Link>
              <Link href="#pricing" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                Detections
              </Link>
              <Link href="#faq" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                FAQ
              </Link>
              <Link href="#" className="text-sm text-muted-foreground hover:text-foreground transition-colors">
                Docs
              </Link>
            </nav>

            <div className="flex gap-3">
              <Link
                href="#"
                aria-label="GitHub"
                className="w-9 h-9 border border-zinc-200 rounded-full flex items-center justify-center text-muted-foreground hover:text-foreground hover:border-zinc-400 transition-colors"
              >
                <Github className="w-4 h-4" />
              </Link>
              <Link
                href="#"
                aria-label="Twitter"
                className="w-9 h-9 border border-zinc-200 rounded-full flex items-center justify-center text-muted-foreground hover:text-foreground hover:border-zinc-400 transition-colors"
              >
                <Twitter className="w-4 h-4" />
              </Link>
            </div>
          </div>

          <div className="pt-6 border-t border-zinc-200 flex flex-col md:flex-row justify-between items-center gap-3">
            <p className="text-xs text-muted-foreground">© 2026 SentraEDR</p>
            <p className="text-xs text-muted-foreground">Built in Rust</p>
          </div>
        </div>
      </footer>
    </div>
  )
}
