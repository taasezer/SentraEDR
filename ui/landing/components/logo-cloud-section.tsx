"use client"

import { motion } from "framer-motion"
import { useI18n } from "@/lib/i18n"

const logos = [
  { name: "Aurora SOC", tag: "FinServ" },
  { name: "Helix Labs", tag: "Research" },
  { name: "Northgate", tag: "Defense" },
  { name: "Vertex MSP", tag: "Managed Services" },
  { name: "Cipher.io", tag: "FinTech" },
  { name: "Quantum CSIRT", tag: "Public Sector" },
]

export function LogoCloudSection() {
  const { t } = useI18n()
  return (
    <section className="px-6 py-20 bg-zinc-50/50 border-y border-zinc-100">
      <div className="max-w-6xl mx-auto">
        <p className="text-center text-xs uppercase tracking-[0.25em] text-muted-foreground mb-10">
          {t("logocloud.title")}
        </p>
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-x-8 gap-y-6">
          {logos.map((logo, i) => (
            <motion.div
              key={logo.name}
              initial={{ opacity: 0, y: 10 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.08, duration: 0.4 }}
              viewport={{ once: true }}
              className="flex flex-col items-center text-center"
            >
              <span className="font-serif text-xl text-zinc-600 group-hover:text-zinc-900">{logo.name}</span>
              <span className="text-[10px] uppercase tracking-wider text-zinc-400 mt-1">{logo.tag}</span>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  )
}
