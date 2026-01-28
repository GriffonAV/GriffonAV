import { Link, useLocation } from "react-router-dom";
import { usePlugins } from "@/hooks/usePlugins";
import { Button } from "@/components/ui/button";
import clsx from "clsx";

export function Sidebar() {
  const { plugins } = usePlugins();
    const location = useLocation();


  return (
    <aside className="w-48 p-4 space-y-2">
      <span className="text-xs text-muted-foreground uppercase px-2">
        Plugins
      </span>

      {plugins.map((plugin) => {
        const isActive = location.pathname === `/plugin/${plugin.pid}`;

        return (
          <Link key={plugin.pid} to={`/plugin/${plugin.pid}`}>
            <Button
              variant="ghost"
              className={clsx(
                "w-full justify-start cursor-pointer capitalize",
                // Hover UI (applies always)
                "hover:bg-accent hover:text-accent-foreground",
                // Active UI
                isActive &&
                  "bg-accent text-accent-foreground font-medium"
              )}
            >
              {plugin.name}
            </Button>
          </Link>
        );
      })}

      <span className="text-xs text-muted-foreground uppercase px-2 mt-6 block">
        App
      </span>

      {/* Settings example */}
      <Link to="/settings">
        <Button
          variant="ghost"
          className={clsx(
            "w-full justify-start cursor-pointer capitalize",
            "hover:bg-accent hover:text-accent-foreground",
            location.pathname === "/settings" &&
              "bg-accent text-accent-foreground font-medium"
          )}
        >
          Settings
        </Button>
      </Link>
    </aside>
  );
}
