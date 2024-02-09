import React from "react";
import { MainLayoutProps } from "./types";
import cn from "classnames";
import MainToolBar from "./MainToolBar";
function MainLayout(props: MainLayoutProps) {
    const { children } = props;
    return (
        <div className={cn("h-full", cn("relative"), cn("flex", "flex-col"))}>
            <div className={cn("relative", "w-full")}>
                <MainToolBar />
            </div>
            <div className={cn("flex-auto", cn("p-[8px_12px]"))}>
                {children}
            </div>
        </div>
    );
}

export default MainLayout;
