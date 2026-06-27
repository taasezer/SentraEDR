import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [health, setHealth] = useState<any>(null);

  useEffect(() => {
    // In a real implementation we would poll or listen to IPC events
    // For now we just mock an initial health fetch
    const fetchHealth = async () => {
      try {
        const res = await invoke("get_health_status");
        setHealth(res);
      } catch (err) {
        console.error("Failed to fetch health:", err);
      }
    };
    fetchHealth();
  }, []);

  return (
    <div className="flex h-screen bg-[#0f172a] text-white">
      {/* Sidebar */}
      <div className="w-64 bg-[#1e293b] border-r border-slate-700 p-6 flex flex-col">
        <div className="flex items-center gap-3 mb-10">
          <div className="w-8 h-8 rounded bg-[#00ff9d] flex items-center justify-center text-slate-900 font-bold">
            S
          </div>
          <h1 className="text-xl font-bold tracking-wider">SentraEDR</h1>
        </div>

        <nav className="flex-1 space-y-4">
          <a href="#" className="block px-4 py-3 rounded-lg bg-slate-700/50 text-[#00ff9d] font-medium border border-[#00ff9d]/30 shadow-[0_0_15px_rgba(0,255,157,0.1)]">
            Dashboard
          </a>
          <a href="#" className="block px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 transition-colors">
            Processes
          </a>
          <a href="#" className="block px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 transition-colors">
            Network
          </a>
          <a href="#" className="block px-4 py-3 rounded-lg text-slate-400 hover:bg-slate-800 transition-colors">
            Detections
          </a>
        </nav>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Topbar */}
        <header className="h-20 border-b border-slate-800 flex items-center justify-between px-8 bg-[#0f172a]/80 backdrop-blur-md">
          <h2 className="text-2xl font-semibold">System Overview</h2>
          <div className="flex items-center gap-4">
            <div className="px-4 py-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-sm font-medium flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
              Engine Active
            </div>
          </div>
        </header>

        {/* Dashboard Content */}
        <main className="flex-1 p-8 overflow-y-auto space-y-6">
          
          {/* Stats Grid */}
          <div className="grid grid-cols-4 gap-6">
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-[#00ff9d]/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">Active Processes</p>
              <p className="text-3xl font-bold text-white relative z-10">142</p>
            </div>
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-blue-500/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">Network Connections</p>
              <p className="text-3xl font-bold text-white relative z-10">89</p>
            </div>
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-red-500/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">Blocked Threats</p>
              <p className="text-3xl font-bold text-red-400 relative z-10">3</p>
            </div>
            <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 relative overflow-hidden group">
              <div className="absolute top-0 right-0 w-32 h-32 bg-purple-500/5 rounded-full blur-2xl -mr-10 -mt-10 transition-transform group-hover:scale-150"></div>
              <p className="text-slate-400 text-sm mb-2 relative z-10">Memory Usage</p>
              <p className="text-3xl font-bold text-white relative z-10">48 MB</p>
            </div>
          </div>

          {/* Activity Feed */}
          <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6">
            <h3 className="text-lg font-semibold mb-4">Recent Detections</h3>
            <div className="space-y-4">
              <div className="flex items-center gap-4 p-4 rounded-lg bg-slate-800/50 border border-slate-700/50">
                <div className="w-10 h-10 rounded-full bg-red-500/10 flex items-center justify-center text-red-400 border border-red-500/20">
                  !
                </div>
                <div className="flex-1">
                  <h4 className="text-red-400 font-medium">Suspicious PowerShell Execution</h4>
                  <p className="text-sm text-slate-400">powershell.exe -enc JABzAD0ATgBlAHcALQBPAGIAagBlAGMAdAAgAEkATwAuAE0AZQBtAG8AcgB5AFMAdAByAGUAYQBtACgAWwBDAG8AbgB2AGUAcgB0AF0AOgA...</p>
                </div>
                <div className="text-right">
                  <span className="text-xs text-slate-500">2 mins ago</span>
                  <div className="text-emerald-400 text-sm font-medium mt-1">Blocked & Quarantined</div>
                </div>
              </div>
              <div className="flex items-center gap-4 p-4 rounded-lg bg-slate-800/50 border border-slate-700/50">
                <div className="w-10 h-10 rounded-full bg-yellow-500/10 flex items-center justify-center text-yellow-400 border border-yellow-500/20">
                  !
                </div>
                <div className="flex-1">
                  <h4 className="text-yellow-400 font-medium">Unsigned Binary Network Connection</h4>
                  <p className="text-sm text-slate-400">C:\Users\Public\svchost.exe connecting to 185.112.x.x:443</p>
                </div>
                <div className="text-right">
                  <span className="text-xs text-slate-500">15 mins ago</span>
                  <div className="text-slate-400 text-sm font-medium mt-1">Process Suspended</div>
                </div>
              </div>
            </div>
          </div>
          
        </main>
      </div>
    </div>
  );
}

export default App;
