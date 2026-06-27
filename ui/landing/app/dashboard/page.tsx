"use client"

import { useEffect, useMemo, useState } from "react"
import Link from "next/link"
import {
  Activity,
  ArrowLeft,
  Bell,
  ChevronRight,
  Cpu,
  HardDrive,
  KeyRound,
  LayoutDashboard,
  Pause,
  Play,
  Radar,
  Save,
  Search,
  Settings,
  Shield,
  ShieldAlert,
  ShieldCheck,
  Terminal,
  Users,
  Zap,
} from "lucide-react"
import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Pie,
  PieChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts"
import { Badge } from "@/components/ui/badge"
import { Progress } from "@/components/ui/progress"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

type Severity = "Critical" | "High" | "Medium" | "Low"
type ViewKey =
  | "overview"
  | "alerts"
  | "telemetry"
  | "endpoints"
  | "detections"
  | "console"
  | "users"
  | "settings"

const initialAlerts: {
  id: string
  severity: Severity
  title: string
  host: string
  user: string
  mitre: string
  time: string
  status: "open" | "triaging" | "contained"
}[] = [
  {
    id: "ALR-2841",
    severity: "Critical",
    title: "PowerShell EncodedCommand from Office",
    host: "WIN-FIN-04",
    user: "j.morrison",
    mitre: "T1059.001",
    time: "2m ago",
    status: "open",
  },
  {
    id: "ALR-2840",
    severity: "Critical",
    title: "Suspicious LSASS handle access",
    host: "WIN-DEV-12",
    user: "SYSTEM",
    mitre: "T1003.001",
    time: "8m ago",
    status: "triaging",
  },
  {
    id: "ALR-2837",
    severity: "High",
    title: "Run-key persistence written (AppData binary)",
    host: "WIN-OPS-07",
    user: "k.gomez",
    mitre: "T1547.001",
    time: "14m ago",
    status: "open",
  },
  {
    id: "ALR-2836",
    severity: "High",
    title: "DLL sideload from temp path",
    host: "WIN-FIN-09",
    user: "a.lee",
    mitre: "T1574.002",
    time: "22m ago",
    status: "open",
  },
  {
    id: "ALR-2832",
    severity: "Medium",
    title: "Beacon-like outbound to rare ASN",
    host: "WIN-DEV-03",
    user: "m.berg",
    mitre: "T1071.001",
    time: "31m ago",
    status: "contained",
  },
  {
    id: "ALR-2828",
    severity: "Medium",
    title: "Hidden scheduled task created",
    host: "WIN-IT-02",
    user: "SYSTEM",
    mitre: "T1053.005",
    time: "47m ago",
    status: "contained",
  },
  {
    id: "ALR-2820",
    severity: "Low",
    title: "Unsigned binary executed from AppData",
    host: "WIN-DEV-11",
    user: "h.ozkan",
    mitre: "T1204.002",
    time: "1h ago",
    status: "contained",
  },
]

const endpoints = [
  { host: "WIN-FIN-04", os: "Win 11", risk: 92, eps: 1247, status: "online", lastSeen: "now" },
  { host: "WIN-DEV-12", os: "Win 11", risk: 88, eps: 982, status: "online", lastSeen: "now" },
  { host: "WIN-OPS-07", os: "Win 10", risk: 64, eps: 412, status: "online", lastSeen: "now" },
  { host: "WIN-FIN-09", os: "Win 11", risk: 58, eps: 690, status: "online", lastSeen: "now" },
  { host: "WIN-DEV-03", os: "Win 11", risk: 42, eps: 188, status: "online", lastSeen: "now" },
  { host: "WIN-IT-02", os: "Srv 22", risk: 35, eps: 1530, status: "online", lastSeen: "now" },
  { host: "WIN-MKT-08", os: "Win 11", risk: 12, eps: 84, status: "online", lastSeen: "now" },
  { host: "WIN-LEG-01", os: "Win 10", risk: 8, eps: 62, status: "offline", lastSeen: "3h ago" },
]

const initialThroughput = Array.from({ length: 30 }, (_, i) => ({
  t: i,
  events: Math.round(6000 + Math.sin(i / 3) * 1800 + Math.random() * 1500),
}))

const tactics = [
  { name: "Initial Access", covered: 7, total: 10 },
  { name: "Execution", covered: 9, total: 14 },
  { name: "Persistence", covered: 12, total: 19 },
  { name: "Privilege Esc.", covered: 8, total: 13 },
  { name: "Defense Evasion", covered: 14, total: 42 },
  { name: "Credential Access", covered: 6, total: 17 },
  { name: "Discovery", covered: 11, total: 30 },
  { name: "Lateral Movement", covered: 5, total: 9 },
  { name: "Collection", covered: 4, total: 17 },
  { name: "Command & Control", covered: 10, total: 16 },
  { name: "Exfiltration", covered: 3, total: 9 },
  { name: "Impact", covered: 5, total: 13 },
]

const liveEventsSeed = [
  { source: "ETW:Process", text: "powershell.exe spawned by WINWORD.EXE (suspicious parent)" },
  { source: "ETW:Image", text: "C:\\Users\\j.morrison\\AppData\\Local\\Temp\\loader.dll loaded" },
  { source: "ETW:Registry", text: "HKCU\\...\\Run value written → 'UpdaterSvc'" },
  { source: "ETW:Process", text: "rundll32.exe -> loader.dll,Start (unsigned)" },
  { source: "net", text: "Outbound TCP 47.91.83.221:443 (rare ASN, beacon cadence)" },
  { source: "ETW:Thread", text: "Remote thread injected into lsass.exe (handle 0x4f8)" },
  { source: "ETW:PS", text: "Invoke-Expression on base64 blob (8.4KB)" },
  { source: "ETW:Process", text: "WINWORD.EXE child: cmd.exe /c powershell -enc <...>" },
  { source: "ETW:Image", text: "ntdll.dll mapped to remote process (5832)" },
  { source: "net", text: "DNS query: cdn-update-tracker.xyz (newly seen)" },
  { source: "ETW:Process", text: "schtasks.exe /create /tn UpdaterSvc /sc onlogon" },
  { source: "ETW:Registry", text: "HKLM\\System\\CurrentControlSet\\Services modified" },
]

