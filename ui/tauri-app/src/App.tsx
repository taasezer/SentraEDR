import { useEffect, useState, useRef } from 'react';
import { Shield, Activity, AlertTriangle, Cpu, ShieldAlert } from 'lucide-react';
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
  const [isConnected, setIsConnected] = useState(false);
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    connectWebSocket();
    return () => {
      if (ws.current) {
        ws.current.close();
      }
    };
  }, []);

  const connectWebSocket = () => {
    const socket = new WebSocket('ws://127.0.0.1:8080/ws');

    socket.onopen = () => {
      setIsConnected(true);
      console.log('Connected to SentraEDR Detection Engine');
    };

    socket.onclose = () => {
      setIsConnected(false);
      // Try to reconnect every 3 seconds
      setTimeout(connectWebSocket, 3000);
    };

    socket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.alert_id) {
          // It's an alert object
          setAlerts((prev) => [data, ...prev]);
        }
      } catch (e) {
        // Just a normal text message (like the welcome message)
        console.log("Server message:", event.data);
      }
    };

    ws.current = socket;
  };

  return (
    <div className="dashboard-container">
      <header className="header">
        <div className="header-title">
          <Shield className="shield-icon" size={32} />
          SentraEDR Dashboard
        </div>
        <div className={`status-badge ${!isConnected ? 'disconnected' : ''}`}>
          <div className="status-dot"></div>
          {isConnected ? 'Engine Connected' : 'Disconnected - Retrying...'}
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
            <div className="stat-value" style={{ fontSize: '24px', display: 'flex', alignItems: 'center', gap: '8px' }}>
              <Cpu size={24} color="var(--accent-color)" />
              Active
            </div>
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
