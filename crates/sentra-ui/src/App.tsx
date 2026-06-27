import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { ProcessMonitor } from "./components/ProcessMonitor";
import { DetectionTimeline } from "./components/DetectionTimeline";
import "./App.css";

function App() {
  const [health, setHealth] = useState<any>(null);
  const [processes, setProcesses] = useState<any[]>([]);

  useEffect(() => {
    // Listen for IPC messages from the Rust backend
    const unlisten = listen("ipc-message", (event: any) => {
      const msg = event.payload;
      if (msg && typeof msg === "object") {
        if ("HealthResponse" in msg) {
          setHealth(msg.HealthResponse);
        } else if ("ProcessList" in msg) {
          setProcesses(msg.ProcessList);
        }
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="min-h-screen bg-[#0f172a] text-slate-200 font-sans selection:bg-[#00ff9d]/30 flex flex-col">
      {/* Top Navigation Bar */}
      <nav className="h-16 bg-[#1e293b]/80 backdrop-blur-md border-b border-slate-700/50 flex items-center justify-between px-6 sticky top-0 z-50">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-[#00ff9d] to-emerald-600 shadow-[0_0_15px_rgba(0,255,157,0.4)] flex items-center justify-center">
            <svg className="w-5 h-5 text-[#0f172a]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
            </svg>
          </div>
          <div>
            <h1 className="text-xl font-bold text-white tracking-tight">Sentra<span className="text-[#00ff9d]">EDR</span></h1>
            <p className="text-[10px] text-slate-400 font-medium uppercase tracking-wider -mt-1">Security Platform</p>
          </div>
        </div>
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-slate-800/50 border border-slate-700">
            <div className={`w-2 h-2 rounded-full ${health ? 'bg-[#00ff9d] shadow-[0_0_8px_#00ff9d]' : 'bg-red-500 shadow-[0_0_8px_#ef4444]'}`}></div>
            <span className="text-sm font-medium">{health ? 'Engine Active' : 'Disconnected'}</span>
          </div>
        </div>
      </nav>

      {/* Main Content Dashboard */}
      <main className="flex-1 p-6">
        <div className="max-w-7xl mx-auto space-y-6">
          
          {/* Stats Grid */}
          <div className="grid grid-cols-4 gap-6">
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-[#00ff9d]/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">CPU Usage</p>
              <p className="text-3xl font-bold text-white relative z-10">{health ? `${health.cpu_usage.toFixed(1)}%` : "..."}</p>
            </div>
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-blue-500/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">Events / Sec</p>
              <p className="text-3xl font-bold text-white relative z-10">{health ? Math.floor(health.events_per_second) : "..."}</p>
            </div>
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-red-500/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">Blocked Threats</p>
              <p className="text-3xl font-bold text-red-400 relative z-10">3</p>
            </div>
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-purple-500/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">Memory Usage</p>
              <p className="text-3xl font-bold text-white relative z-10">{health ? `${health.memory_usage_mb.toFixed(1)} MB` : "..."}</p>
            </div>
          </div>

          {/* Main Workspace Area */}
          <div className="grid grid-cols-3 gap-6 h-[500px]">
            {/* Process Monitor (Left/Center 2 cols) */}
            <div className="col-span-2">
              <ProcessMonitor processes={processes} />
            </div>
            
            {/* Detection Timeline (Right col) */}
            <div className="col-span-1">
              <DetectionTimeline />
            </div>
          </div>

        </div>
      </main>
    </div>
  );
}

export default App;
