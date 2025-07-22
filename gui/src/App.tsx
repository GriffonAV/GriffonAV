import { useState } from "react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [scanResult, setScanResult] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleScan = async () => {
        setLoading(true);
        setError(null);
        setScanResult(null);

        try {
            const pathToScan = "/home/file.txt";

            // Call Rust backend command via Tauri invoke
            const result = await invoke<string>("scan_from_gui", {
                path: pathToScan,
            });

            setScanResult(result);
        } catch (err) {
            setError(String(err));
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex min-h-svh flex-col items-center justify-center p-4">
            <img
                src="/assets/logo.png"
                alt="Griffon Logo"
                style={{
                    imageRendering: "pixelated",
                }}
                className="w-32 h-auto"
            />

            <Button onClick={handleScan} disabled={loading}>
                {loading ? "Scanning..." : "Start Scan"}
            </Button>

            {scanResult && (
                <p className="mt-4 text-green-600">Scan result: {scanResult}</p>
            )}

            {error && <p className="mt-4 text-red-600">Error: {error}</p>}
        </div>
    );
}

export default App;
