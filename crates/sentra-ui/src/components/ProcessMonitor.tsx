import React from "react";

export function ProcessMonitor() {
  // Mock data for initial UI layout
  const mockProcesses = [
    { pid: 1404, name: "chrome.exe", user: "ASUS", integrity: "Medium", status: "Safe" },
    { pid: 4892, name: "powershell.exe", user: "SYSTEM", integrity: "High", status: "Suspicious" },
    { pid: 112, name: "svchost.exe", user: "SYSTEM", integrity: "System", status: "Safe" },
    { pid: 8832, name: "unknown_miner.exe", user: "ASUS", integrity: "Medium", status: "Malicious" },
  ];

  return (
    <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 h-full flex flex-col">
      <h2 className="text-xl font-semibold text-white mb-4">Process Monitor</h2>
      <div className="flex-1 overflow-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-slate-700/50 text-slate-400 text-sm">
              <th className="pb-3 font-medium">PID</th>
              <th className="pb-3 font-medium">Process Name</th>
              <th className="pb-3 font-medium">User</th>
              <th className="pb-3 font-medium">Integrity</th>
              <th className="pb-3 font-medium">Status</th>
            </tr>
          </thead>
          <tbody>
            {mockProcesses.map((proc) => (
              <tr key={proc.pid} className="border-b border-slate-700/20 hover:bg-slate-800/50 transition-colors">
                <td className="py-3 text-slate-300">{proc.pid}</td>
                <td className="py-3 text-white font-medium">{proc.name}</td>
                <td className="py-3 text-slate-400">{proc.user}</td>
                <td className="py-3 text-slate-400">
                  <span className="px-2 py-1 rounded-md text-xs bg-slate-800 border border-slate-700">
                    {proc.integrity}
                  </span>
                </td>
                <td className="py-3">
                  <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                    proc.status === 'Safe' ? 'bg-[#00ff9d]/10 text-[#00ff9d]' : 
                    proc.status === 'Suspicious' ? 'bg-amber-500/10 text-amber-500' : 
                    'bg-red-500/10 text-red-400'
                  }`}>
                    {proc.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
