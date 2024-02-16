import { MainToolBarProps } from "./types";
import * as NavigationMenu from "@radix-ui/react-navigation-menu";
import cn from "classnames";
import { useCallback, useContext } from "react";
import { PageType } from "../../types";
import PageContext from "../../contexts/PageContext";

function MainToolBar(props: MainToolBarProps) {
    const {} = props;
    const { updatePageType } = useContext(PageContext);
    const handleChangePageType = useCallback((pageType: PageType) => {
        updatePageType?.(pageType);
    }, []);

    return (
        <NavigationMenu.Root className={cn("h-full", cn("border-b-[1px]"))}>
            <NavigationMenu.List className={cn("flex items-center")}>
                <NavigationMenu.Item
                    onClick={() => handleChangePageType("Version")}
                    className={cn(
                        "p-[8px_16px]",
                        cn("text-[12px]"),
                        cn("cursor-pointer")
                    )}
                >
                    版本管理
                </NavigationMenu.Item>
                <NavigationMenu.Item
                    onClick={() => handleChangePageType("Project")}
                    className={cn(
                        "p-[8px_16px]",
                        cn("text-[12px]"),
                        cn("cursor-pointer")
                    )}
                >
                    项目管理
                </NavigationMenu.Item>
            </NavigationMenu.List>
        </NavigationMenu.Root>
    );
}

export default MainToolBar;
