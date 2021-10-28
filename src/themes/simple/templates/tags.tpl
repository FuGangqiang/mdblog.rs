{% extends "base.tpl" %}

{% block title %}
  <title>Tags</title>
{% endblock title %}

{%- block css %}
<style>
  article a {
    margin: 0.5rem;
    line-height: 1.5rem;
    white-space: nowrap;
  }

  @media (max-width: 767px) {
    article a {
      font-size: 1.5rem;
      line-height: 2rem;
    }

    article a sup {
      font-size: 1rem;
    }
  }
</style>
{% endblock css -%}

{% block main %}
  <h1>Tags</h1>
  <article>
  {%- for tag in tags %}
    <a href="{{ config.site_url }}/tags/{{ tag.name | urlencode }}.html">{{ tag.name }}<sup>{{ tag.num }}</sup></a>
  {%- endfor %}
  </article>
{%- endblock main %}

{% block js %}
<script src="{{ config.site_url }}/static/main.js"></script>
{% endblock js %}
