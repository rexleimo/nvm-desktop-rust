import { MainToolBarProps } from "./types";
import * as NavigationMenu from "@radix-ui/react-navigation-menu";
import cn from "classnames";

function MainToolBar(props: MainToolBarProps) {
    return (
        <NavigationMenu.Root className={cn("h-full",cn("border-b-[1px]"))}>
            <NavigationMenu.List className={cn("flex items-center")}>
                <NavigationMenu.Item className={cn("p-[8px_16px]")}>
                    版本列表
                </NavigationMenu.Item>
            </NavigationMenu.List>
        </NavigationMenu.Root>
    );
}

export default MainToolBar;
