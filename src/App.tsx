// 需要异步执行
// import { invoke } from "@tauri-apps/api/tauri";
import { useMemo, useState } from "react";
import "./App.css";
import { MainLayout } from "./components/Layout";
import { VersionList } from "./pages/Versions";
import { PageType } from "./types";
import { PageContextProvider } from "./contexts/PageContext";
import { ProjectList } from "./pages/Projects";
import { ProjectFrom } from "./pages/ProjectFrom";

function App() {
    const [pageType, updatePageType] = useState<PageType>("Version");

    const contextValue = useMemo(() => {
        return {
            pageType,
            updatePageType,
        };
    }, [pageType, updatePageType]);

    return (
        <PageContextProvider value={contextValue}>
            <MainLayout>
                {"Version" === pageType && <VersionList />}
                {"Project" === pageType && <ProjectList />}
                {"ProjectFrom" === pageType && <ProjectFrom />}
            </MainLayout>
        </PageContextProvider>
    );
}

export default App;
