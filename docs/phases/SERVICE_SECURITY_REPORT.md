# Service Security Report

## Privilege Minimization
The `PrivilegeValidator` enforces that `service-host` runs at `SYSTEM` integrity but immediately drops unnecessary privileges across child worker threads, honoring the minimum privilege principle.

## Permission Boundaries
- Named Pipe ACLs restrict read/write exclusively to the Local System and the local Administrators group.
- Registry Keys for configuration updates enforce read-only access for Users.
