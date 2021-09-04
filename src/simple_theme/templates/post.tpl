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
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/katex.min.css">
<script src="https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/katex.min.js"></script>
<script>
  "use strict";
  document.addEventListener("DOMContentLoaded", function () {
      var maths = document.getElementsByTagName("language-math");
      for (var i=0; i<maths.length; i++) {
          var el = maths[i];
          katex.render(el.innerText, el, {displayMode: true});
      }

      var inline_maths = document.getElementsByTagName("language-inline-math");
      for (var i=0; i<inline_maths.length; i++) {
          var el = inline_maths[i];
          katex.render(el.innerText, el);
      }
  });
</script>
{% endblock js %}
