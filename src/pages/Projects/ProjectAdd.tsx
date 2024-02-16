import cn from "classnames";
import { Button } from "@radix-ui/themes";
import { useContext } from "react";
import PageContext from "../../contexts/PageContext";

function ProjectAdd() {
    const { updatePageType } = useContext(PageContext);

    return (
        <>
            <div className={cn("text-right")}>
                <Button onClick={() => updatePageType?.("ProjectFrom")}>
                    添加项目
                </Button>
            </div>
        </>
    );
}

export default ProjectAdd;
