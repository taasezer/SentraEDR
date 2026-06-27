"use client"

import { Activity, Brain, Shield } from "lucide-react"
import { useState, useEffect, useRef } from "react"
import { useI18n } from "@/lib/i18n"

function AnimatedIcon({ Icon, delay = 0 }: { Icon: any; delay?: number }) {
  const [isVisible, setIsVisible] = useState(false)
  const iconRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true)
        }
      },
      { threshold: 0.3 },
    )

    if (iconRef.current) {
      observer.observe(iconRef.current)
    }

    return () => observer.disconnect()
  }, [])

  return (
    <div ref={iconRef} className="relative">
      <Icon
        className={`text-foreground h-16 w-16 ${isVisible ? "animate-draw-icon" : ""}`}
        strokeWidth={1}
        style={{
          strokeDasharray: isVisible ? undefined : 1000,
          strokeDashoffset: isVisible ? undefined : 1000,
        }}
      />
    </div>
  )
}

export function ServicesSection() {
  const [isVisible, setIsVisible] = useState(false)
  const sectionRef = useRef<HTMLDivElement>(null)
  const { t } = useI18n()

  const services = [
    {
      icon: Activity,
      title: t("services.etw.title"),
      description: t("services.etw.desc"),
    },
    {
      icon: Brain,
      title: t("services.behavior.title"),
      description: t("services.behavior.desc"),
    },
    {
      icon: Shield,
      title: t("services.remediation.title"),
      description: t("services.remediation.desc"),
    },
  ]

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true)
        }
      },
      { threshold: 0.2 },
    )

    if (sectionRef.current) {
      observer.observe(sectionRef.current)
    }

    return () => observer.disconnect()
  }, [])

  return (
    <section id="how-it-works" className="py-32 px-6 pb-24 relative overflow-hidden">
      <div className="absolute top-0 left-0 right-0 flex justify-center pointer-events-none z-0">
        <span className="font-bold text-center text-[18vw] sm:text-[16vw] md:text-[14vw] lg:text-[12vw] leading-none tracking-tighter text-zinc-100 whitespace-nowrap">
          MISSION
        </span>
      </div>

      <style jsx>{`
        @keyframes drawPath {
          from {
            stroke-dasharray: 1000;
            stroke-dashoffset: 1000;
          }
          to {
            stroke-dasharray: 1000;
            stroke-dashoffset: 0;
          }
        }
        :global(.animate-draw-icon) :global(path),
        :global(.animate-draw-icon) :global(line),
        :global(.animate-draw-icon) :global(polyline),
        :global(.animate-draw-icon) :global(circle),
        :global(.animate-draw-icon) :global(rect) {
          animation: drawPath 2s ease-out forwards;
        }
      `}</style>

      <div className="max-w-7xl mx-auto relative z-10">
        <div ref={sectionRef} className="relative px-6 lg:px-8 py-16 lg:py-10 mb-32 overflow-hidden rounded-3xl">
          {/* Animated cyber background */}
          <div className="absolute inset-0 w-full h-full bg-gradient-to-br from-slate-950 via-slate-900 to-zinc-950">
            <div
              className={`absolute inset-0 opacity-30 transition-transform duration-1000 ease-out ${
                isVisible ? "scale-100" : "scale-110"
              }`}
              style={{
                backgroundImage:
                  "linear-gradient(rgba(56, 189, 248, 0.18) 1px, transparent 1px), linear-gradient(90deg, rgba(56, 189, 248, 0.18) 1px, transparent 1px)",
                backgroundSize: "50px 50px",
                maskImage: "radial-gradient(ellipse at center, black 30%, transparent 80%)",
              }}
            />
            <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,rgba(16,185,129,0.18),transparent_60%)]" />
            <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_bottom_left,rgba(59,130,246,0.22),transparent_60%)]" />
          </div>

          {/* Text content on top */}
          <div className="relative z-10 grid lg:grid-cols-2 gap-12 lg:gap-20 items-center">
            <div className="order-1 lg:order-2">
              <p className="text-sm uppercase tracking-[0.2em] text-white/70 font-medium mb-4">{t("services.eyebrow")}</p>
              <h2 className="font-sans md:text-4xl lg:text-5xl font-medium text-white text-balance mb-8 text-5xl">
                {t("services.heading")}
              </h2>
              <div className="space-y-6 text-white/85 leading-relaxed">
                <p>{t("services.lead1")}</p>
                <p>{t("services.lead2")}</p>
              </div>
              <div className="mt-10"></div>
            </div>
          </div>
        </div>

        <div className="text-center mb-20">
          <h2 className="text-4xl md:text-5xl font-normal mb-6 text-balance font-serif">{t("services.stackTitle")}</h2>
          <p className="text-muted-foreground max-w-2xl mx-auto leading-relaxed">{t("services.stackSubtitle")}</p>
        </div>

        <div className="grid md:grid-cols-3 gap-8">
          {services.map((service, index) => (
            <div
              key={index}
              className="group p-8 rounded-3xl hover:bg-zinc-50 transition-colors duration-300 text-center"
            >
              <div className="mb-6 flex justify-center">
                <AnimatedIcon Icon={service.icon} delay={index * 0.2} />
              </div>
              <h3 className="text-xl font-medium mb-3 text-foreground">{service.title}</h3>
              <p className="text-muted-foreground leading-relaxed text-sm">{service.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
