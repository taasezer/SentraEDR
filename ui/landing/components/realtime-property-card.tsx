"use client"

import { AreaChart, Area, XAxis, ResponsiveContainer, Tooltip, CartesianGrid } from "recharts"
import { motion, AnimatePresence } from "framer-motion"
import { useState, useEffect } from "react"
import { Activity, ShieldAlert, Cpu, Network, ChevronRight, AlertTriangle, Radar } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs"

const defaultThroughput = [
  { t: "00:00", events: 4200 },
  { t: "02:00", events: 3100 },
  { t: "04:00", events: 2400 },
  { t: "06:00", events: 3900 },
  { t: "08:00", events: 8100 },
  { t: "10:00", events: 11200 },
  { t: "12:00", events: 9800 },
  { t: "14:00", events: 12400 },
  { t: "16:00", events: 14600 },
  { t: "18:00", events: 12100 },
  { t: "20:00", events: 8800 },
  { t: "22:00", events: 5200 },
]

const defaultAlerts = [
  {
    severity: "Critical",
    title: "PowerShell EncodedCmd",
    host: "WIN-FIN-04",
    mitre: "T1059.001",
    time: "2m ago",
  },
  {
    severity: "Critical",
    title: "LSASS handle access",
    host: "WIN-DEV-12",
    mitre: "T1003.001",
    time: "8m ago",
  },
  {
    severity: "High",
    title: "Run-key persistence",
    host: "WIN-OPS-07",
    mitre: "T1547.001",
    time: "14m ago",
  },
  {
    severity: "High",
    title: "Suspicious DLL sideload",
    host: "WIN-FIN-09",
    mitre: "T1574.002",
    time: "22m ago",
  },
  {
    severity: "Medium",
    title: "Beacon-like outbound",
    host: "WIN-DEV-03",
    mitre: "T1071.001",
    time: "31m ago",
  },
]

const severityBadge: Record<string, string> = {
  Critical: "bg-zinc-900 text-zinc-50 border-zinc-900 hover:bg-zinc-900",
  High: "bg-zinc-200 text-zinc-800 border-zinc-200 hover:bg-zinc-200",
  Medium: "bg-zinc-100 text-zinc-600 border-zinc-100 hover:bg-zinc-100",
}

