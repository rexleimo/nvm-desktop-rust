import { LogSelectProps } from "./types";
import { useMount } from "ahooks";
import { event } from "@tauri-apps/api";
import { useRef } from "react";

function LogSelect(props: LogSelectProps) {
    const {} = props;
    const logDivRef = useRef<HTMLDivElement>(null);
    // 监听日志选择事件
    useMount(() => {
        event.listen("logs_event", (event: any) => {
            const { payload } = event;
            const p = document.createElement("p");
            p.innerText = payload.message;
            logDivRef.current?.appendChild(p);
        });
    });
    return <div ref={logDivRef}></div>;
}

export default LogSelect;
