import { Header } from "@/components/header"
import { HeroSection } from "@/components/hero-section"
import { LogoCloudSection } from "@/components/logo-cloud-section"
import { StatsSection } from "@/components/stats-section"
import { ServicesSection } from "@/components/services-section"
import { FeaturesSection } from "@/components/features-section"
import { ArchitectureSection } from "@/components/architecture-section"
import { PipelineSection } from "@/components/pipeline-section"
import { PricingSection } from "@/components/pricing-section"
import { ComparisonSection } from "@/components/comparison-section"
import { CTASection } from "@/components/cta-section"
import { FAQSection } from "@/components/faq-section"
import { Footer } from "@/components/footer"

export default function Home() {
  return (
    <main className="min-h-screen bg-background">
      <Header />
      <HeroSection />
      <LogoCloudSection />
      <StatsSection />
      <ServicesSection />
      <FeaturesSection />
      <ArchitectureSection />
      <PipelineSection />
      <PricingSection />
      <ComparisonSection />
      <CTASection />
      <FAQSection />
      <Footer />
    </main>
  )
}
