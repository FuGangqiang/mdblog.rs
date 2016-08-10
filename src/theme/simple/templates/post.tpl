{% extends "base.tpl" %}


{% block main %}
    <h1>{{ title }}</h1>
    <article>
      {{ content }}

      <ul id="article_footer">
        <li>tags: {% for tag in post_tags %}<a href="{{ tag.url }}">{{ tag.name }}<sup>{{ tag.num }}</sup></a>{% endfor %}</li>
        <li>date: {{ datetime }}</li>
      </ul>
    </article>
{% endblock main %}
