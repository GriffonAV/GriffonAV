import { ThemeProvider } from "@/components/theme-provider";
import { TitleBar } from "./components/title-bar";

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <div className="flex h-screen flex-col">
        <TitleBar />
        <div className="flex-1 flex flex-row">
          <div className="flex flex-col w-48">
            <div className="mt-6"></div>

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
            </div>
          </div>
          <div className="flex h-full w-full bg-secondary text-secondary-foreground p-4 radius-0">
            content
          </div>
        </div>
        {/* <ModeToggle /> */}
      </div>
    </ThemeProvider>
  );
}

export default App;
