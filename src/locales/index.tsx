import i18n from "i18next";
import { initReactI18next, I18nextProvider as Provider } from "react-i18next";
import en from "./en";
import zh from "./zh";

i18n.use(initReactI18next).init({
    resources: {
        en: {
            translation: en,
        },
        zh: {
            translation: zh,
        },
    },
    lng: "en",
    fallbackLng: "en",
    interpolation: {
        escapeValue: false, // not needed for react as it escapes by default
    },
});

function I18nextProvider(props: { children: React.ReactNode }) {
    return <Provider i18n={i18n} {...props} />;
}

export default I18nextProvider;
export { i18n };
