import { useCallback, useRef, useState } from "react";
import ProjectAdd from "./ProjectAdd";
import { ProjectListProps } from "./types";
import { AgGridReact } from "ag-grid-react";
import { useMount, useUpdateEffect } from "ahooks";
import { invoke } from "@tauri-apps/api";
import { Button } from "@radix-ui/themes";
import cn from "classnames";
import { PlayIcon, PauseIcon } from "@radix-ui/react-icons";
import MoreMenu from "./MoreMenu";
import { useTranslation } from "react-i18next";

function ProjectList(props: ProjectListProps) {
    const {} = props;
    const [projectList, updateProjectList] = useState<any[]>([]);
    const [runs, updateRuns] = useState<string[]>([]);
    const { t } = useTranslation();
    const timeRef = useRef<number>(0);

    useMount(() => {
        invoke("get_project_list").then((res: any) => {
            if (res) {
                updateProjectList(res);
            }
        });
    });

    const startProject = useCallback((projectName: string) => {
        invoke("run_project", { projectName }).then((_res) => {
            runs.push(projectName);
            updateRuns([...runs]);
        });
    }, []);

    const stopProject = useCallback((projectName: string) => {
        invoke("stop_project", { projectName }).then((_res) => {
            updateRuns(runs.filter((item) => item != projectName));
        });
    }, []);

    useUpdateEffect(() => {
        const handle = (runs: string[]) => {
            if (runs.length === 0) {
                return;
            }
            timeRef.current = setTimeout(() => {
                runs.forEach((projectName: string) => {
                    invoke("get_process_info", { projectName }).then(
                        (res: any) => {
                            if (res === null) return;
                            updateProjectList((prev) => {
                                let row = prev.find(
                                    (item) => item.name === projectName
                                );
                                row.memory = (res.memory / 1024 / 1024).toFixed(
                                    2
                                );
                                row.cpu_usage = (res.cpu_usage * 100).toFixed(
                                    2
                                );
                                return Array.from(prev);
                            });
                        }
                    );
                });
                handle(runs);
            }, 1000);
        };
        console.log(runs);
        handle(runs);
        return () => {
            clearTimeout(timeRef.current);
        };
    }, [runs]);

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
                        headerName: "使用情况",
                        editable: false,
                        width: 200,
                        cellRenderer: (props: any) => {
                            return (
                                <>
                                    <span>memory:{props.data.memory}MB</span>,
                                    <span>
                                        cpu_usage:{props.data.cpu_usage}%
                                    </span>
                                </>
                            );
                        },
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
