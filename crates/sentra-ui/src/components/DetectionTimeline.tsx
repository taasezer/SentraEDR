import React from "react";

export function DetectionTimeline() {
  const mockDetections = [
    { id: 1, time: "10:45:22", type: "Process Injection", desc: "Suspicious remote thread created in explorer.exe", severity: "Critical" },
    { id: 2, time: "10:42:15", type: "Network Anomaly", desc: "Outbound connection to known C2 server", severity: "High" },
    { id: 3, time: "10:38:05", type: "Persistence", desc: "New startup registry key added", severity: "Medium" },
  ];

  return (
    <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 h-full flex flex-col">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-semibold text-white">Detection Timeline</h2>
        <span className="bg-red-500/10 text-red-400 text-xs font-bold px-3 py-1 rounded-full animate-pulse">
          3 Active Alerts
        </span>
      </div>
      
      <div className="flex-1 overflow-auto space-y-4 pr-2">
        {mockDetections.map((alert) => (
          <div key={alert.id} className="relative pl-6 border-l-2 border-slate-700 pb-2 last:pb-0">
            {/* Timeline Dot */}
            <div className={`absolute -left-[9px] top-1 w-4 h-4 rounded-full border-4 border-[#1e293b] ${
              alert.severity === 'Critical' ? 'bg-red-500' :
              alert.severity === 'High' ? 'bg-orange-500' : 'bg-amber-500'
            }`}></div>
            
            <div className="bg-slate-800/40 rounded-lg p-4 border border-slate-700/50 hover:border-slate-600 transition-colors">
              <div className="flex justify-between items-start mb-2">
                <span className="text-white font-medium">{alert.type}</span>
                <span className="text-slate-500 text-xs">{alert.time}</span>
              </div>
              <p className="text-slate-400 text-sm leading-relaxed">
                {alert.desc}
              </p>
              <div className="mt-3 flex gap-2">
                <button className="px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-white text-xs rounded transition-colors">
                  Investigate
                </button>
                <button className="px-3 py-1.5 bg-red-500/20 hover:bg-red-500/30 text-red-400 text-xs rounded transition-colors">
                  Block Process
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
