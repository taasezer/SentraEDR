import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@/components/ui/accordion"

const faqs = [
  {
    question: "Which operating systems does SentraEDR support?",
    answer:
      "The SentraEDR agent targets Windows 10 and Windows 11 (x64), with Windows Server support on the roadmap. The agent relies on built-in Event Tracing for Windows (ETW) — no kernel driver is required for current detections.",
  },
  {
    question: "How is SentraEDR different from a traditional antivirus?",
    answer:
      "Antivirus tools mostly match signatures against files on disk. SentraEDR focuses on behavior: it correlates ETW telemetry across process, registry, network, and PowerShell events to detect RATs, stealers, and fileless attacks that signature engines miss.",
  },
  {
    question: "How much memory and CPU does the agent use?",
    answer:
      "The agent targets <150MB RAM at idle, with minimal CPU usage. It uses bounded channels, multi-runtime Tokio isolation, and zero-copy serialization to keep the hot path allocation-free even under heavy event load.",
  },
  {
    question: "Is the detection engine extensible?",
    answer:
      "Yes. Detections live in the engine-detection crate with a clean heuristic + correlation API. Future expansions include YARA rules (engine-yara), ML anomaly scoring (engine-ml), and a kernel-mode collector (engine-kernel).",
  },
  {
    question: "How does remediation stay safe?",
    answer:
      "Every action is reversible. Suspect processes are suspended and isolated before any deletion. Registry changes are backed up; quarantined files can be restored. Critical remediation requires multi-signal correlation and a confidence score >= 85.",
  },
  {
    question: "How do you test detections without real malware?",
    answer:
      "We rely on Atomic Red Team, MITRE ATT&CK simulations, EICAR, and controlled VM lab scenarios. The platform ships with a telemetry replay harness so detections can be re-run deterministically against captured ETW traces.",
  },
]

export function FAQSection() {
  return (
    <section id="faq" className="py-32 px-6 pb-80">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-normal mb-6 text-balance font-serif">Frequently asked questions</h2>
          <p className="text-muted-foreground max-w-2xl mx-auto leading-relaxed">
            Everything you need to know about SentraEDR. Have a question not listed? Reach out to the team.
          </p>
        </div>

        <Accordion type="single" collapsible className="space-y-3 py-0 my-0">
          {faqs.map((faq, index) => (
            <AccordionItem
              key={index}
              value={`item-${index}`}
              className="bg-card border border-border rounded-xl px-6 data-[state=open]:border-foreground/30"
            >
              <AccordionTrigger className="text-left text-base font-medium text-foreground hover:no-underline py-5">
                {faq.question}
              </AccordionTrigger>
              <AccordionContent className="text-muted-foreground pb-5 leading-relaxed text-sm">
                {faq.answer}
              </AccordionContent>
            </AccordionItem>
          ))}
        </Accordion>
      </div>
    </section>
  )
}
