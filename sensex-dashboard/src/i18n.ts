import {getRequestConfig} from 'next-intl/server';
import {locales, type Locale} from './i18n/routing';

// This is the default configuration for next-intl,
// see: https://next-intl-docs.vercel.app/docs/getting-started/app-router
export default getRequestConfig(async ({locale}) => {
  // Validate that the incoming `locale` parameter is valid
  if (!locales.includes(locale as Locale)) {
    // Return default locale messages if locale is invalid
    return {
      messages: (await import(`../messages/en.json`)).default,
      timeZone: 'Asia/Taipei'
    };
  }
 
  return {
    messages: (await import(`../messages/${locale}.json`)).default,
    timeZone: 'Asia/Taipei'
  };
});
