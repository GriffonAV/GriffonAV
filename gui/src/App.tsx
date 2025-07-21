import { useState } from "react";
import { Button } from "@/components/ui/button";
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
            // Replace with the actual path or let user input it
        } catch (err) {
            setError(String(err));
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex min-h-svh flex-col items-center justify-center p-4">
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
