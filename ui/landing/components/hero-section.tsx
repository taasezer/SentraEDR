"use client"
import { useEffect, useState } from "react"
import { ArrowUpRight, ArrowRight, Shield, Activity, Lock } from "lucide-react"
import { AnimatedText } from "./animated-text"
import { useI18n } from "@/lib/i18n"

export function HeroSection() {
  const [isVisible, setIsVisible] = useState(false)
  const [scrollProgress, setScrollProgress] = useState(0)
  const { t, locale } = useI18n()

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsVisible(true)
    }, 100)
    return () => clearTimeout(timer)
  }, [])

  useEffect(() => {
    let rafId: number
    let currentProgress = 0

    const handleScroll = () => {
      const scrollY = window.scrollY
      const maxScroll = 400
      const targetProgress = Math.min(scrollY / maxScroll, 1)

      const smoothUpdate = () => {
        currentProgress += (targetProgress - currentProgress) * 0.1

        if (Math.abs(targetProgress - currentProgress) > 0.001) {
          setScrollProgress(currentProgress)
          rafId = requestAnimationFrame(smoothUpdate)
        } else {
          setScrollProgress(targetProgress)
        }
      }

      cancelAnimationFrame(rafId)
      smoothUpdate()
    }

    window.addEventListener("scroll", handleScroll, { passive: true })
    return () => {
      window.removeEventListener("scroll", handleScroll)
      cancelAnimationFrame(rafId)
    }
  }, [])

  const easeOutQuad = (t: number) => t * (2 - t)
  const easeOutCubic = (t: number) => 1 - Math.pow(1 - t, 3)

  const scale = 1 - easeOutQuad(scrollProgress) * 0.15
  const borderRadius = easeOutCubic(scrollProgress) * 48
  const heightVh = 100 - easeOutQuad(scrollProgress) * 37.5

  return (
    <section className="pt-32 pb-12 px-6 min-h-screen flex items-center relative overflow-hidden">
      <div className="absolute inset-0 top-0">
        <div
          className="w-full will-change-transform overflow-hidden relative bg-[#020806]"
          style={{
            transform: `scale(${scale})`,
            borderRadius: `${borderRadius}px`,
            height: `${heightVh}vh`,
          }}
        >
          {/* Cyber green grid */}
          <div
            className="absolute inset-0 opacity-[0.35]"
            style={{
              backgroundImage:
                "linear-gradient(rgba(34, 197, 94, 0.25) 1px, transparent 1px), linear-gradient(90deg, rgba(34, 197, 94, 0.25) 1px, transparent 1px)",
              backgroundSize: "64px 64px",
              maskImage: "radial-gradient(ellipse at center, black 30%, transparent 78%)",
            }}
          />
          {/* Layered radial green glow */}
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,rgba(34,197,94,0.32),transparent_55%)]" />
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_30%_70%,rgba(16,185,129,0.22),transparent_55%)]" />
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_70%_25%,rgba(20,184,166,0.18),transparent_55%)]" />

          {/* Animated scan lines */}
          <div className="pointer-events-none absolute inset-x-0 top-1/3 h-px bg-gradient-to-r from-transparent via-emerald-400/70 to-transparent" />
          <div className="pointer-events-none absolute inset-x-0 bottom-1/3 h-px bg-gradient-to-r from-transparent via-emerald-300/40 to-transparent" />

          {/* Top vignette */}
          <div className="absolute inset-0 bg-gradient-to-b from-black/40 via-transparent to-black/60" />
        </div>
      </div>

      <div className="max-w-7xl mx-auto w-full relative z-10">
        <div className="text-center mb-12">
          <div
            className={`transition-all duration-1000 delay-[600ms] ${
              isVisible ? "opacity-100 translate-y-0" : "opacity-0 -translate-y-4"
            }`}
          >
            <div className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-emerald-500/10 backdrop-blur-md border border-emerald-400/30 text-emerald-100 text-xs uppercase tracking-[0.2em] mb-8">
              <span className="relative flex h-2 w-2">
                <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-75" />
                <span className="relative inline-flex h-2 w-2 rounded-full bg-emerald-400" />
              </span>
              {t("hero.badge")}
            </div>
          </div>

          <div
            className={`transition-all duration-1000 delay-[800ms] ${
              isVisible ? "opacity-100 translate-y-0" : "opacity-0 -translate-y-4"
            }`}
          >
            <h1
              className="font-serif text-[3.5rem] sm:text-[4.5rem] md:text-[5.5rem] lg:text-[6.5rem] xl:text-[7.5rem] 2xl:text-[8.5rem] font-normal leading-tight mb-6 w-full px-4 max-w-6xl mx-auto text-balance text-white"
              style={{ textShadow: "0 0 50px rgba(34, 197, 94, 0.25)" }}
            >
              <AnimatedText key={locale} text={t("hero.title")} delay={0.3} />
            </h1>
          </div>

          <div
            className={`transition-all duration-1000 delay-[1100ms] ${
              isVisible ? "opacity-100 translate-y-0" : "opacity-0 -translate-y-4"
            }`}
          >
            <p className="max-w-2xl mx-auto text-emerald-50/85 leading-relaxed text-base md:text-lg mb-10">
              {t("hero.subtitle")}
            </p>
          </div>

          <div
            className={`flex flex-col sm:flex-row gap-4 justify-center transition-all duration-1000 delay-[1300ms] ${
              isVisible ? "opacity-100 translate-y-0" : "opacity-0 translate-y-4"
            }`}
          >
            <a
              href="/dashboard"
              aria-label={t("common.dashboard.aria")}
              className="relative flex items-center justify-center gap-0 bg-emerald-400 text-emerald-950 rounded-full pl-6 pr-1.5 py-1.5 transition-all duration-300 group overflow-hidden shadow-[0_0_30px_rgba(34,197,94,0.45)] hover:shadow-[0_0_45px_rgba(34,197,94,0.6)]"
            >
              <span className="text-sm font-medium pr-4">{t("hero.cta.primary")}</span>
              <span className="w-10 h-10 bg-emerald-950 rounded-full flex items-center justify-center">
                <ArrowUpRight className="w-4 h-4 text-emerald-400" />
              </span>
            </a>

            <button className="relative flex items-center justify-center gap-0 border border-emerald-300/40 rounded-full pl-6 pr-1.5 py-1.5 transition-all duration-300 group overflow-hidden">
              <span className="absolute inset-0 bg-emerald-400 rounded-full scale-x-0 origin-right group-hover:scale-x-100 transition-transform duration-300" />
              <span className="text-sm text-white group-hover:text-emerald-950 pr-4 relative z-10 transition-colors duration-300">
                {t("hero.cta.secondary")}
              </span>
              <span className="w-10 h-10 rounded-full flex items-center justify-center relative z-10">
                <ArrowRight className="w-4 h-4 text-white group-hover:opacity-0 absolute transition-opacity duration-300" />
                <ArrowUpRight className="w-4 h-4 text-white group-hover:text-emerald-950 opacity-0 group-hover:opacity-100 transition-all duration-300" />
              </span>
            </button>
          </div>
        </div>

        <div
          className={`mt-16 grid grid-cols-1 sm:grid-cols-3 gap-4 max-w-3xl mx-auto transition-all duration-1000 delay-[1500ms] ${
            isVisible ? "opacity-100 translate-y-0" : "opacity-0 translate-y-8"
          }`}
        >
          {[
            { icon: Shield, label: t("hero.chip.1") },
            { icon: Activity, label: t("hero.chip.2") },
            { icon: Lock, label: t("hero.chip.3") },
          ].map(({ icon: Icon, label }, i) => (
            <div
              key={i}
              className="flex items-center gap-3 px-4 py-3 rounded-2xl bg-emerald-500/10 backdrop-blur-md border border-emerald-400/20 hover:border-emerald-400/40 transition-colors"
            >
              <Icon className="w-5 h-5 text-emerald-300" strokeWidth={1.75} />
              <span className="text-sm text-white/90">{label}</span>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
