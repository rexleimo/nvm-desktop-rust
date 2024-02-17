import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { MoreMenuProps } from "./types";
import cn from "classnames";
import { Button } from "@radix-ui/themes";
import React, { useCallback, useContext } from "react";
import { invoke } from "@tauri-apps/api";
import { TrashIcon, FileTextIcon, Pencil2Icon } from "@radix-ui/react-icons";
import PageContext from "../../contexts/PageContext";

const iconSize = 16;

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

const ZongDuanIcon = () => {
    return (
        <svg
            className='icon'
            viewBox='0 0 1024 1024'
            version='1.1'
            xmlns='http://www.w3.org/2000/svg'
            p-id='7091'
            width={iconSize}
            height={iconSize}
        >
            <path d='M250.8 463.6c-7.8 0-15.6-3-21.4-9-11.6-11.8-11.4-30.8 0.4-42.4l53.6-52.8-53.6-53.6c-11.8-11.8-11.8-30.8 0-42.4 11.8-11.8 30.8-11.8 42.4 0l75 75c5.6 5.6 8.8 13.4 8.8 21.4 0 8-3.2 15.6-9 21.2l-75 74c-6 5.6-13.6 8.6-21.2 8.6z m343.6 23.2c0-16.6-13.4-30-30-30h-154.2c-16.6 0-30 13.4-30 30s13.4 30 30 30h154.2c16.6 0 30-13.4 30-30zM476.2 692l5-8.6H198.8c-29 0-52.8-23.6-52.8-52.8V214.2c0-29 23.6-52.8 52.8-52.8h566.8c29 0 52.8 23.6 52.8 52.8v280.8c23 0.2 44.6 10 60 26.4V214.2c0-62.2-50.6-112.8-112.8-112.8H198.8C136.6 101.4 86 152 86 214.2v416.6c0 62.2 50.6 112.8 112.8 112.8h267c-2.2-17.6 1.4-35.8 10.4-51.6z m312.2-94.4h-121.2c-11 0-21.2 5.8-26.8 15.4L579.8 718c-5.6 9.6-5.6 21.4 0 31l60.6 105c5.6 9.6 15.8 15.4 26.8 15.4h121.2c11 0 21.2-5.8 26.8-15.4l60.6-105c5.6-9.6 5.6-21.4 0-31l-60.6-105c-5.4-9.6-15.8-15.4-26.8-15.4m0-60c32.4 0 62.6 17.4 78.8 45.4l60.6 105c16.2 28 16.2 62.8 0 91L867.2 884c-16.2 28-46.4 45.4-78.8 45.4h-121.2c-32.4 0-62.6-17.4-78.8-45.4l-60.6-105c-16.2-28-16.2-62.8 0-91l60.6-105c16.2-28 46.4-45.4 78.8-45.4h121.2z m22 195.8c0-45.6-37-82.6-82.6-82.6s-82.6 37-82.6 82.6 37 82.6 82.6 82.6 82.6-37 82.6-82.6z m-60 0c0 12.4-10.2 22.6-22.6 22.6s-22.6-10.2-22.6-22.6 10.2-22.6 22.6-22.6 22.6 10.2 22.6 22.6z'></path>
        </svg>
    );
};

function MoreMenuItem(props: DropdownMenu.DropdownMenuItemProps) {
    const classNames = cn(
        "group text-violet11 rounded-[3px] flex items-center py-[10px] relative px-[10px] select-none outline-none",
        "cursor-pointer",
        "hover:bg-slate-400",
        props.className
    );
    return <DropdownMenu.Item {...props} className={classNames} />;
}

const MoreMenuItemMemo = React.memo(MoreMenuItem);

function MoreMenu(props: MoreMenuProps) {
    const { updateProjectList, projectName, projectId } = props;
    const { updatePageType, updatePageParams } = useContext(PageContext);

    const deleteProject = useCallback(() => {
        invoke("delete_project", { id: projectId }).then((res: any) => {
            updateProjectList(res);
        });
    }, []);

    const openProject = useCallback(() => {
        invoke("open_project", { projectName }).then((_res: any) => {});
    }, []);

    const openLog = useCallback(() => {
        invoke("open_log", { projectName }).then((_res: any) => {});
    }, []);

    const openCmd = useCallback(() => {
        invoke("open_cmd", { projectName }).then((_res: any) => {});
    }, []);

    const editProject = useCallback(() => {
        invoke("get_project_info", { projectName }).then((res: any) => {
            updatePageType?.("ProjectFrom");
            updatePageParams?.(res);
        });
    }, []);

    return (
        <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild>
                <Button
                    variant='outline'
                    highContrast
                    className={cn("cursor-pointer")}
                >
                    更多
                </Button>
            </DropdownMenu.Trigger>

            <DropdownMenu.Portal>
                <DropdownMenu.Content className='min-w-[40px] bg-white rounded-md p-[5px] shadow-[0px_10px_38px_-10px_rgba(22,_23,_24,_0.35),_0px_10px_20px_-15px_rgba(22,_23,_24,_0.2)] will-change-[opacity,transform] data-[side=top]:animate-slideDownAndFade data-[side=right]:animate-slideLeftAndFade data-[side=bottom]:animate-slideUpAndFade data-[side=left]:animate-slideRightAndFade'>
                    <MoreMenuItemMemo onClick={editProject}>
                        <Pencil2Icon width={iconSize} height={iconSize} />
                    </MoreMenuItemMemo>
                    <MoreMenuItemMemo onClick={deleteProject}>
                        <TrashIcon width={iconSize} height={iconSize} />
                    </MoreMenuItemMemo>
                    <MoreMenuItemMemo onClick={openProject}>
                        <VscodeIcon />
                    </MoreMenuItemMemo>
                    <MoreMenuItemMemo onClick={openLog}>
                        <FileTextIcon />
                    </MoreMenuItemMemo>
                    <MoreMenuItemMemo onClick={openCmd}>
                        <ZongDuanIcon />
                    </MoreMenuItemMemo>
                </DropdownMenu.Content>
            </DropdownMenu.Portal>
        </DropdownMenu.Root>
    );
}

MoreMenu.Item = MoreMenuItem;

export default MoreMenu;
