export type PageType = "Version" | "Project" | "ProjectFrom";

export interface PageContextProps {
    pageType: PageType;
    updatePageType?: (pageType: PageType) => void;
}
