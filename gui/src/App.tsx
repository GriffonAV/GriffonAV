import { ThemeProvider } from "@/components/theme-provider";
import { TitleBar } from "./components/title-bar";
import { Button } from "./components/ui/button";

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <div className="flex h-screen flex-col">
        <TitleBar />
        <div className="flex-1 flex flex-row">
          <div className="flex flex-col w-48 p-4">
            {/* <div className="mt-6"></div>

            <div className="mt-2 flex bg-sidebar-accent rounded-none h-10 items-center">
              <div className="bg-red-500 w-2 h-full mr-6"></div>
              <span>extensions</span>
            </div>
            <div className="mt-2 flex h-8 items-center">
              <div className="w-2 h-full mr-6"></div>
              <span>extensions</span>
            </div>
            <div className="mt-2 flex h-10 items-center">
              <div className="w-2 h-full mr-6"></div>
              <span>extensions</span>
            </div> */}
            <span className="mt-6 mb-2 px-4 text-xs text-muted-foreground uppercase">
              Plugins
            </span>
            <Button
              className="cursor-pointer mb-2"
              variant={"ghost"}
            >
              Analyser
            </Button>
          </div>
          <div className="flex h-full w-full border p-4 m-4 radius-0">
            content
          </div>
        </div>
        {/* <ModeToggle /> */}
      </div>
    </ThemeProvider>
  );
}

export default App;
