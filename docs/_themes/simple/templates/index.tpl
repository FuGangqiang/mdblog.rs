{% extends "base.tpl" %}


{% block main %}
  <article>
  {% for post in posts %}
    <section>
      <span>{{ post.headers.created | date }}</span>
      <span><a href="{{ url_prefix }}{{ post.url  | urlencode }}">{{ post.title }}</a></span>
    </section>
  {% endfor %}
  </article>
{% endblock main %}


{% block js %}
<script src="{{ url_prefix }}/static/main.js"></script>
{% endblock js %}
