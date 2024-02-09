import { useEffect, useState } from "react";
import { VersionListProps } from "./types";
import { invoke } from "@tauri-apps/api/tauri";

function VersionList(props: VersionListProps) {
    const {} = props;

    const [versionList, updateVersionLit] = useState([]);

    useEffect(() => {
        invoke("get_version_list").then((response: any) => {
            updateVersionLit(response);
        });
    }, []);

    return (
        <>
            <ul>
                {versionList
                    .sort((a: string, b: string) => b.localeCompare(a))
                    .map((item) => {
                        return <li>{item}</li>;
                    })}
            </ul>
        </>
    );
}

export default VersionList;
