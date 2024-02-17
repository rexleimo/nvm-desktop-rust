import cn from "classnames";
import { Button } from "@radix-ui/themes";
import { useContext } from "react";
import PageContext from "../../contexts/PageContext";
import { useTranslation } from "react-i18next";

function ProjectAdd() {
    const { updatePageType } = useContext(PageContext);
    const { t } = useTranslation();
    return (
        <>
            <div className={cn("text-right")}>
                <Button onClick={() => updatePageType?.("ProjectFrom")}>
                    {t("add_project_txt")}
                </Button>
            </div>
        </>
    );
}

export default ProjectAdd;
