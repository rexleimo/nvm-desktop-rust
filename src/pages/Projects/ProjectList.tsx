import { useCallback, useState } from "react";
import ProjectAdd from "./ProjectAdd";
import { ProjectListProps } from "./types";
import { AgGridReact } from "ag-grid-react";
import { useMount } from "ahooks";
import { invoke } from "@tauri-apps/api";
import { Button } from "@radix-ui/themes";
import cn from "classnames";

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
                                <>
                                    <Button
                                        color='green'
                                        className='cursor-pointer'
                                        onClick={() =>
                                            startProject(props.data.name)
                                        }
                                    >
                                        运行
                                    </Button>
                                    <Button
                                        color='red'
                                        className='cursor-pointer'
                                        onClick={() =>
                                            stopProject(props.data.name)
                                        }
                                    >
                                        Stop
                                    </Button>
                                </>
                            );
                        },
                    },
                ]}
            />
        </div>
    );
}

export default ProjectList;