const detectionRules = [
  {
    name: "PowerShell EncodedCommand from Office app",
    technique: "T1059.001",
    severity: "Critical" as Severity,
    enabled: true,
    hits: 24,
    confidence: 96,
  },
  {
    name: "LSASS read access from unsigned binary",
    technique: "T1003.001",
    severity: "Critical" as Severity,
    enabled: true,
    hits: 18,
    confidence: 94,
  },
  {
    name: "Run-key written to AppData payload",
    technique: "T1547.001",
    severity: "High" as Severity,
    enabled: true,
    hits: 12,
    confidence: 88,
  },
  {
    name: "DLL sideload — temp path + signed loader",
    technique: "T1574.002",
    severity: "High" as Severity,
    enabled: true,
    hits: 9,
    confidence: 85,
  },
  {
    name: "Beacon cadence — uniform packets, rare ASN",
    technique: "T1071.001",
    severity: "Medium" as Severity,
    enabled: true,
    hits: 6,
    confidence: 78,
  },
  {
    name: "Hidden scheduled task on logon trigger",
    technique: "T1053.005",
    severity: "Medium" as Severity,
    enabled: true,
    hits: 5,
    confidence: 81,
  },
  {
    name: "Unsigned binary from AppData",
    technique: "T1204.002",
    severity: "Low" as Severity,
    enabled: false,
    hits: 41,
    confidence: 62,
  },
  {
    name: "WMI persistence (__EventConsumer)",
    technique: "T1546.003",
    severity: "High" as Severity,
    enabled: true,
    hits: 2,
    confidence: 92,
  },
]

const users = [
  { name: "Elif Kaynar", email: "elif.kaynar@boemar.com.tr", role: "Admin", lastActive: "online" },
  { name: "Burak Yıldız", email: "burak@boemar.com.tr", role: "Analyst", lastActive: "12m ago" },
  { name: "Hannah Cole", email: "h.cole@boemar.com.tr", role: "Analyst", lastActive: "2h ago" },
  { name: "Marko Lehtinen", email: "m.lehtinen@boemar.com.tr", role: "Responder", lastActive: "1d ago" },
  { name: "Aiko Tanaka", email: "a.tanaka@boemar.com.tr", role: "Viewer", lastActive: "4d ago" },
]

const eventDistribution = [
  { name: "Process", value: 38, color: "#10b981" },
  { name: "Image load", value: 22, color: "#52525b" },
  { name: "Registry", value: 14, color: "#a1a1aa" },
  { name: "Network", value: 12, color: "#71717a" },
  { name: "Thread", value: 8, color: "#d4d4d8" },
  { name: "PowerShell", value: 6, color: "#27272a" },
]

const providersHealth = [
  { id: "Microsoft-Windows-Kernel-Process", events: 4280, status: "ok" },
  { id: "Microsoft-Windows-Kernel-ImageLoad", events: 2640, status: "ok" },
  { id: "Microsoft-Windows-Kernel-Registry", events: 1820, status: "ok" },
  { id: "Microsoft-Windows-PowerShell", events: 612, status: "ok" },
  { id: "Microsoft-Windows-Kernel-Network", events: 1240, status: "ok" },
  { id: "Microsoft-Windows-Threat-Intelligence", events: 84, status: "degraded" },
  { id: "Microsoft-Windows-WMI-Activity", events: 56, status: "ok" },
]

function severityChip(s: Severity) {
  if (s === "Critical") return "bg-rose-500/15 text-rose-300 border-rose-500/30"
  if (s === "High") return "bg-amber-500/15 text-amber-300 border-amber-500/30"
  if (s === "Medium") return "bg-zinc-500/15 text-zinc-300 border-zinc-500/30"
  return "bg-zinc-700/40 text-zinc-400 border-zinc-700"
}

function statusChip(s: "open" | "triaging" | "contained") {
  if (s === "open") return "bg-rose-500/10 text-rose-300 border-rose-500/30"
  if (s === "triaging") return "bg-emerald-500/10 text-emerald-300 border-emerald-500/30"
  return "bg-zinc-700/40 text-zinc-400 border-zinc-700"
}

function riskBar(risk: number) {
  if (risk >= 80) return "bg-rose-500"
  if (risk >= 50) return "bg-amber-500"
  if (risk >= 25) return "bg-zinc-400"
  return "bg-emerald-500"
}

const SIDEBAR_ITEMS: { key: ViewKey; icon: any; label: string; badge?: () => string | number }[] = [
  { key: "overview", icon: LayoutDashboard, label: "Overview" },
  { key: "alerts", icon: Bell, label: "Alerts", badge: () => initialAlerts.filter((a) => a.status === "open").length },
  { key: "telemetry", icon: Activity, label: "Telemetry" },
  { key: "endpoints", icon: HardDrive, label: "Endpoints", badge: () => endpoints.length },
  { key: "detections", icon: Radar, label: "Detections", badge: () => detectionRules.length },
  { key: "console", icon: Terminal, label: "Live console" },
  { key: "users", icon: Users, label: "Users", badge: () => users.length },
  { key: "settings", icon: Settings, label: "Settings" },
]

