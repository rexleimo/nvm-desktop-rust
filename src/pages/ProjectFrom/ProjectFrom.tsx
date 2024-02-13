import { ProjectFromProps } from "./types";
import { TextField, Button, Select } from "@radix-ui/themes";
import { useMemoizedFn } from "ahooks";
import { open } from "@tauri-apps/api/dialog";
import cn from "classnames";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";

const formItemClassName =
    "flex flex-row gap-4 min-w-0 shrink-0 mt-4 items-center";
const formItemLabelClassName = "shrink-0 text-right w-20";

function ProjectFrom(props: ProjectFromProps) {
    const {} = props;

    const [versionList, updateVersionLit] = useState([]);
    const [from, setFrom] = useState({
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
        invoke("create_project", { body: from }).then((res) => {});
    });

    useEffect(() => {
        invoke("get_version_list").then((response: any) => {
            return updateVersionLit(
                response.filter((item: any) => item.status > 0)
            );
        });
    }, []);

    return (
        <>
            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}>项目名：</label>
                <TextField.Root>
                    <TextField.Input
                        value={from.name}
                        style={{ width: 240 }}
                        placeholder='项目'
                        onChange={(e) => {
                            setFrom({ ...from, name: e.target.value });
                        }}
                    />
                </TextField.Root>
            </div>

            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}>Node版本：</label>
                <Select.Root
                    value={from.version}
                    onValueChange={(val) => {
                        setFrom({ ...from, version: val });
                    }}
                >
                    <Select.Trigger placeholder='请选择版本' />
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
                <label className={cn(formItemLabelClassName)}>启动命令：</label>
                <TextField.Root>
                    <TextField.Input
                        style={{ width: 240 }}
                        onChange={(e) => {
                            setFrom({ ...from, run_cmd: e.target.value });
                        }}
                    />
                </TextField.Root>
            </div>

            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}>项目路径：</label>
                <TextField.Root>
                    <Button onClick={openInputFile}>选择文件夹</Button>
                </TextField.Root>
                <span>{from.dir}</span>
            </div>

            <div className={cn(formItemClassName)}>
                <label className={cn(formItemLabelClassName)}></label>
                <TextField.Root>
                    <Button color='green' onClick={onSubmit}>
                        确认
                    </Button>
                </TextField.Root>
            </div>
        </>
    );
}

export default ProjectFrom;
