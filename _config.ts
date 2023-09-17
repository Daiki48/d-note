import lume from "lume/mod.ts";
import jsx from "lume/plugins/jsx.ts";
import sass from "lume/plugins/sass.ts";

import metas from "lume/plugins/metas.ts";
import date from "lume/plugins/date.ts";

import codeHighlight from "lume/plugins/code_highlight.ts";
import lang_javascript from "npm:highlight.js/lib/languages/javascript";
import lang_bash from "npm:highlight.js/lib/languages/bash";

const site = lume({
  src: "./src",
});

site
  .use(jsx())
  .copy("assets/img")
  .use(sass())
  .use(date())
  .use(
    codeHighlight({
      languages: {
        javascript: lang_javascript,
        bash: lang_bash,
      },
    })
  )
  .use(metas());

export default site;
