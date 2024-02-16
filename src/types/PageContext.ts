export type PageType = "Version" | "Project" | "ProjectFrom" | "LogSelect";

export interface PageContextProps {
    pageType: PageType;
    updatePageType?: (pageType: PageType) => void;
}
