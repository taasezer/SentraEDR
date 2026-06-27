"use client"

import { motion } from "framer-motion"
import { Check, Minus, X } from "lucide-react"

const rows = [
  { feature: "Behavioral detection (RAT, stealers, PowerShell)", av: false, edr: true, sentra: true },
  { feature: "ETW kernel telemetry", av: false, edr: true, sentra: true },
  { feature: "MITRE ATT&CK technique mapping", av: false, edr: true, sentra: true },
  { feature: "Quarantine with rollback", av: false, edr: "partial", sentra: true },
  { feature: "Modular open Rust codebase", av: false, edr: false, sentra: true },
  { feature: "Idle RAM under 150 MB", av: "partial", edr: false, sentra: true },
  { feature: "No kernel driver required", av: true, edr: false, sentra: true },
  { feature: "Multi-runtime async isolation", av: false, edr: "partial", sentra: true },
]

function Cell({ value }: { value: boolean | "partial" }) {
  if (value === true) {
    return (
      <span className="inline-flex h-6 w-6 items-center justify-center rounded-full bg-emerald-50 text-emerald-600">
        <Check className="h-3.5 w-3.5" strokeWidth={2.5} />
      </span>
    )
  }
  if (value === "partial") {
    return (
      <span className="inline-flex h-6 w-6 items-center justify-center rounded-full bg-zinc-100 text-zinc-500">
        <Minus className="h-3.5 w-3.5" strokeWidth={2.5} />
      </span>
    )
  }
  return (
    <span className="inline-flex h-6 w-6 items-center justify-center rounded-full bg-zinc-50 text-zinc-300">
      <X className="h-3.5 w-3.5" strokeWidth={2.5} />
    </span>
  )
}

export function ComparisonSection() {
  return (
    <section className="py-32 px-6">
      <div className="max-w-5xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-normal mb-6 text-balance font-serif">
            How SentraEDR compares
          </h2>
          <p className="text-muted-foreground max-w-2xl mx-auto leading-relaxed">
            Signature antivirus stops yesterday&apos;s files. Legacy EDR is heavy and opaque. SentraEDR is the modern
            middle path — built for analysts, transparent by design.
          </p>
        </div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          viewport={{ once: true }}
          className="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm"
        >
          <div className="grid grid-cols-[1.6fr_1fr_1fr_1fr] gap-2 px-6 py-4 border-b border-slate-200 bg-slate-50">
            <div className="text-xs uppercase tracking-wider text-slate-500">Capability</div>
            <div className="text-xs uppercase tracking-wider text-slate-500 text-center">Signature AV</div>
            <div className="text-xs uppercase tracking-wider text-slate-500 text-center">Legacy EDR</div>
            <div className="text-xs uppercase tracking-wider text-zinc-900 text-center font-semibold">
              SentraEDR
            </div>
          </div>

          {rows.map((row, i) => (
            <motion.div
              key={row.feature}
              initial={{ opacity: 0 }}
              whileInView={{ opacity: 1 }}
              transition={{ delay: i * 0.05 }}
              viewport={{ once: true }}
              className={`grid grid-cols-[1.6fr_1fr_1fr_1fr] gap-2 px-6 py-4 items-center ${
                i % 2 === 1 ? "bg-slate-50/40" : ""
              } ${i !== rows.length - 1 ? "border-b border-slate-100" : ""}`}
            >
              <div className="text-sm text-slate-800">{row.feature}</div>
              <div className="flex justify-center">
                <Cell value={row.av} />
              </div>
              <div className="flex justify-center">
                <Cell value={row.edr} />
              </div>
              <div className="flex justify-center">
                <Cell value={row.sentra} />
              </div>
            </motion.div>
          ))}
        </motion.div>

        <p className="mt-6 text-center text-xs text-slate-400">
          Comparison reflects typical product behavior; individual vendors may differ.
        </p>
      </div>
    </section>
  )
}
