import ProjectAdd from "./ProjectAdd";
import { ProjectListProps } from "./types";
import { AgGridReact } from "ag-grid-react";

function ProjectList(props: ProjectListProps) {
    const {} = props;
    return (
        <>
            <ProjectAdd />
            <AgGridReact />
        </>
    );
}

export default ProjectList;
