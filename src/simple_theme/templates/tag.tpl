{% extends "base.tpl" %}

{% block title %}
<title>{{ tag.name }}</title>
{% endblock title %}

{% block main %}
  <h1>{{ tag.name }}</h1>
  <article>
  {% for post in tag.posts %}
    <section>
      <span>{{ post.headers.created | date }}</span>
      <span><a href="{{ config.url_prefix }}{{ post.url  | urlencode }}">{{ post.title }}</a></span>
    </section>
  {% endfor %}
  </article>
{% endblock main %}


{% block js %}
<script src="{{ config.url_prefix }}/static/main.js"></script>
{% endblock js %}
