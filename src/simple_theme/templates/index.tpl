{% extends "base.tpl" %}

{% block title %}
<title>{{ config.site_name }}</title>
{% endblock title %}

{% block main %}
  <article>
  {% for post in posts %}
    <section>
      <span>{{ post.headers.created | date }}</span>
      <span><a href="{{ config.url_prefix }}{{ post.url  | urlencode }}">{{ post.title }}</a></span>
    </section>
  {% endfor %}
  </article>

  <div id="pages">
    {% if prev_name %}<span class="prev"><a href="{{ prev_name | urlencode }}">« Previous</a></span>{% endif %}
    {% if next_name %}<span class="next"><a href="{{ next_name | urlencode }}">Next »</a></span>{% endif %}
  </div>
{% endblock main %}


{% block js %}
<script src="{{ config.url_prefix }}/static/main.js"></script>
{% endblock js %}
