{% extends "base.tpl" %}

{% block title %}
  <title>{{ post.title }}</title>
{% endblock title %}

{%- block css %}
  <style>
    .katex      { font-size: 1em !important; }
  </style>
{% endblock css -%}

{% block main %}
    <h1>{{ post.title }}</h1>
    <article>
      {{ post.content }}
      <ul id="article_footer">
      {%- if post_tags %}
        <li>tags: {% for tag in post_tags %}<a href="{{ config.site_url }}{{ tag.url }}">{{ tag.name }}<sup>{{ tag.num }}</sup></a>{% endfor %}</li>
      {% endif -%}
        <li>date: {{ post.headers.created | date(format="%Y-%m-%d %H:%M:%S") }}</li>
      </ul>
    </article>
{%- endblock main %}

{% block js %}
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.9.0/dist/katex.min.css">
<script src="https://cdn.jsdelivr.net/npm/katex@0.9.0/dist/katex.min.js"></script>
<script>
  "use strict";
  document.addEventListener("DOMContentLoaded", function () {
      var maths = document.getElementsByClassName("language-math");
      for (var i=0; i<maths.length; i++) {
          var el = maths[i];
          katex.render(el.innerText, el, {displayMode: true});
      }
      var codes = document.getElementsByTagName("code");
      for (i=0; i<codes.length; i++) {
          el = codes[i];
          if (el.classList.contains("language-math")) continue;
          if (el.classList.contains("language-inline-math")) {
              katex.render(el.innerText, el);
              continue;
          }
          var parent = el.parentNode;
          if (parent.nodeName.toLowerCase() === "pre") continue;
          // TODO: Can this be done with DOM manipulation rather than string manipulation?
          // https://stackoverflow.com/q/48438067/3019990
          var inlineMath = "$" + el.outerHTML + "$";
          if (parent.innerHTML.indexOf(inlineMath) !== -1) {
              el.classList.add("language-inline-math");
              parent.innerHTML = parent.innerHTML.replace("$" + el.outerHTML + "$", el.outerHTML);
              i--;
          }
      }
  });
</script>
{% endblock js %}
