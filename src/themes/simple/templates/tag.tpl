{% extends "base.tpl" %}

{% block title %}
  <title>{{ tag.name }}</title>
{% endblock title %}

{%- block css %}
<style>
  .title {
    font-size: 1.5rem;
    margin-left: 1rem;
  }
</style>
{% endblock css -%}

{% block main %}
  <h1>{{ tag.name }}</h1>
  <article>
  {%- for post in posts %}
    <section>
      <span>{{ post.headers.created | date }}</span>
      <span class="title"><a href="{{ config.site_url }}{{ post.url  | urlencode }}">{{ post.title }}</a></span>
    </section>
  {%- endfor %}
  </article>

  <div id="pages">
  {%- if page.index > 1 %}
    <a class="prev" href="{{ tag_pages | get(key=tag.name) | nth(n=page.index - 2) | get(key='name') | urlencode }}">« Previous</a>
  {%- endif -%}
    <span class="spacer"></span>
    <span class="info">{{ page.index }} / {{ tag_pages | get(key=tag.name) | length }}</span>
    <span class="spacer"></span>
  {%- if page.index < tag_pages | get(key=tag.name) | length %}
    <a class="next" href="{{ tag_pages | get(key=tag.name) | nth(n=page.index) | get(key='name') | urlencode }}">Next »</a>
  {% endif -%}
  </div>
{%- endblock main %}

{% block js %}{% endblock js %}
