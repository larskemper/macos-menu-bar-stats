import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface SystemStats {
    cpu_usage: number;
    memory_used: number;
    memory_total: number;
    memory_percent: number;
    battery_percent: number;
    battery_state: string;
}

function App() {
    const [stats, setStats] = useState<SystemStats | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const fetchStats = async () => {
        try {
            const result = await invoke<SystemStats>("get_system_stats");
            setStats(result);
            setError(null);
        } catch (err) {
            setError(err as string);
            console.error("Failed to fetch stats:", err);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchStats();
        const interval = setInterval(fetchStats, 2000);
        return () => clearInterval(interval);
    }, []);

    const formatBytes = (bytes: number) => {
        const gb = bytes / (1024 * 1024 * 1024);
        return `${gb.toFixed(2)} GB`;
    };

    const getBatteryIcon = (percent: number, state: string) => {
        if (state.includes("Charging")) return "⚡";
        if (percent > 75) return "🔋";
        if (percent > 50) return "🔋";
        if (percent > 25) return "🪫";
        return "🪫";
    };

    if (loading) {
        return (
            <div className="container">
                <div className="loading">Loading...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="container">
                <div className="error">Error: {error}</div>
            </div>
        );
    }

    return (
        <div className="container">
            <div className="stats-grid">
                <div className="stat-card battery">
                    <div className="stat-icon">{stats && getBatteryIcon(stats.battery_percent, stats.battery_state)}</div>
                    <div className="stat-content">
                        <div className="stat-label">Battery</div>
                        <div className="stat-value">{stats?.battery_percent.toFixed(0)}%</div>
                        <div className="stat-sub">{stats?.battery_state}</div>
                    </div>
                </div>

                <div className="stat-card cpu">
                    <div className="stat-icon">🧠</div>
                    <div className="stat-content">
                        <div className="stat-label">CPU Usage</div>
                        <div className="stat-value">{stats?.cpu_usage.toFixed(1)}%</div>
                        <div className="progress-bar">
                            <div 
                                className="progress-fill cpu-fill" 
                                style={{ width: `${stats?.cpu_usage}%` }}
                            ></div>
                        </div>
                    </div>
                </div>

                <div className="stat-card memory">
                    <div className="stat-icon">🧠</div>
                    <div className="stat-content">
                        <div className="stat-label">Memory</div>
                        <div className="stat-value">{stats?.memory_percent.toFixed(1)}%</div>
                        <div className="stat-sub">
                            {stats && formatBytes(stats.memory_used)} / {stats && formatBytes(stats.memory_total)}
                        </div>
                        <div className="progress-bar">
                            <div 
                                className="progress-fill memory-fill" 
                                style={{ width: `${stats?.memory_percent}%` }}
                            ></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default App;