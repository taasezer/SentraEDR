import { useEffect, useState } from 'react';
import { Activity, AlertTriangle, Cpu, ShieldAlert } from 'lucide-react';
import logoUrl from '../app-icon.png';
import './App.css';

interface Alert {
  alert_id: string;
  rule_id: string;
  severity: 'High' | 'Medium' | 'Low';
  confidence: 'High' | 'Medium' | 'Low';
  evidence: {
    reasoning_path: string;
    related_event_ids: string[];
  };
}

function App() {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [engineStatus, setEngineStatus] = useState<'Connecting' | 'Active' | 'Error'>('Connecting');
  const [errorMessage, setErrorMessage] = useState('');

  useEffect(() => {
    let unlistenAlerts: () => void;
    let unlistenStart: () => void;
    let unlistenError: () => void;

    const setupListeners = async () => {
      const { listen } = await import('@tauri-apps/api/event');
      const { invoke } = await import('@tauri-apps/api/core');

      const unlistAlert = await listen<string>('edr-alert', (event) => {
        try {
          const data = JSON.parse(event.payload);
          setAlerts((prev) => [data, ...prev]);
        } catch (e) {
          console.error("Failed to parse alert", e);
        }
      });
      unlistenAlerts = unlistAlert;

      const unlistStart = await listen<string>('engine-started', () => {
        setEngineStatus('Active');
      });
      unlistenStart = unlistStart;

      const unlistErr = await listen<string>('engine-error', (event) => {
        setEngineStatus('Error');
        setErrorMessage(event.payload);
      });
      unlistenError = unlistErr;

      // Start the engine after listeners are attached
      await invoke('start_engine');
    };

    setupListeners();

    return () => {
      if (unlistenAlerts) unlistenAlerts();
      if (unlistenStart) unlistenStart();
      if (unlistenError) unlistenError();
    };
  }, []);

  return (
    <div className="dashboard-container">
      <header className="header">
        <div className="header-title">
          <img src={logoUrl} alt="SentraEDR Logo" className="logo-image" />
          SentraEDR
        </div>
        <div className={`status-badge ${engineStatus === 'Error' ? 'disconnected' : ''}`}>
          <div className="status-dot"></div>
          {engineStatus === 'Active' ? 'Engine Active' : engineStatus === 'Error' ? 'Error: Run as Admin' : 'Initializing Engine...'}
        </div>
      </header>

      <main className="main-content">
        <aside className="stats-sidebar">
          <div className="stat-card">
            <div className="stat-title">Total Alerts</div>
            <div className="stat-value">{alerts.length}</div>
          </div>
          <div className="stat-card">
            <div className="stat-title">High Severity</div>
            <div className="stat-value" style={{ color: 'var(--danger-color)' }}>
              {alerts.filter(a => a.severity === 'High').length}
            </div>
          </div>
          <div className="stat-card">
            <div className="stat-title">Engine Status</div>
            <div className={`stat-value engine-status ${engineStatus === 'Active' ? 'active' : 'inactive'}`}>
              <Cpu size={24} color={engineStatus === 'Active' ? "var(--accent-color)" : engineStatus === 'Error' ? "var(--danger-color)" : "var(--text-secondary)"} />
              {engineStatus === 'Active' ? 'Active' : engineStatus === 'Error' ? 'Failed' : 'Starting'}
            </div>
            {errorMessage && <div style={{ fontSize: '11px', color: 'var(--danger-color)', marginTop: '4px' }}>{errorMessage}</div>}
          </div>
        </aside>

        <section className="feed-container">
          <div className="feed-header">
            <Activity size={20} />
            Live Threat Feed
          </div>
          <div className="feed-list">
            {alerts.length === 0 ? (
              <div className="empty-state">
                <ShieldAlert size={48} />
                <p>No threats detected yet. Engine is monitoring...</p>
              </div>
            ) : (
              alerts.map((alert) => (
                <div key={alert.alert_id} className="alert-card">
                  <div className="alert-header">
                    <div className="alert-title-container">
                      <AlertTriangle size={18} color="var(--danger-color)" />
                      <div className="alert-title">{alert.rule_id}</div>
                    </div>
                    <div className="alert-time">Just now</div>
                  </div>
                  <div className="alert-id">ID: {alert.alert_id}</div>
                  <div className="alert-body">
                    {alert.evidence.reasoning_path}
                  </div>
                  <div className="alert-metrics">
                    <div className="metric">
                      <span className="metric-label">Risk</span>
                      <span className={`metric-value ${alert.severity.toLowerCase()}`}>
                        {alert.severity}
                      </span>
                    </div>
                    <div className="metric">
                      <span className="metric-label">Confidence</span>
                      <span className={`metric-value ${alert.confidence.toLowerCase()}`}>
                        {alert.confidence}
                      </span>
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
