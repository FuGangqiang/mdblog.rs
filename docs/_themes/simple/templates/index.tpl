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
        <span>{{ post.headers.created | truncate(length=10, end="") }}</span>
        <a class="title" href="{{ config.site_url }}{{ post.url  | urlencode }}">{{ post.title }}</a>
      </section>
    {%- endfor %}
  </article>

  <div id="pages">
  {%- if page.index > 1 %}
    <a class="prev" href="{{ index_pages | nth(n=page.index - 2) | get(key='name') | urlencode }}">« Previous</a>
  {%- endif -%}
    <span class="spacer"></span>
    <span class="info">{{ page.index }} / {{ index_pages | length }}</span>
    <span class="spacer"></span>
  {%- if page.index < index_pages | length %}
    <a class="next" href="{{ index_pages | nth(n=page.index) | get(key='name') | urlencode }}">Next »</a>
  {% endif -%}
  </div>
{%- endblock main %}

{% block js %}{% endblock js %}
