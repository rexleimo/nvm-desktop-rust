import { ProjectFromProps } from "./types";
import { TextField, Button, Select } from "@radix-ui/themes";
import { useMemoizedFn } from "ahooks";
import { open } from "@tauri-apps/api/dialog";
import cn from "classnames";
import { useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import PageContext from "../../contexts/PageContext";
import { useTranslation } from "react-i18next";

const formItemClassName =
    "flex flex-row gap-4 min-w-0 shrink-0 mt-4 items-center";
const formItemLabelClassName = "shrink-0 text-right w-[100px]";

function ProjectFrom(props: ProjectFromProps) {
    const {} = props;
    const { updatePageType, pageParams } = useContext(PageContext);
    const [versionList, updateVersionLit] = useState([]);

    const { t } = useTranslation();

    const [from, setFrom] = useState({
        id: undefined,
        name: "",
        dir: "",
        version: "",
        run_cmd: "",
    });

    const openInputFile = useMemoizedFn(async () => {
        let selectDirPath = (await open({
            directory: true,
        })) as string;

        const data = {
            ...from,
            dir: selectDirPath,
        };
        const dirName = selectDirPath.split("\\").pop();
        if (!from.name && dirName) {
            data.name = dirName;
        }
        setFrom(data);
    });

    const onSubmit = useMemoizedFn(() => {
        invoke("create_project", { body: from }).then(() => {
            updatePageType?.("Project");
        });
    });

    useEffect(() => {
        invoke("get_version_list").then((response: any) => {
            return updateVersionLit(
                response.filter((item: any) => item.status > 0)
            );
        });
    }, []);

    useEffect(() => {
        if (pageParams) {
            const paramLength = Object.keys(pageParams).length;
            if (paramLength > 0) {
                setFrom(pageParams as any);
            }
        }
    }, [pageParams]);

    return (
        <>
            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}>
                    {t("project_txt")}：
                </label>
                <TextField.Root>
                    <TextField.Input
                        value={from.name}
                        style={{ width: 240 }}
                        placeholder={t("project_txt")}
                        onChange={(e) => {
                            setFrom({ ...from, name: e.target.value });
                        }}
                    />
                </TextField.Root>
            </div>

            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}>
                    {t("node_version_txt")}：
                </label>
                <Select.Root
                    value={from.version}
                    onValueChange={(val) => {
                        setFrom({ ...from, version: val });
                    }}
                >
                    <Select.Trigger
                        placeholder={t("please_select_version_txt")}
                    />
                    <Select.Content>
                        <Select.Group>
                            {versionList.map((item: any) => {
                                return (
                                    <Select.Item
                                        key={item.name}
                                        value={item.name}
                                    >
                                        {item.name}
                                    </Select.Item>
                                );
                            })}
                        </Select.Group>
                    </Select.Content>
                </Select.Root>
            </div>

            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}>
                    {t("startup_command")}：
                </label>
                <TextField.Root>
                    <TextField.Input
                        value={from.run_cmd}
                        style={{ width: 240 }}
                        onChange={(e) => {
                            setFrom({ ...from, run_cmd: e.target.value });
                        }}
                    />
                </TextField.Root>
            </div>

            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}>
                    {t("project_txt")}：
                </label>
                <TextField.Root>
                    <Button onClick={openInputFile}>
                        {t("select_folder_txt")}
                    </Button>
                </TextField.Root>
                <span>{from.dir}</span>
            </div>

            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}></label>
                <TextField.Root>
                    <Button color='green' onClick={onSubmit}>
                        {t("submit_txt")}
                    </Button>
                </TextField.Root>
            </div>
        </>
    );
}

export default ProjectFrom;