const VIEW_TITLES: Record<ViewKey, { title: string; subtitle: string }> = {
  overview: { title: "Fleet overview", subtitle: "Real-time ETW telemetry, behavioral detections, and remediation status." },
  alerts: { title: "Alerts", subtitle: "Active and historical detection alerts across the fleet." },
  telemetry: { title: "Telemetry", subtitle: "ETW provider health, event distribution, and ingest pipeline metrics." },
  endpoints: { title: "Endpoints", subtitle: "All managed Windows hosts, their risk score and live status." },
  detections: { title: "Detection rules", subtitle: "Behavioral signatures mapped to MITRE ATT&CK." },
  console: { title: "Live console", subtitle: "Raw ETW event stream, tail-style." },
  users: { title: "Users", subtitle: "SOC team members with console access." },
  settings: { title: "Settings", subtitle: "Agent configuration, integrations, and engine policy." },
}

export default function DashboardPage() {
  const [view, setView] = useState<ViewKey>("overview")
  const [eventsPerSec, setEventsPerSec] = useState(8472)
  const [throughput, setThroughput] = useState(initialThroughput)
  const [paused, setPaused] = useState(false)
  const [tick, setTick] = useState(0)
  const [mounted, setMounted] = useState(false)
  const [lastRefresh, setLastRefresh] = useState<string>("")
  const [liveLog, setLiveLog] = useState<
    { id: number; time: string; source: string; text: string }[]
  >([])

  useEffect(() => {
    setMounted(true)
    setLiveLog(
      liveEventsSeed.slice(0, 8).map((e, i) => ({
        id: i,
        time: new Date(Date.now() - i * 3000).toLocaleTimeString(),
        ...e,
      })),
    )
    setLastRefresh(new Date().toLocaleTimeString())
  }, [])

  useEffect(() => {
    if (paused) return
    const id = setInterval(() => {
      setEventsPerSec((p) => Math.max(2200, p + Math.floor(Math.random() * 700) - 320))
      setThroughput((prev) => [
        ...prev.slice(1),
        {
          t: prev[prev.length - 1].t + 1,
          events: Math.max(800, prev[prev.length - 1].events + Math.floor(Math.random() * 1400) - 700),
        },
      ])
      setLiveLog((prev) => {
        const next = liveEventsSeed[Math.floor(Math.random() * liveEventsSeed.length)]
        return [{ id: Date.now(), time: new Date().toLocaleTimeString(), ...next }, ...prev].slice(0, 30)
      })
      setLastRefresh(new Date().toLocaleTimeString())
      setTick((x) => x + 1)
    }, 1800)
    return () => clearInterval(id)
  }, [paused])

  const openAlerts = useMemo(() => initialAlerts.filter((a) => a.status === "open").length, [])
  const active = SIDEBAR_ITEMS.find((i) => i.key === view)
  const meta = VIEW_TITLES[view]

  return (
    <div className="flex min-h-screen">
      {/* Sidebar */}
      <aside className="hidden lg:flex w-60 shrink-0 flex-col border-r border-zinc-900 bg-zinc-950/80 backdrop-blur sticky top-0 h-screen">
        <div className="flex items-center gap-2 px-5 h-16 border-b border-zinc-900">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-emerald-500/10 text-emerald-400">
            <ShieldCheck className="h-5 w-5" strokeWidth={1.75} />
          </div>
          <span className="font-medium tracking-tight">SentraEDR</span>
        </div>

        <nav className="flex-1 px-3 py-4 space-y-1 text-sm">
          {SIDEBAR_ITEMS.map((item) => {
            const Icon = item.icon
            const isActive = view === item.key
            return (
              <button
                key={item.key}
                onClick={() => setView(item.key)}
                className={`group w-full flex items-center justify-between gap-2 rounded-lg px-3 py-2 transition-colors text-left ${
                  isActive
                    ? "bg-zinc-900 text-zinc-100 ring-1 ring-zinc-800"
                    : "text-zinc-400 hover:bg-zinc-900/60 hover:text-zinc-100"
                }`}
              >
                <span className="flex items-center gap-2.5">
                  <Icon
                    className={`h-4 w-4 ${isActive ? "text-emerald-400" : ""}`}
                    strokeWidth={1.75}
                  />
                  {item.label}
                </span>
                {item.badge && (
                  <span className="text-[10px] tabular-nums text-zinc-500 group-hover:text-zinc-300">
                    {item.badge()}
                  </span>
                )}
              </button>
            )
          })}
        </nav>

        <div className="border-t border-zinc-900 px-4 py-3 text-xs text-zinc-500">
          <Link href="/" className="inline-flex items-center gap-1.5 hover:text-zinc-300 transition-colors">
            <ArrowLeft className="h-3 w-3" />
            Back to site
          </Link>
        </div>
      </aside>

      {/* Main */}
      <main className="flex-1 min-w-0">
        {/* Topbar */}
        <header className="sticky top-0 z-20 border-b border-zinc-900 bg-zinc-950/85 backdrop-blur">
          <div className="flex h-16 items-center gap-4 px-6">
            <div className="flex items-center gap-2 text-sm text-zinc-400">
              {active && <active.icon className="h-4 w-4" />}
              <span>{active?.label}</span>
            </div>

            <div className="ml-auto flex items-center gap-3">
              <div className="hidden md:flex items-center gap-2 h-9 rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 text-sm text-zinc-400 w-72">
                <Search className="h-4 w-4" />
                <input
                  placeholder="Search hosts, alerts, MITRE IDs…"
                  className="bg-transparent outline-none flex-1 placeholder:text-zinc-600 text-zinc-200"
                />
                <kbd className="hidden lg:inline-flex h-5 items-center rounded border border-zinc-800 bg-zinc-950 px-1.5 text-[10px] text-zinc-500">
                  ⌘K
                </kbd>
              </div>

              <button
                onClick={() => setPaused((p) => !p)}
                className="inline-flex items-center gap-1.5 h-9 rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 text-xs text-zinc-300 hover:bg-zinc-900 transition-colors"
              >
                {paused ? <Play className="h-3.5 w-3.5" /> : <Pause className="h-3.5 w-3.5" />}
                {paused ? "Resume" : "Pause"} stream
              </button>

              <Badge variant="outline" className="border-emerald-500/40 bg-emerald-500/10 text-emerald-300 gap-1.5">
                <span className="relative flex h-2 w-2">
                  <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-75" />
                  <span className="relative inline-flex h-2 w-2 rounded-full bg-emerald-400" />
                </span>
                Live
              </Badge>
            </div>
          </div>
        </header>

        <div className="p-6 space-y-6">
          {/* Page heading */}
          <div className="flex items-end justify-between gap-4 flex-wrap">
            <div>
              <h1 className="text-2xl font-semibold tracking-tight text-zinc-100">{meta.title}</h1>
              <p className="text-sm text-zinc-500">{meta.subtitle}</p>
            </div>
            <div className="text-xs text-zinc-500 tabular-nums" suppressHydrationWarning>
              {mounted ? `Last refresh · ${lastRefresh}` : "Last refresh · —"}
            </div>
          </div>

          {/* Views */}
          {view === "overview" && (
            <OverviewView
              eventsPerSec={eventsPerSec}
              openAlerts={openAlerts}
              throughput={throughput}
              liveLog={liveLog}
            />
          )}
          {view === "alerts" && <AlertsView />}
          {view === "telemetry" && <TelemetryView throughput={throughput} />}
          {view === "endpoints" && <EndpointsView />}
          {view === "detections" && <DetectionsView />}
          {view === "console" && <ConsoleView liveLog={liveLog} />}
          {view === "users" && <UsersView />}
          {view === "settings" && <SettingsView />}

          {/* Footer strip */}
          <div className="flex items-center justify-between text-xs text-zinc-600 pt-2">
            <span className="inline-flex items-center gap-1.5">
              <Zap className="h-3 w-3" />
              Engine: detection v0.4.2 · ETW providers: 7 active
            </span>
            <span>Build · sentra-edr · main</span>
          </div>
        </div>
      </main>
    </div>
  )
}

