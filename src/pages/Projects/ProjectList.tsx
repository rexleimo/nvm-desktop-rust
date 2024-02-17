import { useCallback, useState } from "react";
import ProjectAdd from "./ProjectAdd";
import { ProjectListProps } from "./types";
import { AgGridReact } from "ag-grid-react";
import { useMount } from "ahooks";
import { invoke } from "@tauri-apps/api";
import { Button } from "@radix-ui/themes";
import cn from "classnames";
import { PlayIcon, PauseIcon } from "@radix-ui/react-icons";
import MoreMenu from "./MoreMenu";
import { useTranslation } from "react-i18next";

function ProjectList(props: ProjectListProps) {
    const {} = props;
    const [projectList, updateProjectList] = useState<any[]>([]);
    const { t } = useTranslation();

    useMount(() => {
        invoke("get_project_list").then((res: any) => {
            if (res) {
                updateProjectList(res);
            }
        });
    });

    const startProject = useCallback((projectName: string) => {
        invoke("run_project", { projectName }).then((res) => {
            console.log(res);
        });
    }, []);

    const stopProject = useCallback((projectName: string) => {
        invoke("stop_project", { projectName }).then((res) => {
            console.log(res);
        });
    }, []);

    return (
        <div className='ag-theme-quartz h-full'>
            <ProjectAdd />
            <AgGridReact
                className='mt-4'
                domLayout='autoHeight'
                rowData={projectList}
                columnDefs={[
                    {
                        field: "name",
                        headerName: t("project_txt"),
                        editable: false,
                        width: 200,
                    },
                    {
                        field: "dir",
                        headerName: t("project_dir_txt"),
                        editable: false,
                        width: 400,
                    },
                    {
                        headerName: t("project_options"),
                        cellClass: cn("flex items-center"),
                        cellRenderer: (props: any) => {
                            return (
                                <div
                                    className={cn(
                                        "flex",
                                        "gap-2",
                                        "items-center"
                                    )}
                                >
                                    <Button
                                        color='green'
                                        className='cursor-pointer'
                                        onClick={() =>
                                            startProject(props.data.name)
                                        }
                                    >
                                        <PlayIcon />
                                    </Button>
                                    <Button
                                        color='red'
                                        className='cursor-pointer'
                                        onClick={() =>
                                            stopProject(props.data.name)
                                        }
                                    >
                                        <PauseIcon />
                                    </Button>
                                    <MoreMenu
                                        projectId={props.data.id}
                                        projectName={props.data.name}
                                        updateProjectList={updateProjectList}
                                    />
                                </div>
                            );
                        },
                    },
                ]}
            />
        </div>
    );
}

export default ProjectList;
