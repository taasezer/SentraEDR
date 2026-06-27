"use client"

import { motion } from "framer-motion"
import { Card } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Activity, Cpu, FileCog, Network, Brain, ShieldCheck, GitBranch, Boxes } from "lucide-react"

const crates = [
  {
    id: "engine-etw",
    icon: Activity,
    title: "engine-etw",
    role: "Telemetry ingestion",
    desc: "ETW provider lifecycle, schema normalization, bounded MPSC routing.",
  },
  {
    id: "engine-process",
    icon: Cpu,
    title: "engine-process",
    role: "Process analysis",
    desc: "Parent-child trees, unsigned binary detection, AppData execution heuristics.",
  },
  {
    id: "engine-persistence",
    icon: FileCog,
    title: "engine-persistence",
    role: "Persistence",
    desc: "Run keys, scheduled tasks, services, WMI — with backup &amp; rollback.",
  },
  {
    id: "engine-network",
    icon: Network,
    title: "engine-network",
    role: "Network telemetry",
    desc: "Connection tracking, DNS anomalies, beacon cadence, C2 heuristics.",
  },
  {
    id: "engine-detection",
    icon: Brain,
    title: "engine-detection",
    role: "Central brain",
    desc: "Heuristic scoring, attack-chain correlation, false-positive suppression.",
  },
  {
    id: "shared-ipc",
    icon: GitBranch,
    title: "shared-ipc",
    role: "IPC layer",
    desc: "Named pipes, bounded queues, backpressure, schema-validated transport.",
  },
]

export function ArchitectureSection() {
  return (
    <section id="architecture" className="py-32 px-6 relative overflow-hidden">
      <div className="absolute top-1/2 -translate-y-1/2 left-0 right-0 flex justify-center pointer-events-none z-0">
        <span className="font-bold text-center text-[20vw] sm:text-[18vw] md:text-[16vw] lg:text-[14vw] leading-none tracking-tighter text-zinc-100 whitespace-nowrap">
          ENGINE
        </span>
      </div>

      <div className="max-w-7xl mx-auto relative z-10">
        <div className="text-center mb-16">
          <Badge variant="outline" className="mb-6 border-zinc-300 bg-zinc-50 text-zinc-700">
            <Boxes className="h-3 w-3" />
            Modular Rust workspace
          </Badge>
          <h2 className="text-4xl md:text-5xl font-normal mb-6 text-balance font-serif">
            Strict crate boundaries. One-way data flow.
          </h2>
          <p className="text-muted-foreground max-w-2xl mx-auto leading-relaxed">
            Every engine crate owns a single responsibility. No cross-engine calls, no shared mutable state — only
            schema-validated IPC. The architecture stays auditable as new detections land.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {crates.map((crate, i) => {
            const Icon = crate.icon
            return (
              <motion.div
                key={crate.id}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5, delay: i * 0.08 }}
                viewport={{ once: true }}
              >
                <Card className="relative h-full rounded-2xl border border-zinc-200 bg-white py-0 p-6 shadow-none hover:border-zinc-300 transition-colors">
                  <div className="flex items-start justify-between mb-4">
                    <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-zinc-100 text-zinc-700">
                      <Icon className="h-5 w-5" strokeWidth={1.75} />
                    </div>
                    <Badge variant="outline" className="bg-white border-zinc-200 text-zinc-500 text-[10px] uppercase tracking-wider">
                      {crate.role}
                    </Badge>
                  </div>
                  <h3 className="font-mono text-base font-semibold text-zinc-900 mb-2">{crate.title}</h3>
                  <p
                    className="text-sm text-zinc-600 leading-relaxed"
                    dangerouslySetInnerHTML={{ __html: crate.desc }}
                  />
                </Card>
              </motion.div>
            )
          })}
        </div>

        {/* Data flow */}
        <div className="mt-16 rounded-2xl border border-zinc-200 bg-white p-6 md:p-8">
          <div className="flex items-center gap-2 mb-6">
            <ShieldCheck className="h-5 w-5 text-emerald-600" />
            <p className="text-sm font-medium text-zinc-900">One-way data flow guarantee</p>
          </div>
          <div className="flex flex-col md:flex-row items-stretch md:items-center gap-3 text-xs">
            {[
              "ETW providers",
              "engine-etw",
              "process · network · persistence",
              "engine-detection",
              "shared-ipc",
              "UI / remediation",
            ].map((label, i, arr) => (
              <div key={i} className="flex items-center gap-3 flex-1 min-w-0">
                <div className="flex-1 rounded-xl border border-zinc-200 bg-zinc-50 text-zinc-700 px-3 py-2.5 text-center font-mono">
                  {label}
                </div>
                {i < arr.length - 1 && (
                  <span className="hidden md:block text-zinc-300" aria-hidden>
                    →
                  </span>
                )}
              </div>
            ))}
          </div>
          <p className="mt-4 text-xs text-zinc-500">
            Reverse flow is forbidden by design. Coupling breaks are caught at the workspace level.
          </p>
        </div>
      </div>
    </section>
  )
}
