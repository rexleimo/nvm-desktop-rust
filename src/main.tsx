import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
