{% extends "base.tpl" %}

{% block title %}
  <title>{{ config.site_name }}</title>
{% endblock title %}

{% block css %}
<style>
  .title {
    font-size: 1.5rem;
    margin-left: 1rem;
  }
</style>
{% endblock css %}

{% block main %}
  <article>
    {%- for post in posts %}
      <section>
        <span>{{ post.headers.created | date(format="%Y-%m-%d") }}</span>
        <a class="title" href="{{ config.site_url }}{{ post.url  | urlencode }}">{{ post.title }}</a>
      </section>
    {%- endfor %}
  </article>

  <div id="pages">
  {%- if page.index > 1 %}
    <span class="prev"><a href="{{ index_pages | nth(n=page.index - 2) | get(key='name') | urlencode }}">« Previous</a></span>
  {%- endif -%}
    <span class="info">{{ page.index }} / {{ index_pages | length }}</span>
  {%- if page.index < index_pages | length %}
    <span class="next"><a href="{{ index_pages | nth(n=page.index) | get(key='name') | urlencode }}">Next »</a></span>
  {% endif -%}
  </div>
{%- endblock main %}

{% block js %}
<script src="{{ config.site_url }}/static/main.js"></script>
{% endblock js %}
