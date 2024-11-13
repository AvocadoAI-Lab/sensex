import createMiddleware from 'next-intl/middleware';
import {locales, defaultLocale} from './src/i18n/routing';

export default createMiddleware({
  // A list of all locales that are supported
  locales,
  defaultLocale,
  // Used when no locale matches
  localeDetection: true
});

export const config = {
  // Skip all paths that should not be internationalized
  matcher: [
    // Match all pathnames except for
    // - … if they start with `/api`, `/_next` or `/_vercel`
    // - … the ones containing a dot (e.g. `favicon.ico`)
    '/((?!api|_next|_vercel|.*\\..*).*)',
    // Optional: Match all pathnames within `/users`, optionally with a locale prefix
    '/([\\w-]+)?/users/(.+)'
  ]
};
