"use client"

import { useRef, useEffect, useState } from "react"
import { PropertyBookingCard } from "./property-booking-card"

const detections = [
  {
    propertyName: "PowerShell Encoded Command Abuse",
    location: "T1059.001 • Execution",
    duration: "Multi-signal correlation",
    availableDate: "Behavioral",
    image: "ps",
    pricePerNight: 92,
    propertyType: "Critical",
    features: ["Encoded payload", "Hidden window", "AMSI bypass attempt", "Parent: Office app"],
    amenities: ["ETW", "Process tree", "AMSI"],
    rating: 4.9,
  },
  {
    propertyName: "LSASS Credential Access",
    location: "T1003.001 • Credential Access",
    duration: "Memory + handle",
    availableDate: "Behavioral",
    image: "lsass",
    pricePerNight: 96,
    propertyType: "Critical",
    features: ["Suspicious handle on LSASS", "Unsigned caller", "Read access from non-system"],
    amenities: ["ETW", "Handle audit", "PE check"],
    rating: 4.95,
  },
  {
    propertyName: "Run-Key Persistence",
    location: "T1547.001 • Persistence",
    duration: "Registry telemetry",
    availableDate: "Heuristic",
    image: "run",
    pricePerNight: 78,
    propertyType: "High",
    features: ["HKCU Run write", "AppData payload", "Unsigned binary", "Recently dropped"],
    amenities: ["Registry", "Backup", "Rollback"],
    rating: 4.8,
  },
  {
    propertyName: "DLL Sideloading",
    location: "T1574.002 • Defense Evasion",
    duration: "Image load chain",
    availableDate: "Behavioral",
    image: "dll",
    pricePerNight: 84,
    propertyType: "High",
    features: ["DLL from temp path", "Loaded by signed binary", "Mismatched hash"],
    amenities: ["ETW", "PE check", "Sign verify"],
    rating: 4.85,
  },
  {
    propertyName: "Beacon-Like C2 Traffic",
    location: "T1071.001 • C2",
    duration: "Connection cadence",
    availableDate: "Network",
    image: "c2",
    pricePerNight: 80,
    propertyType: "High",
    features: ["Periodic outbound", "Rare domain", "Long-lived TCP", "Small uniform packets"],
    amenities: ["pcap", "DNS", "Heuristics"],
    rating: 4.75,
  },
  {
    propertyName: "Scheduled Task Abuse",
    location: "T1053.005 • Persistence",
    duration: "Task scheduler audit",
    availableDate: "Heuristic",
    image: "task",
    pricePerNight: 72,
    propertyType: "Medium",
    features: ["Hidden task", "Logon trigger", "Unsigned action", "AppData payload"],
    amenities: ["Tasks", "Backup", "Rollback"],
    rating: 4.7,
  },
]

export function PricingSection() {
  const scrollRef = useRef<HTMLDivElement>(null)
  const [isHovered, setIsHovered] = useState(false)
  const positionRef = useRef(0)
  const animationRef = useRef<number>()

  const duplicated = [...detections, ...detections, ...detections]

  useEffect(() => {
    const scrollContainer = scrollRef.current
    if (!scrollContainer) return

    const speed = isHovered ? 0.3 : 1
    let lastTime = performance.now()

    const animate = (currentTime: number) => {
      const deltaTime = currentTime - lastTime
      lastTime = currentTime

      positionRef.current += speed * (deltaTime / 16)

      const totalWidth = scrollContainer.scrollWidth / 3

      if (positionRef.current >= totalWidth) {
        positionRef.current = 0
      }

      scrollContainer.style.transform = `translateX(-${positionRef.current}px)`
      animationRef.current = requestAnimationFrame(animate)
    }

    animationRef.current = requestAnimationFrame(animate)

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current)
      }
    }
  }, [isHovered])

  return (
    <section id="pricing" className="py-32 overflow-hidden">
      <div className="max-w-7xl mx-auto px-6 text-center mb-20">
        <h2 className="text-4xl md:text-5xl font-normal mb-6 text-balance font-serif">Detection catalog</h2>
        <p className="text-muted-foreground max-w-2xl mx-auto leading-relaxed">
          A curated set of high-fidelity behavioral detections, each mapped to MITRE ATT&amp;CK and tuned against
          Atomic Red Team scenarios.
        </p>
      </div>

      <div className="relative w-full" onMouseEnter={() => setIsHovered(true)} onMouseLeave={() => setIsHovered(false)}>
        <div ref={scrollRef} className="flex gap-6" style={{ width: "fit-content" }}>
          {duplicated.map((detection, index) => (
            <div key={index} className="flex-shrink-0 w-[85vw] sm:w-[60vw] lg:w-[400px]">
              <PropertyBookingCard {...detection} onBook={() => console.log(`Inspecting ${detection.propertyName}`)} />
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
