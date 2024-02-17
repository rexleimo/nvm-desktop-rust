import { useCallback, useEffect, useState } from "react";
import { VersionListProps } from "./types";
import { invoke } from "@tauri-apps/api/tauri";
import { AgGridReact } from "ag-grid-react";
import { Badge, Button, Link, TextField } from "@radix-ui/themes";
import cn from "classnames";
import { useTranslation } from "react-i18next";

function VersionList(props: VersionListProps) {
    const {} = props;

    const [versionList, updateVersionLit] = useState([]);
    const [versionTxt, updateVersionTxt] = useState<string>("");
    const { t } = useTranslation();

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
            <TextField.Root className={cn("flex", "items-center")}>
                <TextField.Input
                    onChange={(e) => updateVersionTxt(e.target.value)}
                    placeholder={`${t("download_input_placeholder")}: 21.4.0`}
                />
                <Button color='green' onClick={handleDownloadRemote}>
                    {t("submit_text")}
                </Button>
            </TextField.Root>

            <Link
                href='https://nodejs.org/dist'
                className={cn("mb-4", "inline-flex", "my-3")}
                target='_blank'
            >
                {t("for_more_version_see")}: https://nodejs.org/dist
            </Link>

            <AgGridReact
                domLayout='autoHeight'
                rowData={versionList}
                columnDefs={[
                    {
                        headerName: t("version_txt"),
                        cellRenderer: (props: any) => {
                            return <>{props.data.name}</>;
                        },
                        resizable: false,
                        width: 375,
                    },
                    {
                        headerName: t("status_txt"),
                        cellRenderer: (props: any) => {
                            if (0 === props.data.status) {
                                return (
                                    <Badge color='blue'>
                                        {t("not_download_txt")}
                                    </Badge>
                                );
                            } else {
                                return (
                                    <Badge color='green'>
                                        {t("is_download_txt")}
                                    </Badge>
                                );
                            }
                        },
                        resizable: false,
                    },
                    {
                        headerName: t("options_txt"),
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
                                        {t("download_txt")}
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
                                        {t("install_txt")}
                                    </Button>
                                );
                            }
                            if (2 === props.data.status) {
                                return (
                                    <Button
                                        disabled={props.data.is_use === 1}
                                        color='blue'
                                        onClick={() =>
                                            handleUseVersion(props.data.name)
                                        }
                                    >
                                        {t("use_txt")}
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
