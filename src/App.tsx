// 需要异步执行
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { MainLayout } from "./components/Layout";
import { VersionList } from "./pages/Versions";

function App() {
    return (
        <MainLayout>
            新年快乐
            <VersionList />
        </MainLayout>
    );
}

export default App;
