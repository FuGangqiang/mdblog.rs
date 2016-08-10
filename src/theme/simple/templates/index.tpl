{% extends "base.tpl" %}


{% block main %}
  <article>
  {% for post in posts %}
    <section>
      <span>{{ post.datetime }}</span>
      <span><a href="{{ post.url }}">{{ post.title }}</a></span>
    </section>
  {% endfor %}
  </article>
{% endblock main %}
