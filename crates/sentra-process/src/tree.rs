use sentra_core::ProcessInfo;
use std::collections::{HashMap, HashSet};

pub struct ProcessTree {
    parent_to_children: HashMap<u32, Vec<u32>>,
    pid_to_ppid: HashMap<u32, u32>,
}

impl ProcessTree {
    pub fn build(processes: &[ProcessInfo]) -> Self {
        let mut parent_to_children: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut pid_to_ppid: HashMap<u32, u32> = HashMap::new();

        for p in processes {
            pid_to_ppid.insert(p.pid, p.ppid);
            parent_to_children.entry(p.ppid).or_default().push(p.pid);
        }

        Self {
            parent_to_children,
            pid_to_ppid,
        }
    }

    pub fn children_of(&self, pid: u32) -> Vec<u32> {
        self.parent_to_children.get(&pid).cloned().unwrap_or_default()
    }

    pub fn ancestors_of(&self, pid: u32) -> Vec<u32> {
        let mut ancestors = Vec::new();
        let mut current_pid = pid;

        let mut seen = HashSet::new();

        while let Some(&ppid) = self.pid_to_ppid.get(&current_pid) {
            if ppid == 0 || seen.contains(&ppid) {
                break;
            }
            ancestors.push(ppid);
            seen.insert(ppid);
            current_pid = ppid;
        }

        ancestors
    }

    pub fn is_suspicious_parent_child(parent: &ProcessInfo, child: &ProcessInfo) -> bool {
        let p_name = parent.name.to_lowercase();
        let c_name = child.name.to_lowercase();

        if p_name == "svchost.exe" && (c_name == "cmd.exe" || c_name == "powershell.exe") {
            return true;
        }

        if p_name == "explorer.exe" && (c_name == "cscript.exe" || c_name == "wscript.exe" || c_name == "mshta.exe") {
            return true;
        }

        if (p_name == "winword.exe" || p_name == "excel.exe") && (c_name == "cmd.exe" || c_name == "powershell.exe") {
            return true;
        }

        if p_name == "wmiprvse.exe" && c_name != "unsecapp.exe" && c_name != "scrcons.exe" {
            // Very noisy, but good for demo
            return true;
        }

        false
    }
}
