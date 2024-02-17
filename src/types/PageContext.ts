type PageParamsValue = string | number | undefined;
export type PageType = "Version" | "Project" | "ProjectFrom" | "LogSelect";
export type PageParams = Record<string, PageParamsValue>;
export interface PageContextProps {
    pageType: PageType;
    locale: string;
    pageParams?: PageParams;
    updatePageType?: (pageType: PageType) => void;
    updatePageParams?: (pageParams: PageParams) => void;
    updateLocale?: (locale: string) => void;
}
