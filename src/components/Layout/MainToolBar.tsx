import { MainToolBarProps } from "./types";
import * as NavigationMenu from "@radix-ui/react-navigation-menu";
import cn from "classnames";
import { useCallback, useContext } from "react";
import { PageType } from "../../types";
import PageContext from "../../contexts/PageContext";
import { useTranslation } from "react-i18next";
import { DropdownMenu } from "../DropdownMenu";

function MainToolBar(props: MainToolBarProps) {
    const {} = props;
    const { updatePageType, updateLocale } = useContext(PageContext);

    const { t } = useTranslation();

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
                    {t("version_manage")}
                </NavigationMenu.Item>
                <NavigationMenu.Item
                    onClick={() => handleChangePageType("Project")}
                    className={cn(
                        "p-[8px_16px]",
                        cn("text-[12px]"),
                        cn("cursor-pointer")
                    )}
                >
                    {t("project_manage")}
                </NavigationMenu.Item>
                <NavigationMenu.Item
                    onClick={() => handleChangePageType("LogSelect")}
                    className={cn(
                        "p-[8px_16px]",
                        cn("text-[12px]"),
                        cn("cursor-pointer")
                    )}
                >
                    {t("log_manage")}
                </NavigationMenu.Item>

                <NavigationMenu.Item
                    className={cn(
                        "p-[8px_16px]",
                        cn("text-[12px]"),
                        cn("cursor-pointer")
                    )}
                >
                    <DropdownMenu
                        items={["English", "简体中文"]}
                        onClick={(item) => {
                            if ("English" === item) {
                                updateLocale?.("en");
                                return;
                            }

                            if ("简体中文" === item) {
                                updateLocale?.("zh");
                                return;
                            }
                        }}
                    >
                        <span>{t("locales_txt")}</span>
                    </DropdownMenu>
                </NavigationMenu.Item>
            </NavigationMenu.List>
        </NavigationMenu.Root>
    );
}

export default MainToolBar;
