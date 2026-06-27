# Production Deployment Guide

## Pre-requisites
- Windows 10/11 Enterprise or Windows Server 2019+
- Administrative privileges for MSI installation
- Exclusions applied to competing Antivirus software.

## Execution
Run the signed MSI installer across the enterprise fleet via SCCM, GPO, or Intune. The agent installs silently without requiring a system reboot and begins streaming Kernel ETW telemetry immediately.
