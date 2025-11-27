import { invoke } from "@tauri-apps/api/core"
import { useEffect, useState } from "react"

export type PluginInfo = {
  pid: number
  name: string
}

export function usePlugins() {
  const [plugins, setPlugins] = useState<PluginInfo[]>([])
  const [active, setActive] = useState<number | null>(null)

  useEffect(() => {
    invoke<PluginInfo[]>("get_plugins").then((list) => {
      setPlugins(list)
      if (list.length > 0) setActive(list[0].pid)
    })
  }, [])

  return { plugins, active, setActive }
}
