# UI Performance Budget

## Limits
- **Startup Time:** < 500ms to first paint.
- **Idle Memory (Frontend WebView):** < 50MB RSS.
- **Maximum Event Rendering Rate:** Cap rendering updates to 60fps (requestAnimationFrame boundary). Backend events exceeding this are dropped or aggregated.
- **Maximum CPU Usage (Live Monitoring):** < 5% across a single core during heavy telemetry aggregation rendering.
