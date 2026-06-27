export function ProcessMonitor({ processes }: { processes: any[] }) {
  // Use real data if available, otherwise fallback to empty state
  const displayProcesses = processes && processes.length > 0 
    ? processes.slice(0, 50) // Show top 50
    : [];

  return (
    <div className="bg-[#1e293b] border border-slate-700/50 rounded-xl p-6 h-full flex flex-col">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold text-white">Process Monitor</h2>
        <span className="text-sm text-slate-400">Showing {displayProcesses.length} active processes</span>
      </div>
      <div className="flex-1 overflow-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-slate-700/50 text-slate-400 text-sm">
              <th className="pb-3 font-medium">PID</th>
              <th className="pb-3 font-medium">Process Name</th>
              <th className="pb-3 font-medium">User</th>
              <th className="pb-3 font-medium">Integrity</th>
            </tr>
          </thead>
          <tbody>
            {displayProcesses.length === 0 ? (
              <tr>
                <td colSpan={4} className="py-8 text-center text-slate-500">
                  Waiting for process telemetry...
                </td>
              </tr>
            ) : (
              displayProcesses.map((proc) => (
                <tr key={proc.pid} className="border-b border-slate-700/20 hover:bg-slate-800/50 transition-colors">
                  <td className="py-2 text-slate-300 text-sm">{proc.pid}</td>
                  <td className="py-2 text-white font-medium text-sm">{proc.name}</td>
                  <td className="py-2 text-slate-400 text-sm">{proc.user || "UNKNOWN"}</td>
                  <td className="py-2 text-slate-400 text-sm">
                    <span className="px-2 py-1 rounded-md text-[10px] uppercase tracking-wider bg-slate-800 border border-slate-700">
                      {proc.integrity_level}
                    </span>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