/* ───────────────────────── Views ───────────────────────── */

function OverviewView({
  eventsPerSec,
  openAlerts,
  throughput,
  liveLog,
}: {
  eventsPerSec: number
  openAlerts: number
  throughput: { t: number; events: number }[]
  liveLog: { id: number; time: string; source: string; text: string }[]
}) {
  return (
    <>
      {/* KPI grid */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <Kpi label="Events / sec" value={eventsPerSec.toLocaleString()} delta="+4.2%" positive icon={Activity} />
        <Kpi label="Open alerts" value={openAlerts.toString()} delta="-2" positive icon={ShieldAlert} />
        <Kpi label="Endpoints online" value="247 / 248" delta="99.6%" icon={HardDrive} />
        <Kpi label="Agent CPU avg" value="2.4%" delta="−0.3%" positive icon={Cpu} />
      </div>

      {/* Throughput + MITRE coverage */}
      <div className="grid grid-cols-1 xl:grid-cols-3 gap-3 mt-6">
        <div className="xl:col-span-2 rounded-xl border border-zinc-900 bg-zinc-950/60">
          <div className="flex items-center justify-between px-5 pt-4">
            <div>
              <p className="text-xs uppercase tracking-wider text-zinc-500">ETW throughput</p>
              <p className="text-sm text-zinc-300">last 60 seconds · all hosts</p>
            </div>
            <div className="flex items-center gap-2 text-xs text-zinc-500">
              <span className="inline-block h-2 w-2 rounded-full bg-emerald-400" />
              events / sec
            </div>
          </div>
          <div className="h-56 px-2 pb-2 pt-3">
            <ThroughputChart data={throughput} />
          </div>
          <Separator className="bg-zinc-900" />
          <div className="grid grid-cols-3 divide-x divide-zinc-900 text-xs">
            <MetricMini label="Drops" value="0" />
            <MetricMini label="Queue depth" value="14 / 4096" />
            <MetricMini label="Avg ingest" value="0.4 ms" />
          </div>
        </div>

        <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
          <div className="px-5 pt-4 pb-3">
            <p className="text-xs uppercase tracking-wider text-zinc-500">MITRE ATT&amp;CK coverage</p>
            <p className="text-sm text-zinc-300">technique mapping</p>
          </div>
          <ScrollArea className="h-[18rem] px-5 pb-4">
            <div className="space-y-2.5">
              {tactics.map((t) => {
                const pct = Math.round((t.covered / t.total) * 100)
                return (
                  <div key={t.name} className="space-y-1">
                    <div className="flex items-center justify-between text-xs">
                      <span className="text-zinc-300">{t.name}</span>
                      <span className="text-zinc-500 tabular-nums">
                        {t.covered}/{t.total}
                      </span>
                    </div>
                    <Progress value={pct} className="h-1.5 bg-zinc-900 [&>div]:bg-emerald-500" />
                  </div>
                )
              })}
            </div>
          </ScrollArea>
        </div>
      </div>

      {/* Alerts preview + Live events */}
      <div className="grid grid-cols-1 xl:grid-cols-3 gap-3 mt-6">
        <div className="xl:col-span-2 rounded-xl border border-zinc-900 bg-zinc-950/60">
          <div className="flex items-center justify-between px-5 pt-4">
            <p className="text-xs uppercase tracking-wider text-zinc-500">Recent alerts</p>
            <span className="text-xs text-zinc-500">{initialAlerts.length} total</span>
          </div>
          <div className="mt-3">
            <AlertsTable rows={initialAlerts.slice(0, 5)} />
          </div>
        </div>

        <LiveStreamPanel log={liveLog.slice(0, 12)} />
      </div>
    </>
  )
}

function AlertsView() {
  const [filter, setFilter] = useState<"all" | "open" | "triaging" | "contained">("all")
  const rows = filter === "all" ? initialAlerts : initialAlerts.filter((a) => a.status === filter)
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
      <div className="flex items-center justify-between px-5 pt-4 pb-3">
        <Tabs value={filter} onValueChange={(v) => setFilter(v as typeof filter)}>
          <TabsList className="bg-zinc-900/60 border border-zinc-900">
            <TabsTrigger value="all" className="text-xs data-[state=active]:bg-zinc-800">
              All ({initialAlerts.length})
            </TabsTrigger>
            <TabsTrigger value="open" className="text-xs data-[state=active]:bg-zinc-800">
              Open ({initialAlerts.filter((a) => a.status === "open").length})
            </TabsTrigger>
            <TabsTrigger value="triaging" className="text-xs data-[state=active]:bg-zinc-800">
              Triaging ({initialAlerts.filter((a) => a.status === "triaging").length})
            </TabsTrigger>
            <TabsTrigger value="contained" className="text-xs data-[state=active]:bg-zinc-800">
              Contained ({initialAlerts.filter((a) => a.status === "contained").length})
            </TabsTrigger>
          </TabsList>
        </Tabs>
      </div>
      <AlertsTable rows={rows} />
    </div>
  )
}

