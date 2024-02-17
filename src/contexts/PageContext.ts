import React from "react";
import { PageContextProps } from "../types";

const PageContext = React.createContext<PageContextProps>({
    pageType: "Version",
    locale: "en",
});

const PageContextProvider = PageContext.Provider;
const PageContextConsumer = PageContext.Consumer;

export default PageContext;

export { PageContextProvider, PageContextConsumer };
