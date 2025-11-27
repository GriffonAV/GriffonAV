import { useParams } from "react-router-dom";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";

export default function PluginPage() {
  const { pid } = useParams();
  const [logs, setLogs] = useState<string[]>([]);
  const [running, setRunning] = useState(false);

  function startAnalysis() {
    if (running) return;

    setRunning(true);
    setLogs(["Starting analysis…"]);

    // Fake analysis simulation
    const steps = [
      "Loading target data…",
      "Analyzing structure…",
      "Detecting anomalies…",
      "Generating report…",
      "Analysis complete! ✔",
    ];

    let i = 0;
    const interval = setInterval(() => {
      setLogs((prev) => [...prev, steps[i]]);
      i++;

      if (i >= steps.length) {
        clearInterval(interval);
        setRunning(false);
      }
    }, 800);
  }

  return (
    <div className="flex flex-col h-full gap-4">
      <h1 className="text-lg font-semibold">
        Plugin #{pid} — Analyser
      </h1>

      <Button
        onClick={startAnalysis}
        disabled={running}
        className="w-fit"
      >
        {running ? "Running..." : "Start Analysis"}
      </Button>

      <Card className="flex-1 p-3 bg-black/60 text-green-400 font-mono text-sm overflow-auto border">
        {logs.length === 0 ? (
          <span className="opacity-50">No logs yet…</span>
        ) : (
          logs.map((log, i) => (
            <div key={i} className="whitespace-pre-wrap">
              $ {log}
            </div>
          ))
        )}
      </Card>
    </div>
  );
}