function TelemetryView({ throughput }: { throughput: { t: number; events: number }[] }) {
  return (
    <div className="space-y-6">
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <Kpi label="Providers active" value="7" delta="1 degraded" icon={Radar} />
        <Kpi label="Events / sec" value={throughput[throughput.length - 1].events.toLocaleString()} delta="rolling" icon={Activity} />
        <Kpi label="Queue saturation" value="0.4%" delta="14 / 4096" positive icon={Cpu} />
        <Kpi label="Drop rate" value="0.00%" delta="last 24h" positive icon={Shield} />
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-3 gap-3">
        <div className="xl:col-span-2 rounded-xl border border-zinc-900 bg-zinc-950/60">
          <div className="px-5 pt-4 pb-3 flex items-center justify-between">
            <div>
              <p className="text-xs uppercase tracking-wider text-zinc-500">Provider throughput</p>
              <p className="text-sm text-zinc-300">events per provider · last 5min</p>
            </div>
          </div>
          <div className="h-72 px-2 pb-4">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={providersHealth.map((p) => ({ name: p.id.split("-").pop(), events: p.events }))}>
                <CartesianGrid stroke="#27272a" strokeDasharray="3 3" vertical={false} />
                <XAxis dataKey="name" tick={{ fontSize: 10, fill: "#71717a" }} axisLine={false} tickLine={false} />
                <YAxis tick={{ fontSize: 10, fill: "#52525b" }} axisLine={false} tickLine={false} width={36} />
                <Tooltip
                  contentStyle={{ background: "#09090b", border: "1px solid #27272a", borderRadius: 8, fontSize: 12 }}
                />
                <Bar dataKey="events" radius={[4, 4, 0, 0]}>
                  {providersHealth.map((p, i) => (
                    <Cell key={i} fill={p.status === "ok" ? "#10b981" : "#f59e0b"} />
                  ))}
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
          <div className="px-5 pt-4 pb-3">
            <p className="text-xs uppercase tracking-wider text-zinc-500">Event distribution</p>
            <p className="text-sm text-zinc-300">by ETW source</p>
          </div>
          <div className="h-72 relative">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={eventDistribution}
                  cx="50%"
                  cy="50%"
                  innerRadius={55}
                  outerRadius={85}
                  paddingAngle={3}
                  dataKey="value"
                >
                  {eventDistribution.map((entry, i) => (
                    <Cell key={i} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip
                  contentStyle={{ background: "#09090b", border: "1px solid #27272a", borderRadius: 8, fontSize: 12 }}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
          <div className="px-5 pb-4 space-y-1">
            {eventDistribution.map((d) => (
              <div key={d.name} className="flex items-center justify-between text-xs">
                <span className="flex items-center gap-2 text-zinc-400">
                  <span className="h-2 w-2 rounded-sm" style={{ background: d.color }} />
                  {d.name}
                </span>
                <span className="text-zinc-500 tabular-nums">{d.value}%</span>
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
        <div className="px-5 pt-4 pb-3">
          <p className="text-xs uppercase tracking-wider text-zinc-500">ETW providers</p>
          <p className="text-sm text-zinc-300">subscription health</p>
        </div>
        <Table>
          <TableHeader>
            <TableRow className="border-zinc-900 hover:bg-transparent">
              <TableHead className="text-zinc-500">Provider</TableHead>
              <TableHead className="text-zinc-500">Events (5m)</TableHead>
              <TableHead className="text-zinc-500 text-right">Status</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {providersHealth.map((p) => (
              <TableRow key={p.id} className="border-zinc-900 hover:bg-zinc-900/40">
                <TableCell className="font-mono text-xs text-zinc-300">{p.id}</TableCell>
                <TableCell className="text-sm text-zinc-300 tabular-nums">{p.events.toLocaleString()}</TableCell>
                <TableCell className="text-right">
                  <Badge
                    variant="outline"
                    className={
                      p.status === "ok"
                        ? "bg-emerald-500/10 text-emerald-300 border-emerald-500/30 text-[10px]"
                        : "bg-amber-500/10 text-amber-300 border-amber-500/30 text-[10px]"
                    }
                  >
                    {p.status}
                  </Badge>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  )
}

function EndpointsView() {
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
      <div className="px-5 pt-4 pb-3 flex items-center justify-between">
        <div>
          <p className="text-xs uppercase tracking-wider text-zinc-500">Managed hosts</p>
          <p className="text-sm text-zinc-300">{endpoints.length} endpoints · {endpoints.filter((e) => e.status === "online").length} online</p>
        </div>
      </div>
      <Table>
        <TableHeader>
          <TableRow className="border-zinc-900 hover:bg-transparent">
            <TableHead className="text-zinc-500">Host</TableHead>
            <TableHead className="text-zinc-500">OS</TableHead>
            <TableHead className="text-zinc-500">EPS</TableHead>
            <TableHead className="text-zinc-500">Risk score</TableHead>
            <TableHead className="text-zinc-500">Last seen</TableHead>
            <TableHead className="text-zinc-500 text-right">Status</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {endpoints.map((e) => (
            <TableRow key={e.host} className="border-zinc-900 hover:bg-zinc-900/40 cursor-pointer">
              <TableCell className="font-mono text-sm text-zinc-200">{e.host}</TableCell>
              <TableCell className="text-sm text-zinc-400">{e.os}</TableCell>
              <TableCell className="text-sm text-zinc-300 tabular-nums">{e.eps.toLocaleString()}</TableCell>
              <TableCell className="w-48">
                <div className="flex items-center gap-2">
                  <div className="h-1.5 flex-1 rounded-full bg-zinc-900 overflow-hidden">
                    <div className={`h-full ${riskBar(e.risk)}`} style={{ width: `${e.risk}%` }} />
                  </div>
                  <span className="text-xs text-zinc-400 tabular-nums w-8 text-right">{e.risk}</span>
                </div>
              </TableCell>
              <TableCell className="text-xs text-zinc-500">{e.lastSeen}</TableCell>
              <TableCell className="text-right">
                <Badge
                  variant="outline"
                  className={
                    e.status === "online"
                      ? "bg-emerald-500/10 text-emerald-300 border-emerald-500/30 text-[10px]"
                      : "bg-zinc-800 text-zinc-500 border-zinc-700 text-[10px]"
                  }
                >
                  {e.status}
                </Badge>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}

function DetectionsView() {
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
      <div className="px-5 pt-4 pb-3 flex items-center justify-between">
        <div>
          <p className="text-xs uppercase tracking-wider text-zinc-500">Detection rules</p>
          <p className="text-sm text-zinc-300">{detectionRules.length} rules · {detectionRules.filter((r) => r.enabled).length} active</p>
        </div>
      </div>
      <Table>
        <TableHeader>
          <TableRow className="border-zinc-900 hover:bg-transparent">
            <TableHead className="text-zinc-500">Rule</TableHead>
            <TableHead className="text-zinc-500">Technique</TableHead>
            <TableHead className="text-zinc-500">Severity</TableHead>
            <TableHead className="text-zinc-500">Hits (24h)</TableHead>
            <TableHead className="text-zinc-500">Confidence</TableHead>
            <TableHead className="text-zinc-500 text-right">State</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {detectionRules.map((r) => (
            <TableRow key={r.name} className="border-zinc-900 hover:bg-zinc-900/40 cursor-pointer">
              <TableCell className="text-sm text-zinc-200">{r.name}</TableCell>
              <TableCell className="font-mono text-xs text-zinc-400">{r.technique}</TableCell>
              <TableCell>
                <Badge variant="outline" className={`${severityChip(r.severity)} text-[10px]`}>
                  {r.severity}
                </Badge>
              </TableCell>
              <TableCell className="text-sm text-zinc-300 tabular-nums">{r.hits}</TableCell>
              <TableCell className="w-40">
                <div className="flex items-center gap-2">
                  <div className="h-1.5 flex-1 rounded-full bg-zinc-900 overflow-hidden">
                    <div className="h-full bg-emerald-500" style={{ width: `${r.confidence}%` }} />
                  </div>
                  <span className="text-xs text-zinc-400 tabular-nums w-8 text-right">{r.confidence}</span>
                </div>
              </TableCell>
              <TableCell className="text-right">
                <Badge
                  variant="outline"
                  className={
                    r.enabled
                      ? "bg-emerald-500/10 text-emerald-300 border-emerald-500/30 text-[10px]"
                      : "bg-zinc-800 text-zinc-500 border-zinc-700 text-[10px]"
                  }
                >
                  {r.enabled ? "enabled" : "disabled"}
                </Badge>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}

function ConsoleView({ liveLog }: { liveLog: { id: number; time: string; source: string; text: string }[] }) {
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60 flex flex-col">
      <div className="flex items-center justify-between px-5 pt-4 pb-3 border-b border-zinc-900">
        <div>
          <p className="text-xs uppercase tracking-wider text-zinc-500">Live event stream</p>
          <p className="text-sm text-zinc-300">raw ETW · all hosts · {liveLog.length} buffered</p>
        </div>
        <Badge variant="outline" className="border-zinc-800 bg-zinc-900/60 text-zinc-400 text-[10px] font-mono">
          tail -f
        </Badge>
      </div>
      <ScrollArea className="px-5 py-4 h-[36rem]">
        <ul className="space-y-2 font-mono text-[11px] leading-relaxed">
          {liveLog.map((e) => (
            <li key={e.id} className="flex gap-3 text-zinc-400">
              <span className="text-zinc-600 shrink-0 w-20">{e.time}</span>
              <span className="text-emerald-400 shrink-0 w-32">{e.source}</span>
              <span className="text-zinc-300">{e.text}</span>
            </li>
          ))}
        </ul>
      </ScrollArea>
    </div>
  )
}

function UsersView() {
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
      <div className="px-5 pt-4 pb-3 flex items-center justify-between">
        <div>
          <p className="text-xs uppercase tracking-wider text-zinc-500">SOC team</p>
          <p className="text-sm text-zinc-300">{users.length} members</p>
        </div>
      </div>
      <Table>
        <TableHeader>
          <TableRow className="border-zinc-900 hover:bg-transparent">
            <TableHead className="text-zinc-500">Name</TableHead>
            <TableHead className="text-zinc-500">Email</TableHead>
            <TableHead className="text-zinc-500">Role</TableHead>
            <TableHead className="text-zinc-500 text-right">Last active</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {users.map((u) => (
            <TableRow key={u.email} className="border-zinc-900 hover:bg-zinc-900/40">
              <TableCell>
                <div className="flex items-center gap-3">
                  <div className="h-7 w-7 rounded-full bg-zinc-900 ring-1 ring-zinc-800 flex items-center justify-center text-xs text-zinc-300">
                    {u.name
                      .split(" ")
                      .map((n) => n[0])
                      .join("")}
                  </div>
                  <span className="text-sm text-zinc-100">{u.name}</span>
                </div>
              </TableCell>
              <TableCell className="text-sm text-zinc-400">{u.email}</TableCell>
              <TableCell>
                <Badge variant="outline" className="bg-zinc-900 text-zinc-300 border-zinc-800 text-[10px]">
                  {u.role}
                </Badge>
              </TableCell>
              <TableCell className="text-right text-xs text-zinc-500">{u.lastActive}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}

function SettingsView() {
  const [enabled, setEnabled] = useState({
    autoRemediation: true,
    quarantineFirst: true,
    syslogForward: false,
    telegramAlerts: false,
  })
  return (
    <div className="space-y-4">
      <SettingsGroup
        title="Detection engine"
        icon={Radar}
        rows={[
          { key: "ConfidenceThreshold", label: "Confidence threshold", value: "85" },
          { key: "EnabledRules", label: "Enabled detection rules", value: `${detectionRules.filter((r) => r.enabled).length} / ${detectionRules.length}` },
          { key: "FalsePositiveSuppression", label: "False-positive suppression", value: "aggressive" },
        ]}
      />
      <SettingsGroup
        title="Remediation"
        icon={Shield}
        toggles={[
          { key: "autoRemediation", label: "Auto-remediate when risk ≥ 85", checked: enabled.autoRemediation },
          { key: "quarantineFirst", label: "Always quarantine before delete", checked: enabled.quarantineFirst },
        ]}
        setToggle={(k, v) => setEnabled((s) => ({ ...s, [k]: v }))}
      />
      <SettingsGroup
        title="Integrations"
        icon={KeyRound}
        toggles={[
          { key: "syslogForward", label: "Forward alerts to syslog", checked: enabled.syslogForward },
          { key: "telegramAlerts", label: "Critical alerts to Telegram", checked: enabled.telegramAlerts },
        ]}
        setToggle={(k, v) => setEnabled((s) => ({ ...s, [k]: v }))}
      />
      <SettingsGroup
        title="Agent"
        icon={Cpu}
        rows={[
          { key: "Version", label: "Agent version", value: "0.4.2" },
          { key: "Build", label: "Build channel", value: "stable" },
          { key: "AutoUpdate", label: "Auto-update", value: "enabled" },
        ]}
      />
      <div className="flex justify-end">
        <button className="inline-flex items-center gap-1.5 h-9 rounded-lg border border-emerald-500/40 bg-emerald-500/10 px-3 text-xs text-emerald-300 hover:bg-emerald-500/20 transition-colors">
          <Save className="h-3.5 w-3.5" />
          Save changes
        </button>
      </div>
    </div>
  )
}

/* ───────────────────────── Shared bits ───────────────────────── */

function AlertsTable({ rows }: { rows: typeof initialAlerts }) {
  return (
    <Table>
      <TableHeader>
        <TableRow className="border-zinc-900 hover:bg-transparent">
          <TableHead className="text-zinc-500">Alert</TableHead>
          <TableHead className="text-zinc-500">Host</TableHead>
          <TableHead className="text-zinc-500">MITRE</TableHead>
          <TableHead className="text-zinc-500">Status</TableHead>
          <TableHead className="text-zinc-500 text-right">Time</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {rows.map((a) => (
          <TableRow key={a.id} className="border-zinc-900 hover:bg-zinc-900/40 cursor-pointer group">
            <TableCell>
              <div className="flex items-start gap-3">
                <Badge variant="outline" className={`mt-0.5 ${severityChip(a.severity)} text-[10px]`}>
                  {a.severity}
                </Badge>
                <div className="min-w-0">
                  <p className="text-sm text-zinc-100 truncate">{a.title}</p>
                  <p className="text-[11px] text-zinc-500 font-mono">{a.id}</p>
                </div>
              </div>
            </TableCell>
            <TableCell>
              <div className="text-sm text-zinc-300 font-mono">{a.host}</div>
              <div className="text-[11px] text-zinc-500">{a.user}</div>
            </TableCell>
            <TableCell>
              <span className="font-mono text-xs text-zinc-300">{a.mitre}</span>
            </TableCell>
            <TableCell>
              <Badge variant="outline" className={`${statusChip(a.status)} text-[10px] capitalize`}>
                {a.status}
              </Badge>
            </TableCell>
            <TableCell className="text-right text-xs text-zinc-500">
              <div className="flex items-center justify-end gap-1">
                {a.time}
                <ChevronRight className="h-3.5 w-3.5 text-zinc-700 group-hover:text-zinc-400 transition-colors" />
              </div>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

function ThroughputChart({ data }: { data: { t: number; events: number }[] }) {
  return (
    <ResponsiveContainer width="100%" height="100%">
      <AreaChart data={data}>
        <defs>
          <linearGradient id="eps" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="#10b981" stopOpacity={0.5} />
            <stop offset="100%" stopColor="#10b981" stopOpacity={0} />
          </linearGradient>
        </defs>
        <CartesianGrid stroke="#27272a" strokeDasharray="3 3" vertical={false} />
        <XAxis dataKey="t" hide />
        <YAxis tick={{ fontSize: 10, fill: "#52525b" }} axisLine={false} tickLine={false} width={36} />
        <Tooltip
          contentStyle={{ background: "#09090b", border: "1px solid #27272a", borderRadius: 8, fontSize: 12 }}
          labelStyle={{ color: "#a1a1aa" }}
        />
        <Area type="monotone" dataKey="events" stroke="#10b981" strokeWidth={2} fill="url(#eps)" />
      </AreaChart>
    </ResponsiveContainer>
  )
}

function LiveStreamPanel({ log }: { log: { id: number; time: string; source: string; text: string }[] }) {
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60 flex flex-col">
      <div className="flex items-center justify-between px-5 pt-4 pb-3">
        <div>
          <p className="text-xs uppercase tracking-wider text-zinc-500">Live event stream</p>
          <p className="text-sm text-zinc-300">raw ETW</p>
        </div>
        <Badge variant="outline" className="border-zinc-800 bg-zinc-900/60 text-zinc-400 text-[10px] font-mono">
          tail -f
        </Badge>
      </div>
      <ScrollArea className="flex-1 px-5 pb-4 max-h-[26rem]">
        <ul className="space-y-2 font-mono text-[11px] leading-relaxed">
          {log.map((e) => (
            <li key={e.id} className="flex gap-3 text-zinc-400">
              <span className="text-zinc-600 shrink-0">{e.time.slice(0, 8)}</span>
              <span className="text-emerald-400 shrink-0">{e.source}</span>
              <span className="text-zinc-300">{e.text}</span>
            </li>
          ))}
        </ul>
      </ScrollArea>
    </div>
  )
}

function SettingsGroup({
  title,
  icon: Icon,
  rows,
  toggles,
  setToggle,
}: {
  title: string
  icon: any
  rows?: { key: string; label: string; value: string }[]
  toggles?: { key: string; label: string; checked: boolean }[]
  setToggle?: (k: string, v: boolean) => void
}) {
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60">
      <div className="px-5 pt-4 pb-3 flex items-center gap-2">
        <Icon className="h-4 w-4 text-zinc-400" strokeWidth={1.75} />
        <p className="text-sm text-zinc-200">{title}</p>
      </div>
      <Separator className="bg-zinc-900" />
      <div className="px-5 py-3 space-y-3">
        {rows?.map((r) => (
          <div key={r.key} className="flex items-center justify-between">
            <span className="text-sm text-zinc-400">{r.label}</span>
            <span className="text-sm text-zinc-200 font-mono">{r.value}</span>
          </div>
        ))}
        {toggles?.map((t) => (
          <div key={t.key} className="flex items-center justify-between">
            <span className="text-sm text-zinc-400">{t.label}</span>
            <button
              onClick={() => setToggle?.(t.key, !t.checked)}
              className={`relative h-5 w-9 rounded-full transition-colors ${
                t.checked ? "bg-emerald-500" : "bg-zinc-800"
              }`}
              aria-pressed={t.checked}
            >
              <span
                className={`absolute top-0.5 h-4 w-4 rounded-full bg-white transition-transform ${
                  t.checked ? "translate-x-4" : "translate-x-0.5"
                }`}
              />
            </button>
          </div>
        ))}
      </div>
    </div>
  )
}

function Kpi({
  label,
  value,
  delta,
  icon: Icon,
  positive,
}: {
  label: string
  value: string
  delta?: string
  icon: any
  positive?: boolean
}) {
  return (
    <div className="rounded-xl border border-zinc-900 bg-zinc-950/60 px-5 py-4">
      <div className="flex items-center justify-between">
        <span className="text-[11px] uppercase tracking-wider text-zinc-500">{label}</span>
        <Icon className="h-4 w-4 text-zinc-600" strokeWidth={1.75} />
      </div>
      <div className="mt-2 flex items-baseline gap-2">
        <span className="text-2xl font-semibold text-zinc-100 tabular-nums">{value}</span>
        {delta && (
          <span className={`text-xs tabular-nums ${positive ? "text-emerald-400" : "text-zinc-500"}`}>{delta}</span>
        )}
      </div>
    </div>
  )
}

function MetricMini({ label, value }: { label: string; value: string }) {
  return (
    <div className="px-5 py-3">
      <p className="text-[10px] uppercase tracking-wider text-zinc-500">{label}</p>
      <p className="mt-0.5 text-sm text-zinc-200 tabular-nums">{value}</p>
    </div>
  )
}
