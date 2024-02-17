export interface ProjectListProps {}

export interface MoreMenuProps {
    projectId: number;
    projectName: string;
    updateProjectList: React.Dispatch<React.SetStateAction<any[]>>;
}
