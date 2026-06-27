"use client"

import { createContext, useContext, useEffect, useState, type ReactNode } from "react"

export type Locale = "en" | "tr"

type Dict = Record<string, string>

const en: Dict = {
  // Header
  "nav.mission": "Mission",
  "nav.platform": "Platform",
  "nav.architecture": "Architecture",
  "nav.detections": "Detections",
  "nav.faq": "FAQ",
  "nav.deploy": "Open dashboard",
  "nav.login": "Console login",
  "common.dashboard.cta": "Open dashboard",
  "common.dashboard.aria": "Open the SentraEDR live dashboard",

  // Hero
  "hero.badge": "Real-time endpoint protection",
  "hero.title": "Stop the threat before it lands.",
  "hero.subtitle":
    "SentraEDR is a lightweight Rust-based Endpoint Detection & Response platform for Windows. ETW telemetry, behavioral RAT detection, and safe remediation — all under 150 MB of RAM.",
  "hero.cta.primary": "Open dashboard",
  "hero.cta.secondary": "Read the docs",
  "hero.chip.1": "Behavioral RAT detection",
  "hero.chip.2": "ETW kernel telemetry",
  "hero.chip.3": "Safe remediation",

  // Stats
  "stats.ram": "RAM at idle",
  "stats.latency": "Detection latency",
  "stats.techniques": "MITRE techniques mapped",

  // Logo cloud
  "logocloud.title": "Trusted by blue teams, MSSPs & security researchers",

  // Services / Mission
  "services.eyebrow": "Our mission",
  "services.heading": "Production-grade EDR. No bloat.",
  "services.lead1":
    "Modern attackers don't need malware on disk anymore. RATs, stealers, and PowerShell abuse chains slip past signature antivirus every day. SentraEDR closes that gap.",
  "services.lead2":
    "Built in Rust on a strictly modular crate architecture, the agent operates event-driven over ETW — never blocking, never bloating. Detection, telemetry, and remediation each live in isolated runtimes.",
  "services.stackTitle": "A complete detection stack",
  "services.stackSubtitle":
    "Every layer of a modern EDR — telemetry, behavioral analysis, and safe remediation — in one lightweight Windows agent.",
  "services.etw.title": "ETW Telemetry Engine",
  "services.etw.desc":
    "Real-time kernel-level visibility into process, image load, registry, thread, and PowerShell activity — without polling.",
  "services.behavior.title": "Behavioral Detection",
  "services.behavior.desc":
    "Heuristic scoring and attack-chain correlation catch RATs, stealers, and PowerShell abuse — not just known signatures.",
  "services.remediation.title": "Safe Remediation",
  "services.remediation.desc":
    "Suspend, isolate, quarantine, and rollback. Every action is reversible, audit-logged, and confidence-gated.",

  // Features
  "features.title": "Watch every endpoint, in real time.",
  "features.subtitle":
    "A unified console for live telemetry, alert triage, and remediation across your fleet. Built for SOC analysts who need signal — not noise.",
  "features.list.1": "Sub-millisecond ETW ingestion",
  "features.list.2": "MITRE ATT&CK technique mapping",
  "features.list.3": "Process & DLL injection detection",
  "features.list.4": "Persistence (Run keys, services, tasks)",
  "features.list.5": "PowerShell abuse detection",
  "features.list.6": "Quarantine with rollback",

  // CTA
  "cta.title": "Ready to harden your endpoints?",
  "cta.subtitle":
    "Deploy the SentraEDR agent in minutes, stream live telemetry, and let the detection engine do the rest.",
  "cta.primary": "Open dashboard",
  "cta.secondary": "View architecture",
}

