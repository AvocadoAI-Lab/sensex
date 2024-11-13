import {Pathnames} from 'next-intl/navigation';

export const locales = ['en', 'zh-TW'] as const;
export type Locale = typeof locales[number];

export const pathnames = {
  '/': '/',
  '/dashboard': '/dashboard'
} satisfies Pathnames<typeof locales>;

export const localePrefix = 'always';

export type PathnameLocale = typeof locales[number];

export function getLocale(pathname: string): PathnameLocale | undefined {
  const segments = pathname.split('/');
  const localeCandidate = segments[1];
  return locales.includes(localeCandidate as PathnameLocale)
    ? (localeCandidate as PathnameLocale)
    : undefined;
}

export const defaultLocale: PathnameLocale = 'en';