export function RealtimePropertyCard() {
  const [eventsPerSec, setEventsPerSec] = useState(8472)
  const [openAlerts, setOpenAlerts] = useState(7)
  const [endpoints, setEndpoints] = useState(248)
  const [cpu, setCpu] = useState(2.4)
  const [throughput, setThroughput] = useState(defaultThroughput)
  const [alerts, setAlerts] = useState(defaultAlerts)

  useEffect(() => {
    const interval = setInterval(() => {
      setEventsPerSec((prev) => Math.max(2000, prev + Math.floor(Math.random() * 700) - 300))
      setOpenAlerts((prev) => Math.max(0, prev + Math.floor(Math.random() * 3) - 1))
      setCpu((prev) => Math.max(0.5, Math.min(8, prev + (Math.random() - 0.5) * 0.6)))
    }, 2000)
    return () => clearInterval(interval)
  }, [])

  useEffect(() => {
    const interval = setInterval(() => {
      setThroughput((prev) =>
        prev.map((item) => ({
          ...item,
          events: Math.max(800, item.events + Math.floor(Math.random() * 1400) - 700),
        })),
      )
    }, 2500)
    return () => clearInterval(interval)
  }, [])

  return (
    <motion.div
      initial={{ opacity: 0, y: 40 }}
      whileInView={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.8, ease: "easeOut" }}
      viewport={{ once: true }}
      className="w-full"
    >
      <Card className="rounded-2xl border-slate-200/70 bg-white py-0 shadow-[0_30px_60px_-30px_rgba(15,23,42,0.25),0_0_0_1px_rgba(15,23,42,0.04)]">
        {/* Header */}
        <div className="flex items-center justify-between gap-4 px-6 pt-6">
          <div className="flex items-center gap-3">
            <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-zinc-900 text-emerald-400">
              <Radar className="h-5 w-5" strokeWidth={1.75} />
            </div>
            <div>
              <CardTitle className="text-base text-zinc-900">SentraEDR Console</CardTitle>
              <p className="text-xs text-zinc-500">Fleet telemetry · last 24h</p>
            </div>
          </div>
          <Badge
            variant="outline"
            className="border-emerald-300 bg-emerald-50 text-emerald-700 gap-1.5 px-2.5 py-1"
          >
            <span className="relative flex h-2 w-2">
              <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-75" />
              <span className="relative inline-flex h-2 w-2 rounded-full bg-emerald-500" />
            </span>
            Live
          </Badge>
        </div>

        <Separator className="mt-5" />

        {/* KPI grid */}
        <div className="grid grid-cols-2 gap-px bg-slate-200/70 md:grid-cols-4">
          <KpiCell
            icon={Activity}
            label="Events / sec"
            value={eventsPerSec.toLocaleString()}
            accent
          />
          <KpiCell icon={ShieldAlert} label="Open alerts" value={openAlerts.toString()} />
          <KpiCell icon={Network} label="Endpoints" value={endpoints.toString()} />
          <KpiCell icon={Cpu} label="Agent CPU" value={`${cpu.toFixed(1)}%`} />
        </div>

        <Separator />

        <CardContent className="px-6 pb-6">
          <Tabs defaultValue="throughput" className="w-full">
            <div className="flex items-center justify-between">
              <TabsList className="bg-slate-100/80">
                <TabsTrigger value="throughput" className="text-xs data-[state=active]:bg-white">
                  Throughput
                </TabsTrigger>
                <TabsTrigger value="alerts" className="text-xs data-[state=active]:bg-white">
                  Alerts
                </TabsTrigger>
                <TabsTrigger value="mitre" className="text-xs data-[state=active]:bg-white">
                  MITRE
                </TabsTrigger>
              </TabsList>
              <a
                href="/dashboard"
                className="hidden md:inline-flex items-center gap-1 text-xs font-medium text-zinc-500 hover:text-zinc-900 transition-colors"
              >
                Open full console
                <ChevronRight className="h-3.5 w-3.5" />
              </a>
            </div>

            <TabsContent value="throughput" className="mt-4">
              <div className="h-40 w-full">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={throughput} margin={{ top: 5, right: 5, left: 0, bottom: 0 }}>
                    <defs>
                      <linearGradient id="eventsGradient" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stopColor="#10b981" stopOpacity={0.45} />
                        <stop offset="100%" stopColor="#10b981" stopOpacity={0} />
                      </linearGradient>
                    </defs>
                    <CartesianGrid stroke="#f1f5f9" vertical={false} />
                    <XAxis
                      dataKey="t"
                      tick={{ fontSize: 10, fill: "#94a3b8" }}
                      axisLine={false}
                      tickLine={false}
                      interval={1}
                    />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: "white",
                        border: "1px solid #e2e8f0",
                        borderRadius: "8px",
                        fontSize: "12px",
                      }}
                      labelStyle={{ color: "#0f172a", fontWeight: 600 }}
                    />
                    <Area
                      type="monotone"
                      dataKey="events"
                      stroke="#10b981"
                      strokeWidth={2}
                      fill="url(#eventsGradient)"
                    />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
              <div className="mt-3 flex items-center justify-between text-xs text-slate-500">
                <span>ETW ingest pipeline · bounded MPSC</span>
                <span className="font-medium text-emerald-600">0 drops</span>
              </div>
            </TabsContent>

            <TabsContent value="alerts" className="mt-4">
              <ul className="space-y-2">
                {alerts.map((alert, i) => (
                  <motion.li
                    key={i}
                    initial={{ opacity: 0, y: 8 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: i * 0.05 }}
                    className="group flex items-center gap-3 rounded-xl border border-slate-100 bg-slate-50/50 px-3 py-2.5 hover:border-slate-200 hover:bg-white transition-colors"
                  >
                    <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-white border border-slate-200">
                      <AlertTriangle className="h-4 w-4 text-slate-500" strokeWidth={1.75} />
                    </div>
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center gap-2">
                        <p className="truncate text-sm font-medium text-slate-900">{alert.title}</p>
                        <Badge
                          variant="outline"
                          className={`${severityBadge[alert.severity]} text-[10px] px-1.5 py-0`}
                        >
                          {alert.severity}
                        </Badge>
                      </div>
                      <p className="truncate text-xs text-slate-500">
                        {alert.host} · {alert.mitre}
                      </p>
                    </div>
                    <span className="shrink-0 text-xs text-slate-400 group-hover:text-slate-500">
                      {alert.time}
                    </span>
                  </motion.li>
                ))}
              </ul>
            </TabsContent>

            <TabsContent value="mitre" className="mt-4">
              <div className="grid grid-cols-2 gap-2 sm:grid-cols-3">
                {[
                  { tactic: "Execution", count: 8 },
                  { tactic: "Persistence", count: 6 },
                  { tactic: "Credential Access", count: 4 },
                  { tactic: "Defense Evasion", count: 7 },
                  { tactic: "Discovery", count: 5 },
                  { tactic: "Command & Control", count: 6 },
                ].map((t) => (
                  <div key={t.tactic} className="rounded-xl border border-zinc-100 bg-zinc-50 p-3">
                    <p className="text-[11px] uppercase tracking-wider text-zinc-600">{t.tactic}</p>
                    <p className="mt-1 text-xl font-semibold text-zinc-900">{t.count}</p>
                    <p className="text-[10px] text-zinc-500">techniques mapped</p>
                  </div>
                ))}
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </motion.div>
  )
}

function KpiCell({
  icon: Icon,
  label,
  value,
  accent = false,
}: {
  icon: any
  label: string
  value: string
  accent?: boolean
}) {
  return (
    <div className="bg-white px-5 py-4">
      <div className="flex items-center gap-2">
        <div
          className={`flex h-6 w-6 items-center justify-center rounded-md ${
            accent ? "bg-emerald-500/10 text-emerald-600" : "bg-zinc-100 text-zinc-600"
          }`}
        >
          <Icon className="h-3.5 w-3.5" strokeWidth={2} />
        </div>
        <span className="text-[11px] uppercase tracking-wider text-zinc-500">{label}</span>
      </div>
      <AnimatePresence mode="wait">
        <motion.p
          key={value}
          initial={{ opacity: 0, y: 6 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: -6 }}
          transition={{ duration: 0.2 }}
          className="mt-2 text-2xl font-semibold text-zinc-900 tabular-nums"
        >
          {value}
        </motion.p>
      </AnimatePresence>
    </div>
  )
}
