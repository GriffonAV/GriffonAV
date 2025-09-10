import { ThemeProvider } from "@/components/theme-provider";
import { TitleBar } from "./components/title-bar";

function App() {
    return (
        <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
            <div className="flex h-screen flex-col">
                <TitleBar />
                <div className="flex-1">content</div>
                {/* <ModeToggle /> */}
            </div>
        </ThemeProvider>
    );
}

export default App;
