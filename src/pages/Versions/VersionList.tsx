import { useCallback, useEffect, useState } from "react";
import { VersionListProps } from "./types";
import { invoke } from "@tauri-apps/api/tauri";
import { AgGridReact } from "ag-grid-react";
import { Badge, Button, TextField } from "@radix-ui/themes";
import cn from "classnames";

function VersionList(props: VersionListProps) {
    const {} = props;

    const [versionList, updateVersionLit] = useState([]);
    const [versionTxt, updateVersionTxt] = useState<string>("");

    useEffect(() => {
        invoke("get_version_list").then((response: any) => {
            updateVersionLit(response);
        });
    }, []);

    const handleDownloadNode = useCallback((version: string) => {
        invoke("download_node", {
            versionStr: version,
        }).then((res: any) => {
            updateVersionLit(res);
        });
    }, []);

    const handleInstallNode = useCallback((version: string) => {
        invoke("unzip_version", {
            versionStr: version,
        }).then((res: any) => {
            updateVersionLit(res);
        });
    }, []);

    const handleUseVersion = useCallback((version: string) => {
        invoke("use_version", {
            versionStr: version,
        }).then((res: any) => {
            updateVersionLit(res);
        });
    }, []);

    const handleDownloadRemote = useCallback(() => {
        invoke("download_remote", {
            versionStr: versionTxt,
        }).then((res: any) => {
            if (res) {
                updateVersionLit(res);
            }
        });
    }, [versionTxt]);

    return (
        <div className='ag-theme-quartz h-full w-full'>
            <TextField.Root className={cn("flex", "items-center", cn("mb-4"))}>
                <TextField.Input
                    onChange={(e) => updateVersionTxt(e.target.value)}
                />
                <Button color='green' onClick={handleDownloadRemote}>
                    确认
                </Button>
            </TextField.Root>
            <AgGridReact
                domLayout='autoHeight'
                rowData={versionList}
                columnDefs={[
                    {
                        headerName: "版本",
                        cellRenderer: (props: any) => {
                            return <>{props.data.name}</>;
                        },
                        resizable: false,
                        width: 375,
                    },
                    {
                        headerName: "状态",
                        cellRenderer: (props: any) => {
                            if (0 === props.data.status) {
                                return <Badge color='blue'>未下载</Badge>;
                            } else {
                                return <Badge color='green'>以下载</Badge>;
                            }
                        },
                        resizable: false,
                    },
                    {
                        headerName: "操作",
                        cellClass: cn("flex items-center"),
                        cellRenderer: (props: any) => {
                            if (0 === props.data.status) {
                                return (
                                    <Button
                                        disabled={props.data.isUse}
                                        color='gray'
                                        onClick={() =>
                                            handleDownloadNode(props.data.name)
                                        }
                                    >
                                        下载
                                    </Button>
                                );
                            }
                            if (1 === props.data.status) {
                                return (
                                    <Button
                                        color='green'
                                        onClick={() =>
                                            handleInstallNode(props.data.name)
                                        }
                                    >
                                        安装
                                    </Button>
                                );
                            }
                            if (2 === props.data.status) {
                                return (
                                    <Button
                                        disabled={props.data.is_use}
                                        color='blue'
                                        onClick={() =>
                                            handleUseVersion(props.data.name)
                                        }
                                    >
                                        使用
                                    </Button>
                                );
                            }
                        },
                        resizable: false,
                    },
                ]}
            />
        </div>
    );
}

export default VersionList;