const tr: Dict = {
  // Header
  "nav.mission": "Misyon",
  "nav.platform": "Platform",
  "nav.architecture": "Mimari",
  "nav.detections": "Tespitler",
  "nav.faq": "SSS",
  "nav.deploy": "Dashboard'u aç",
  "nav.login": "Konsola giriş",
  "common.dashboard.cta": "Dashboard'u aç",
  "common.dashboard.aria": "SentraEDR canlı dashboard'unu aç",

  // Hero
  "hero.badge": "Gerçek zamanlı endpoint koruması",
  "hero.title": "Tehdidi inmeden durdur.",
  "hero.subtitle":
    "SentraEDR, Windows için Rust ile geliştirilmiş hafif bir Endpoint Tespit ve Yanıt (EDR) platformudur. ETW telemetrisi, davranışsal RAT tespiti ve güvenli iyileştirme — 150 MB'ın altında RAM ile.",
  "hero.cta.primary": "Dashboard'u aç",
  "hero.cta.secondary": "Belgeleri oku",
  "hero.chip.1": "Davranışsal RAT tespiti",
  "hero.chip.2": "ETW çekirdek telemetrisi",
  "hero.chip.3": "Güvenli iyileştirme",

  // Stats
  "stats.ram": "Bekleme RAM kullanımı",
  "stats.latency": "Tespit gecikmesi",
  "stats.techniques": "MITRE tekniği eşleştirildi",

  // Logo cloud
  "logocloud.title": "Mavi takımlar, MSSP'ler ve güvenlik araştırmacıları tarafından tercih ediliyor",

  // Services / Mission
  "services.eyebrow": "Misyonumuz",
  "services.heading": "Üretim kalitesinde EDR. Şişirme yok.",
  "services.lead1":
    "Modern saldırganların artık diskte zararlı yazılıma ihtiyacı yok. RAT'lar, bilgi hırsızları ve PowerShell suistimal zincirleri her gün imza tabanlı antivirüsleri atlatıyor. SentraEDR bu boşluğu kapatır.",
  "services.lead2":
    "Sıkı modüler crate mimarisi üzerine Rust ile geliştirilen ajan, ETW üzerinden olay-güdümlü çalışır — asla bloklamaz, asla şişmez. Tespit, telemetri ve iyileştirme izole runtime'larda yaşar.",
  "services.stackTitle": "Komple bir tespit yığını",
  "services.stackSubtitle":
    "Modern bir EDR'nin her katmanı — telemetri, davranışsal analiz ve güvenli iyileştirme — tek bir hafif Windows ajanında.",
  "services.etw.title": "ETW Telemetri Motoru",
  "services.etw.desc":
    "Süreç, imaj yüklemesi, kayıt defteri, iş parçacığı ve PowerShell etkinliğine gerçek zamanlı çekirdek seviyesi görünürlük — yoklama yok.",
  "services.behavior.title": "Davranışsal Tespit",
  "services.behavior.desc":
    "Sezgisel skorlama ve saldırı zinciri korelasyonu; sadece bilinen imzaları değil, RAT'ları, stealer'ları ve PowerShell istismarını yakalar.",
  "services.remediation.title": "Güvenli İyileştirme",
  "services.remediation.desc":
    "Askıya al, izole et, karantinaya al ve geri al. Her aksiyon geri alınabilir, denetim kayıtlı ve güven eşiğiyle kontrollüdür.",

  // Features
  "features.title": "Her endpoint'i gerçek zamanlı izle.",
  "features.subtitle":
    "Filonuz genelinde canlı telemetri, alarm değerlendirme ve iyileştirme için birleşik bir konsol. Gürültü değil, sinyal isteyen SOC analistleri için tasarlandı.",
  "features.list.1": "Milisaniyenin altında ETW alımı",
  "features.list.2": "MITRE ATT&CK tekniği eşleştirmesi",
  "features.list.3": "Süreç ve DLL enjeksiyonu tespiti",
  "features.list.4": "Kalıcılık (Run anahtarları, servisler, görevler)",
  "features.list.5": "PowerShell istismarı tespiti",
  "features.list.6": "Geri alma destekli karantina",

  // CTA
  "cta.title": "Endpoint'lerinizi sertleştirmeye hazır mısınız?",
  "cta.subtitle":
    "SentraEDR ajanını dakikalar içinde kurun, canlı telemetri akıtın ve tespit motoru gerisini halletsin.",
  "cta.primary": "Dashboard'u aç",
  "cta.secondary": "Mimariye bak",
}

const dictionaries: Record<Locale, Dict> = { en, tr }

type I18nContextValue = {
  locale: Locale
  setLocale: (l: Locale) => void
  t: (key: string) => string
}

const I18nContext = createContext<I18nContextValue | null>(null)

export function I18nProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>("en")

  useEffect(() => {
    const stored = typeof window !== "undefined" ? (localStorage.getItem("sentra-locale") as Locale | null) : null
    if (stored === "en" || stored === "tr") {
      setLocaleState(stored)
    } else if (typeof navigator !== "undefined" && navigator.language.toLowerCase().startsWith("tr")) {
      setLocaleState("tr")
    }
  }, [])

  const setLocale = (l: Locale) => {
    setLocaleState(l)
    if (typeof window !== "undefined") {
      localStorage.setItem("sentra-locale", l)
      document.documentElement.lang = l
    }
  }

  const t = (key: string) => dictionaries[locale][key] ?? dictionaries.en[key] ?? key

  return <I18nContext.Provider value={{ locale, setLocale, t }}>{children}</I18nContext.Provider>
}

export function useI18n() {
  const ctx = useContext(I18nContext)
  if (!ctx) throw new Error("useI18n must be used inside <I18nProvider>")
  return ctx
}
