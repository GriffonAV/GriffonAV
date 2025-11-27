import { ThemeProvider } from "@/components/theme-provider";
import { TitleBar } from "./components/title-bar";
import { Button } from "./components/ui/button";
import { usePlugins } from "./hooks/usePlugins"

function App() {
  const { plugins, active, setActive } = usePlugins()

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <div className="flex h-screen flex-col">
        <TitleBar />
        <div className="flex-1 flex flex-row">
          <div className="flex flex-col w-48 p-4">
            <span className="mt-6 mb-2 px-4 text-xs text-muted-foreground uppercase">
              Plugins
            </span>
            {plugins.map((plugin) => (
              <Button
                key={plugin.pid}
                className="mb-2 justify-start"
                variant={active === plugin.pid ? "default" : "ghost"}
                onClick={() => setActive(plugin.pid)}
              >
                {plugin.name}
              </Button>
            ))}
            <span className="mt-6 mb-2 px-4 text-xs text-muted-foreground uppercase">
              Griffon 
            </span>
            <Button
                className="mb-2 justify-start"
                variant={"ghost"}
                onClick={() => window.open("https://griffonav.com", "_blank")}
              >
                Visit Website
              </Button>
            <Button
                className="mb-2 justify-start"
                variant={"ghost"}
              >
                Settings
              </Button>
          </div>
          <div className="flex h-full w-full border p-4 m-4 radius-0">
            {active !== null ? (
              <>Selected Plugin PID: {active}</>
            ) : (
              "No plugin selected"
            )}
          </div>



        </div>
        {/* <ModeToggle /> */}
      </div>
    </ThemeProvider>
  );
}

export default App;
