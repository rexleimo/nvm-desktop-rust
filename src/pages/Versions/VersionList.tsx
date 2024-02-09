import { useEffect, useState } from "react";
import { VersionListProps } from "./types";
import { invoke } from "@tauri-apps/api/tauri";

import { AgGridReact } from "ag-grid-react";

function VersionList(props: VersionListProps) {
    const {} = props;

    const [versionList, updateVersionLit] = useState([]);

    useEffect(() => {
        invoke("get_version_list").then((response: any) => {
            updateVersionLit(response);
        });
    }, []);

    return (
        <div className='ag-theme-quartz h-full w-full'>
            <AgGridReact
                domLayout='autoHeight'
                rowData={versionList.sort((a: string, b: string) =>
                    b.localeCompare(a)
                )}
                columnDefs={[
                    {
                        headerName: "版本",
                        cellRenderer: (props: any) => {
                            console.log(props);
                            return <>{props.data}</>;
                        },
                    },
                    {
                        headerName: "状态",
                        cellRenderer: (props: any) => {
                            return <span>安装</span>;
                        },
                    },
                    {
                        headerName: "操作",
                        cellRenderer: (props: any) => {
                            return <button>下载</button>;
                        },
                    },
                ]}
            />
        </div>
    );
}

export default VersionList;
