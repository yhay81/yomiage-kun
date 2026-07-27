import sitemap from "@astrojs/sitemap";
import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://yomiage.yusuke-hayashi.com",
  output: "static",
  trailingSlash: "always",
  compressHTML: true,
  integrations: [sitemap()],
  markdown: {
    syntaxHighlight: false,
  },
  security: {
    csp: {
      directives: [
        "default-src 'self'",
        "base-uri 'self'",
        "connect-src 'self'",
        "font-src 'self'",
        "form-action 'self'",
        "frame-ancestors 'none'",
        "img-src 'self' data:",
        "object-src 'none'",
        "upgrade-insecure-requests",
      ],
    },
  },
  build: {
    format: "directory",
  },
});
