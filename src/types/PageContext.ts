type PageParamsValue = string | number | undefined;
export type PageType = "Version" | "Project" | "ProjectFrom" | "LogSelect";
export type PageParams = Record<string, PageParamsValue>;
export interface PageContextProps {
    pageType: PageType;
    pageParams?: PageParams;
    updatePageType?: (pageType: PageType) => void;
    updatePageParams?: (pageParams: PageParams) => void;
}
