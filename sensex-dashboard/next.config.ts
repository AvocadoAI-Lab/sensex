import createNextIntlPlugin from 'next-intl/plugin';
import type { NextConfig } from "next";

const withNextIntl = createNextIntlPlugin();

const config: NextConfig = {
  // Enable static rendering
  output: 'standalone'
};

export default withNextIntl(config);
