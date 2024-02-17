// 需要异步执行
// import { invoke } from "@tauri-apps/api/tauri";
import { useMemo, useState } from "react";
import "./App.css";
import { MainLayout } from "./components/Layout";
import { VersionList } from "./pages/Versions";
import { PageParams, PageType } from "./types";
import { PageContextProvider } from "./contexts/PageContext";
import { ProjectList } from "./pages/Projects";
import { ProjectFrom } from "./pages/ProjectFrom";
import { LogSelect } from "./pages/LogSelect";

function App() {
    const [pageType, updatePageType] = useState<PageType>("Version");
    const [pageParams, updatePageParams] = useState<PageParams>();

    const contextValue = useMemo(() => {
        return {
            pageType,
            updatePageType,
            pageParams,
            updatePageParams,
        };
    }, [pageType, updatePageType, pageParams, updatePageParams]);

    return (
        <PageContextProvider value={contextValue}>
            <MainLayout>
                {"Version" === pageType && <VersionList />}
                {"Project" === pageType && <ProjectList />}
                {"ProjectFrom" === pageType && <ProjectFrom />}
                <div
                    style={{
                        visibility:
                            "LogSelect" == pageType ? "visible" : "hidden",
                    }}
                >
                    <LogSelect />
                </div>
            </MainLayout>
        </PageContextProvider>
    );
}

export default App;
