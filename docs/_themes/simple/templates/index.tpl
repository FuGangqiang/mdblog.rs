{% extends "base.tpl" %}

{% block title %}
  <title>{{ config.site_name }}</title>
{% endblock title %}

{% block css %}{% endblock css %}

{% block main %}
  <article>
  {%- for post in posts %}
    <section>
      <span>{{ post.headers.created | date }}</span>
      <span><a href="{{ config.site_url }}{{ post.url  | urlencode }}">{{ post.title }}</a></span>
    </section>
  {%- endfor %}
  </article>
  <div id="pages">
  {%- if page.index > 1 %}
    <span class="prev"><a href="{{ index_pages | nth(n=page.index - 2) | get(key='name') | urlencode }}">« Previous</a></span>
  {%- endif -%}
  {%- if page.index < index_pages | length %}
    <span class="next"><a href="{{ index_pages | nth(n=page.index) | get(key='name') | urlencode }}">Next »</a></span>
  {% endif -%}
  </div>
{%- endblock main %}

{% block js %}
<script src="{{ config.site_url }}/static/main.js"></script>
{% endblock js %}
