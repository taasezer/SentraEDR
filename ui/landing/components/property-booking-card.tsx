"use client"

import { Calendar, MapPin, ShieldAlert, Activity, Network } from "lucide-react"
import { cn } from "@/lib/utils"

export interface PropertyBookingCardProps {
  propertyName: string
  location: string
  duration: string
  availableDate: string
  image: string
  pricePerNight: number
  currency?: string
  propertyType?: string
  features?: string[]
  amenities?: string[]
  rating?: number
  onBook?: () => void
  className?: string
}

const severityStyles: Record<string, { ring: string; gradient: string; text: string; chip: string }> = {
  Critical: {
    ring: "ring-zinc-700",
    gradient: "from-zinc-900 via-zinc-800 to-zinc-700",
    text: "text-zinc-900",
    chip: "bg-zinc-100 text-zinc-700",
  },
  High: {
    ring: "ring-zinc-500",
    gradient: "from-zinc-700 via-zinc-600 to-zinc-500",
    text: "text-zinc-700",
    chip: "bg-zinc-100 text-zinc-700",
  },
  Medium: {
    ring: "ring-zinc-400",
    gradient: "from-zinc-500 via-zinc-400 to-zinc-300",
    text: "text-zinc-600",
    chip: "bg-zinc-100 text-zinc-700",
  },
}

export function PropertyBookingCard({
  propertyName,
  location,
  duration,
  availableDate,
  pricePerNight,
  propertyType,
  features = [],
  amenities = [],
  rating,
  onBook,
  className,
}: PropertyBookingCardProps) {
  const severity = severityStyles[propertyType ?? "Medium"] ?? severityStyles.Medium

  return (
    <div
      className={cn("w-full h-full flex flex-col overflow-hidden rounded-3xl bg-white", className)}
      style={{
        boxShadow:
          "rgba(14, 63, 126, 0.04) 0px 0px 0px 1px, rgba(42, 51, 69, 0.04) 0px 1px 1px -0.5px, rgba(42, 51, 70, 0.04) 0px 3px 3px -1.5px, rgba(42, 51, 70, 0.04) 0px 6px 6px -3px, rgba(14, 63, 126, 0.04) 0px 12px 12px -6px, rgba(14, 63, 126, 0.04) 0px 24px 24px -12px",
      }}
    >
      {/* Threat banner */}
      <div className={cn("relative aspect-[16/9] w-full overflow-hidden bg-gradient-to-br", severity.gradient)}>
        {/* Grid pattern */}
        <div
          className="absolute inset-0 opacity-25"
          style={{
            backgroundImage:
              "linear-gradient(rgba(255,255,255,0.5) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.5) 1px, transparent 1px)",
            backgroundSize: "32px 32px",
            maskImage: "radial-gradient(ellipse at center, black 50%, transparent 90%)",
          }}
        />
        <div className="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" />

        {rating && (
          <div className="absolute left-3 top-3 rounded-lg bg-white/25 px-3 py-1 text-sm font-semibold text-white backdrop-blur-sm">
            confidence {rating}
          </div>
        )}

        <div className="absolute bottom-3 left-3 right-3">
          <div className="mb-1 flex items-center gap-2">
            <ShieldAlert className="h-5 w-5 text-white" />
            {propertyType && <span className="text-sm font-medium text-white/90">{propertyType} severity</span>}
          </div>
          <h3 className="text-balance text-2xl font-bold text-white leading-tight">{propertyName}</h3>
        </div>
      </div>

      {/* Content */}
      <div className="p-6 flex-1 flex flex-col">
        <div className="mb-4 space-y-2">
          <div className="flex items-center gap-2 text-sm text-slate-600">
            <MapPin className="h-4 w-4" />
            <span>{location}</span>
          </div>
          <div className="flex items-center gap-2 text-sm text-slate-600">
            <Calendar className="h-4 w-4" />
            <span>
              {availableDate} • {duration}
            </span>
          </div>
        </div>

        {features.length > 0 && (
          <div className="mb-4">
            <div className="mb-2 text-sm font-semibold text-slate-900">Signal indicators</div>
            <div className="flex flex-wrap gap-2">
              {features.slice(0, 3).map((feature, index) => (
                <span key={index} className={cn("rounded-lg px-2 py-1 text-xs font-medium", severity.chip)}>
                  {feature}
                </span>
              ))}
              {features.length > 3 && (
                <span className="rounded-lg bg-slate-100 px-2 py-1 text-xs font-medium text-slate-700">
                  +{features.length - 3} more
                </span>
              )}
            </div>
          </div>
        )}

        {amenities.length > 0 && (
          <div className="mb-4 flex flex-wrap gap-3">
            {amenities.slice(0, 3).map((amenity, index) => {
              let Icon = Activity
              if (amenity.toLowerCase().includes("dns") || amenity.toLowerCase().includes("pcap")) Icon = Network
              if (amenity.toLowerCase().includes("etw")) Icon = Activity
              if (amenity.toLowerCase().includes("backup") || amenity.toLowerCase().includes("rollback"))
                Icon = ShieldAlert

              return (
                <div key={index} className="flex items-center gap-1.5 text-sm text-slate-600">
                  <Icon className="h-4 w-4" />
                  <span>{amenity}</span>
                </div>
              )
            })}
          </div>
        )}

        <div className="flex items-center justify-between mt-auto pt-4">
          <div>
            <div className="text-sm text-slate-500">Risk score</div>
            <div className="text-2xl font-bold text-slate-900">
              {pricePerNight}
              <span className="text-sm font-normal text-slate-500"> / 100</span>
            </div>
          </div>
          <button
            onClick={onBook}
            className="rounded-xl bg-foreground px-6 py-3 font-semibold text-background transition-colors hover:bg-foreground/90 text-sm"
          >
            Inspect
          </button>
        </div>
      </div>
    </div>
  )
}
