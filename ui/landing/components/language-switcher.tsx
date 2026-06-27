"use client"

import { useI18n } from "@/lib/i18n"

export function LanguageSwitcher({ tone = "light" }: { tone?: "light" | "dark" }) {
  const { locale, setLocale } = useI18n()

  const base =
    tone === "dark"
      ? "border-zinc-700 bg-zinc-900/40 text-zinc-300"
      : "border-zinc-200 bg-white/70 text-zinc-600"
  const active =
    tone === "dark" ? "bg-emerald-500/15 text-emerald-300" : "bg-zinc-900 text-white"

  return (
    <div
      className={`inline-flex items-center rounded-full border ${base} p-0.5 text-[11px] font-medium uppercase tracking-wider`}
      role="group"
      aria-label="Language switcher"
    >
      {(["en", "tr"] as const).map((l) => (
        <button
          key={l}
          type="button"
          onClick={() => setLocale(l)}
          aria-pressed={locale === l}
          className={`px-2.5 py-1 rounded-full transition-colors ${
            locale === l ? active : "hover:text-zinc-900"
          }`}
        >
          {l}
        </button>
      ))}
    </div>
  )
}
