import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core"

export interface Plugin {
  pid: number;
  name: string;
}

export function usePlugins() {
  const [plugins, setPlugins] = useState<Plugin[]>([]);

  async function refreshPlugins() {
    try {
      const result = await invoke<Plugin[]>("list_plugins_cmd");
      setPlugins(result);
    } catch (err) {
      console.error("Failed to load plugins:", err);
    }
  }

  useEffect(() => {
    refreshPlugins();
  }, []);

  return { plugins, refreshPlugins };
}
