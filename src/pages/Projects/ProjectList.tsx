import { useCallback, useState } from "react";
import ProjectAdd from "./ProjectAdd";
import { ProjectListProps } from "./types";
import { AgGridReact } from "ag-grid-react";
import { useMount } from "ahooks";
import { invoke } from "@tauri-apps/api";
import { Button } from "@radix-ui/themes";
import cn from "classnames";
import { PlayIcon, PauseIcon, TrashIcon } from "@radix-ui/react-icons";

const iconSize = 24;

const VscodeIcon = () => {
    return (
        <svg
            className='icon'
            viewBox='0 0 1024 1024'
            version='1.1'
            xmlns='http://www.w3.org/2000/svg'
            width={iconSize}
            height={iconSize}
        >
            <path
                d='M746.222933 102.239573l-359.799466 330.820267L185.347413 281.4976 102.2464 329.864533l198.20544 182.132054-198.20544 182.132053 83.101013 48.510293 201.076054-151.558826 359.799466 330.676906 175.527254-85.251413V187.4944z m0 217.57952v384.341334l-255.040853-192.177494z'
                fill='#2196F3'
            ></path>
        </svg>
    );
};

function ProjectList(props: ProjectListProps) {
    const {} = props;
    const [projectList, updateProjectList] = useState<any[]>([]);

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

    const deleteProject = useCallback((projectName: string) => {
        invoke("delete_project", { projectName }).then((res: any) => {
            updateProjectList(res);
        });
    }, []);

    const openProject = useCallback((projectName: string) => {
        invoke("open_project", { projectName }).then((_res: any) => {});
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
                        headerName: "项目名称",
                        editable: false,
                        width: 200,
                    },
                    {
                        field: "dir",
                        headerName: "项目路径",
                        editable: false,
                        width: 400,
                    },
                    {
                        headerName: "操作",
                        cellClass: cn("flex items-center"),
                        cellRenderer: (props: any) => {
                            return (
                                <div className={cn("flex", "gap-2")}>
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
                                    <Button
                                        color='crimson'
                                        className='cursor-pointer'
                                        onClick={() =>
                                            deleteProject(props.data.name)
                                        }
                                    >
                                        <TrashIcon />
                                    </Button>
                                    <Button
                                        color='iris'
                                        className='cursor-pointer'
                                        onClick={() =>
                                            openProject(props.data.name)
                                        }
                                    >
                                        <VscodeIcon />
                                    </Button>
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
