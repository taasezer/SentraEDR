"use client"

import { motion } from "framer-motion"
import { Radio, Filter, Brain, Pause, ShieldCheck } from "lucide-react"
import { Badge } from "@/components/ui/badge"

const steps = [
  {
    icon: Radio,
    step: "01",
    title: "Capture",
    subtitle: "ETW telemetry",
    body: "Process, image-load, registry, thread, PowerShell, and network providers stream into the agent through a dedicated Tokio ingestion runtime — never blocking, never polling.",
    latency: "~0.4ms",
  },
  {
    icon: Filter,
    step: "02",
    title: "Normalize",
    subtitle: "Schema validation",
    body: "Raw events are converted into NormalizedTelemetryEvent structs, validated against shared-models, and routed through bounded MPSC channels with priority-aware drop policies.",
    latency: "~0.8ms",
  },
  {
    icon: Brain,
    step: "03",
    title: "Correlate",
    subtitle: "Behavioral scoring",
    body: "engine-detection runs heuristic scoring and multi-signal correlation across process, network, and persistence streams to build attack-chain verdicts with confidence scores.",
    latency: "~6ms",
  },
  {
    icon: Pause,
    step: "04",
    title: "Decide",
    subtitle: "Two-step gate",
    body: "Verdicts above the confidence threshold (risk ≥ 85) enter the remediation confirmation layer. Lower confidence stays observable; analysts see signal without auto-actions.",
    latency: "policy",
  },
  {
    icon: ShieldCheck,
    step: "05",
    title: "Remediate",
    subtitle: "Safe &amp; reversible",
    body: "Suspend → isolate network → quarantine file → backup registry → verification window. Every action is audited, reversible, and human-overridable in real deployments.",
    latency: "reversible",
  },
]

export function PipelineSection() {
  return (
    <section className="py-32 px-6 bg-zinc-50/40 border-y border-zinc-100">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-20">
          <Badge variant="outline" className="mb-6 border-zinc-300 bg-zinc-50 text-zinc-700">
            Pipeline
          </Badge>
          <h2 className="text-4xl md:text-5xl font-normal mb-6 text-balance font-serif">
            From kernel event to safe remediation.
          </h2>
          <p className="text-muted-foreground max-w-2xl mx-auto leading-relaxed">
            Five stages. Every stage isolated, bounded, and benchmarked — so your endpoint stays fast even under
            burst event load.
          </p>
        </div>

        <div className="relative">
          {/* Vertical line for mobile */}
          <div className="absolute left-6 top-2 bottom-2 w-px bg-gradient-to-b from-zinc-300 via-zinc-200 to-transparent md:hidden" />

          <div className="grid grid-cols-1 md:grid-cols-5 gap-6 md:gap-3">
            {steps.map((s, i) => {
              const Icon = s.icon
              return (
                <motion.div
                  key={s.step}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.5, delay: i * 0.1 }}
                  viewport={{ once: true }}
                  className="relative pl-16 md:pl-0"
                >
                  {/* Step circle */}
                  <div className="absolute left-0 top-0 md:relative md:left-auto md:top-auto md:mb-4 flex items-center md:justify-center">
                    <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-white border border-emerald-200 shadow-sm">
                      <Icon className="h-5 w-5 text-emerald-600" strokeWidth={1.75} />
                    </div>
                  </div>

                  {/* Arrow connector between cards (desktop) */}
                  {i < steps.length - 1 && (
                    <div className="hidden md:block absolute top-6 left-[calc(50%+1.5rem)] right-[-1.5rem] h-px bg-gradient-to-r from-emerald-300 to-transparent" />
                  )}

                  <div>
                    <div className="flex items-center gap-2 mb-1.5">
                      <span className="font-mono text-[11px] text-emerald-700">{s.step}</span>
                      <span className="text-[10px] uppercase tracking-wider text-slate-400">{s.subtitle}</span>
                    </div>
                    <h3 className="text-lg font-medium text-slate-900 mb-2">{s.title}</h3>
                    <p
                      className="text-sm text-slate-600 leading-relaxed mb-3"
                      dangerouslySetInnerHTML={{ __html: s.body }}
                    />
                    <Badge variant="outline" className="bg-white text-[10px] text-slate-500">
                      {s.latency}
                    </Badge>
                  </div>
                </motion.div>
              )
            })}
          </div>
        </div>
      </div>
    </section>
  )
}
